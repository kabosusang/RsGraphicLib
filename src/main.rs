use std::fs;
use std::io;

use graphiclib::image::format::ImageFormat;
use graphiclib::*;

fn read_binrary_file(path: &str) -> io::Result<Vec<u8>> {
    let data = fs::read(path)?;
    Ok(data)
}

fn main() {
    let bin_vec = read_binrary_file("fnm.png");

    match bin_vec {
        Ok(data) => {
            // let first: u8 = data[0];
            // println!("First Char: {:X}", first);
            let format = ImageFormat::detect_format(&data);
            if let Some(extension) = format.extension() {
                println!("extension: {}", extension);
            }
        }
        Err(e) => {
            eprintln!("No Found File Path: {0}", e);
        }
    }
    if let Ok(config) = glenv::EnvConfig::<glenv::EnvShell>::build() {
        println!("{:#?}", config);
    }
}
