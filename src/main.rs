mod compress;

use std::env;
use std::path::Path;

use compress::compress_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    
    if args.len() <= 1 {
        panic!("Please pass a path to image");
    }

    let filename = &args[1];
    let file_path = Path::new(filename);

    println!("The path you entered: {}", file_path.to_str().unwrap());

    compress_file(file_path);

    println!("Optimized Image Successfully");
}
