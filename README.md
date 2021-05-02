A very basic image compressor built in Rust language. Currently supports compression of JPEG & PNG images.

## How To Use

1. Clone this project. And make sure you have installed Rust & Cargo.
2. Run `cargo run "image_path"`

For faster results, the project has to be built in release mode

1. Run `cargo build --release`
2. Move to the directory with the executable
3. `image-compressor "iamge_path"`

## Notes

1. The compression level is fixed. It can be made dynamic 
2. The output file is saved in the same directory as the input file with the filename `${input_file_name}-compressed.${extension}`
3. Only single files can be compressed at a time. Can support batch processing.
4. Can provide binding for NodeJS