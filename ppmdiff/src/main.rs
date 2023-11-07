use array2::Array2;
use csc411_image::{RgbImage, Read, Rgb};
use std::env;
use std::io;
use std::convert::TryInto;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    // assert only two arguments are given in the command line
    assert!(
        env::args().len() == 3,
        "Too many arguments!"
    );

    let (img1, img2) = match (args[1].as_str(), args[2].as_str()) {
        ("-", "-") => {
            std::process::exit(1);
        },
        ("-", file) => (),
        (file, "-") => (),
        (file1, file2) => (),
    };


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


}
