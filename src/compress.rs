use std::path::{ Path, PathBuf };
use std::str::FromStr;
use std::fs::{self};

pub struct Compressor<'a> {
    input_path: &'a str,
    output_path: &'a str,
    is_directory: bool,
    suffix: &'a str
}

struct FilenameComponents {
    file_stem: String,
    file_extension: String
}

impl<'a> Compressor<'a> {
    pub fn new(input_path: &'a str, output_path: &'a str, is_directory: bool, suffix: &'a str) -> Self {
        Compressor {
            input_path,
            output_path,
            is_directory,
            suffix
        }
    }

    pub fn compress(&self) {
        if !self.is_directory {
            self.compress_file(Path::new(self.input_path));
        } else if self.is_directory {
            // read the directory
            for entry in fs::read_dir(self.input_path).unwrap() {
                let entry = entry.unwrap();
                let input_path = entry.path();

                if input_path.as_path().is_file() {
                    self.compress_file(&input_path);
                }
            }
        }
    }

    fn compress_file(&self, input_file_path: &Path) {
        let output_file_path = &self.get_output_file_path(Path::new(input_file_path));

        let filename_components = get_filename_components(input_file_path);

        match filename_components.file_extension.as_str() {
            "png" => {
                png_compressor::compress(input_file_path, output_file_path);
            }
            "jpeg" => {
                jpeg_compressor::compress(input_file_path, output_file_path);
            }
            "jpg" => {
                jpeg_compressor::compress(input_file_path, output_file_path);
            }
            _ => {
                println!("Unrecognized file type");
            }
        }
    }

    fn get_output_file_path(&self, input_file_path: &Path) -> PathBuf {
        let mut output_dir = PathBuf::from_str(self.output_path).unwrap();
        let input_file_components = get_filename_components(input_file_path);
        let final_name = format!("{}-{}.{}", input_file_components.file_stem, self.suffix, input_file_components.file_extension);

        output_dir.push(final_name);
        output_dir
    } 
}

mod png_compressor {
    use oxipng::{self, InFile, OutFile, Options};
    use std::path::Path;

    pub fn compress(input_file_path: &Path, output_file_path: &Path) {
        let mut optimize_options = Options::default();
        optimize_options.fix_errors = true;
        optimize_options.force = true;
        optimize_options.preserve_attrs = true;


        oxipng::optimize(
            &InFile::Path(input_file_path.to_path_buf()), 
            &OutFile::Path(Some(output_file_path.to_path_buf())), 
            &optimize_options
            // &Options::max_compression()
        ).unwrap()
    }
}

mod jpeg_compressor {
    use std::path::Path;
    use std::fs::File;
    use std::convert::TryInto;
    use image::codecs::jpeg;
    use image::ImageDecoder;

    pub fn compress(input_file_path: &Path, output_file_path: &Path) {
        let image_file = File::open(input_file_path).unwrap();

        let decoder = jpeg::JpegDecoder::new(image_file).unwrap();
        let (image_width, image_height) = decoder.dimensions();
        let image_color_type = decoder.color_type();

        let total_bytes_in_image = decoder.total_bytes();
        let mut image_data_vec = vec![0; total_bytes_in_image.try_into().unwrap()];
        
        decoder.read_image(&mut image_data_vec).unwrap();

        // decoding is now done. Should encode it and save it into a file
        let mut output_file = File::create(output_file_path).unwrap();

        let mut encoder = jpeg::JpegEncoder::new_with_quality(&mut output_file, 80);
        encoder.encode(&image_data_vec, image_width, image_height, image_color_type).unwrap();
    }
}

fn get_filename_components(file_path: &Path) -> FilenameComponents {
    let file_stem = file_path.file_stem().unwrap().to_str().unwrap();
    let file_ext = file_path.extension().unwrap().to_str().unwrap();

    FilenameComponents {
        file_stem: String::from(file_stem),
        file_extension: String::from(file_ext)
    }
}