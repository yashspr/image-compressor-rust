use std::path::{ Path, PathBuf };

struct FilenameComponents {
    file_stem: String,
    file_extension: String
}

pub fn compress_file(file_path: &Path) {
    let filename_components = get_filename_components(file_path);

    match filename_components.file_extension.as_str() {
        "png" => {
            png_compressor::compress(file_path);
        }
        "jpeg" => {
            jpeg_compressor::compress(file_path);
        }
        "jpg" => {
            jpeg_compressor::compress(file_path);
        }
        _ => {
            println!("Unrecognized file type");
        }
    }
}

mod png_compressor {
    use oxipng::{self, InFile, OutFile, Options};
    use super::{get_dest_path, get_output_filename};
    use std::path::Path;

    pub fn compress(file_path: &Path) {
        let new_filename = get_output_filename(file_path.file_name().unwrap().to_str().unwrap());
        let mut dest_file_path = get_dest_path(file_path).unwrap();
        dest_file_path.push(Path::new(&new_filename));

        let mut optimize_options = Options::default();
        optimize_options.fix_errors = true;
        optimize_options.force = true;
        optimize_options.preserve_attrs = true;


        oxipng::optimize(
            &InFile::Path(file_path.to_path_buf()), 
            &OutFile::Path(Some(dest_file_path)), 
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
    use super::{get_output_filepath_from_path};

    pub fn compress(file_path: &Path) {
        let image_file = File::open(file_path).unwrap();

        let decoder = jpeg::JpegDecoder::new(image_file).unwrap();
        let (image_width, image_height) = decoder.dimensions();
        let image_color_type = decoder.color_type();

        let total_bytes_in_image = decoder.total_bytes();
        let mut image_data_vec = vec![0; total_bytes_in_image.try_into().unwrap()];
        
        decoder.read_image(&mut image_data_vec).unwrap();

        // decoding is now done. Should encode it and save it into a file
        let output_path = get_output_filepath_from_path(file_path);
        let mut output_file = File::create(output_path.as_path().to_str().unwrap()).unwrap();

        let mut encoder = jpeg::JpegEncoder::new_with_quality(&mut output_file, 80);
        encoder.encode(&image_data_vec, image_width, image_height, image_color_type).unwrap();
    }

    // mozjpeg code

    // pub fn compress(file_path: &Path) {
    //     let d = mozjpeg::Decompress::with_markers(mozjpeg::ALL_MARKERS)
    //         .from_path(file_path.to_str().unwrap())
    //         .unwrap();

    //     d.width();
    //     d.height();

    //     let image = d.rgb().unwrap();
    //     let pixel_data = image.read_scanlines::<[u8; 3]>().unwrap();

    //     let compress_obj = mozjpeg::Compress::new(image.color_space());

    //     // for marker_data in d.markers() {
    //     //     compress_obj.write_marker(marker_data.marker, marker_data.data);
    //     // }

    //     compress_obj.set_optimize_scans(true);
    //     compress_obj.set_quality(80.0);
    //     compress_obj.set_size(image.height(), image.width());
    //     compress_obj.write_scanlines(&pixel_data);

    //     compress_obj.start_compress();
    //     compress_obj.finish_compress();

    //     let compressed_data = compress_obj.data_to_vec();
    // }
}

fn get_dest_path(input_path: &Path) -> Option<PathBuf> {
    let mut input_path_buf = input_path.to_path_buf();

    if input_path.is_dir() {
        // input_path_buf.push(Path::new("compressed"));
        Some(input_path_buf)
    } else if input_path.is_file() {
        input_path_buf.pop();
        // input_path_buf.push(Path::new("compressed"));
        Some(input_path_buf)
    } else {
        None
    }
}

fn get_output_filename(filename: &str) -> String {
    let filename_path = Path::new(filename);
    let filename_components = get_filename_components(filename_path);

    let new_filename = format!("{}-compressed.{}", filename_components.file_stem, filename_components.file_extension);

    new_filename
}

fn get_output_filepath_from_path(file_path: &Path) -> PathBuf {
    let new_filename = get_output_filename(file_path.file_name().unwrap().to_str().unwrap());
    let mut dest_file_path = get_dest_path(file_path).unwrap();
    dest_file_path.push(Path::new(&new_filename));
    dest_file_path
}

fn get_filename_components(file_path: &Path) -> FilenameComponents {
    let file_stem = file_path.file_stem().unwrap().to_str().unwrap();
    let file_ext = file_path.extension().unwrap().to_str().unwrap();

    FilenameComponents {
        file_stem: String::from(file_stem),
        file_extension: String::from(file_ext)
    }
}