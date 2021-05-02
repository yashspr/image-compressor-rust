/* 
use std::env;
use std::path::Path;
use std::fs::File;
use mtpng::{Header,ColorType};
use mtpng::encoder::{Encoder, Options};

extern crate lodepng;
extern crate rgb;
extern crate mtpng;

use rgb::ComponentBytes;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    
    if args.len() <= 1 {
        panic!("Please pass a path to image");
    }

    let filename = &args[1];
    let file_path = Path::new(filename).to_str().unwrap();
    let file_name = Path::new(filename).file_name().unwrap().to_str().unwrap();

    println!("The path you entered: {}", file_path);

    let image = lodepng::decode32_file(file_path).unwrap();

    let image_height = image.height;
    let image_width = image.width;

    let image_buffer = image.buffer;
    let buffer_bytes = image_buffer.as_bytes();

    // create new file for saving png
    let final_file_path = format!("C:/Users/Yashas/Documents/{}-compressed.png", file_name);
    let compressed_file = File::create(Path::new(&final_file_path)).unwrap();

    // now lets initialize whats needed for mtpng
    let mut header = Header::new();
    header.set_size(image_width as u32, image_height as u32).unwrap();
    header.set_color(ColorType::TruecolorAlpha, 8).unwrap();

    let mut options = Options::new();
    options.set_compression_level(mtpng::CompressionLevel::High).unwrap();

    let mut encoder = Encoder::new(compressed_file, &options);

    encoder.write_header(&header).unwrap();
    encoder.write_image_rows(buffer_bytes).unwrap();
    encoder.finish().unwrap();

    println!("file has been saved");
}


*/