use array2::Array2;
use csc411_image::{RgbImage, Read, Rgb};
use std::env;
use std::convert::TryInto;

fn main() {
    let ppm1 = env::args().nth(1);
    let ppm2 = env::args().nth(2);

    // assert only two arguments are given in the command line
    assert!(
        env::args().len() <= 3,
        "Too many arguments!"
    );

    // Set rgb image with th filename
    let img1 = RgbImage::read(ppm1.as_deref()).unwrap();
    let img2 = RgbImage::read(ppm2.as_deref()).unwrap();

    // Get the width and height from the image
    let height1 = img1.height.try_into().unwrap();
    let width1 = img1.width.try_into().unwrap();
    let height2 = img2.height.try_into().unwrap();
    let width2 = img2.width.try_into().unwrap();

    // Create a vec of the pixels
    let pixels_vec1: Vec<Rgb> = img1.pixels.clone();
    let pixels_vec2: Vec<Rgb> = img2.pixels.clone();

    // Create Array2s 
    let img1 = Array2::from_row_major(width1, height1, pixels_vec1).unwrap();
    let img2 = Array2::from_row_major(width2, height2, pixels_vec2).unwrap();

     let mut e = 0.0;
    for (x, y, pixel1) in img1.iter_row_major() {
        let pixel2 = img2.get(x, y);

        let r1 = pixel1.red as f64;
        let g1 = pixel1.green as f64;
        let b1 = pixel1.blue as f64;

        let r2 = pixel2.red as f64;
        let g2 = pixel2.green as f64;
        let b2 = pixel2.blue as f64;

        e += (r1 - r2).powi(2) + (g1 - g2).powi(2) + (b1 - b2).powi(2);
    }
    e = e.sqrt();
    e = e / (3.0 * width1 as f64 * height1 as f64);
    println!("{:.4}", e);
}
