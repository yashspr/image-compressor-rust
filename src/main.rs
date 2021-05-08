mod compress;

use std::path::{Path, PathBuf};
use clap::{App, Arg};
use compress::{Compressor};
use std::fs::create_dir_all;

fn main() {
    let matches = App::new("Image Compressor")
        .version("0.1")
        .author("Yashas Reddy <yashspr5@gmail.com>")
        .about("Simple image compressor for png and jpeg files")
        .arg(
            Arg::new("directory")
                .short('d')
                .long("dir")
                .about("Indicates if you are passing a directory of images")
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("out")
                .takes_value(true)
                .about("Output directory where compressed images are to be saved")
        )
        .arg(
            Arg::new("suffix")
                .short('s')
                .long("suf")
                .takes_value(true)
                .about("The suffix to be added to the compressed image filename")
        )
        .arg(
            Arg::new("input")
                .required(true)
        ).get_matches();

    if !matches.is_present("input") {
        panic!("Please specifiy the filepath");
    }

    let filename = matches.value_of("input").unwrap();
    let file_path = Path::new(filename);
    let def_output_file_path_buf = get_dest_path(file_path).unwrap();

    let is_directory = matches.is_present("directory");
    let output_directory = matches.value_of("output").unwrap_or(def_output_file_path_buf.to_str().unwrap());
    let suffix = matches.value_of("suffix").unwrap_or("compressed");

    let is_path_valid = validate_input_path(file_path, is_directory);
    validate_output_dir(Path::new(output_directory));

    if !is_path_valid {
        panic!("Please provide a valid path");
    }

    // initate compressor here and call compress function
    let compressor = Compressor::new(filename, output_directory, is_directory, suffix);
    compressor.compress();

    println!("Optimized Image Successfully");
}

fn validate_output_dir(output_path: &Path) {
    if !output_path.is_dir() {
        create_dir_all(output_path).unwrap();
    }
}

fn validate_input_path(input_path: &Path, is_directory: bool) -> bool {
    if is_directory && input_path.is_dir() {
        true
    } else if !is_directory && input_path.is_file() {
        true
    } else {
        false
    }
}

fn get_dest_path(input_path: &Path) -> Option<PathBuf> {
    let mut input_path_buf = input_path.to_path_buf();

    if input_path.is_dir() {
        Some(input_path_buf)
    } else if input_path.is_file() {
        input_path_buf.pop();
        Some(input_path_buf)
    } else {
        None
    }
}
