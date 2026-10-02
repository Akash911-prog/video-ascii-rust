// use std::collections::HashSet;

use opencv::{
    core::{AlgorithmHint::ALGO_HINT_DEFAULT, Size},
    imgproc::{COLOR_BGR2GRAY, InterpolationFlags::INTER_AREA, cvt_color, resize},
    prelude::*,
};

use crate::{config::Config, p2c_mapper::P2CMapper};

pub fn process_frame(frame: &Mat, w: i32, h: i32) -> Result<Mat, Box<dyn std::error::Error>> {
    let target = Size::new(w, h);
    let mut resized = Mat::default();
    resize(frame, &mut resized, target, 0.0, 0.0, INTER_AREA.into())?;

    let mut grayscaled = Mat::default();
    cvt_color(
        &resized,
        &mut grayscaled,
        COLOR_BGR2GRAY,
        0,
        ALGO_HINT_DEFAULT,
    )?;

    Ok(grayscaled)
}

pub fn frame_to_ascii(
    frame: &Mat,
    mapper: &P2CMapper,
    config: &Config
) -> Result<String, Box<dyn std::error::Error>> {
    let mut ascii = Vec::new();
    let mut max = 0;
    for y in 0..frame.rows() {
        for x in 0..frame.cols() {
            let pixel = frame.at_2d::<u8>(y, x)?;
            if *pixel > max {
                max = *pixel;
            }
            ascii.push(mapper.map(pixel, config.flip));
        }
        ascii.push(b'\n');
    }
    // println!("Time elapsed: {:?}", start_time.elapsed());
    // println!("Max pixel value: {}", max);

    match String::from_utf8(ascii) {
        Ok(string) => {
            Ok(string)
        }
        Err(e) => {
            println!("Error: {}", e);
            Err(Box::new(e))
        }
    }
}
