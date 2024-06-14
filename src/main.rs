use std::env;
use std::fs::{ self, File };
use std::io::{ copy };
use std::path::Path;
use zip::read::ZipArchive;





fn main() {
    std::process::exit(real_main());
}



fn real_main() -> i32 {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }

    let filename = Path::new(&args[1]);
    let file = File::open(filename).expect("Failed to open file");

    let mut archive = ZipArchive::new(file).expect("Failed to create archive");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).expect("Failed to read entry");
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        
    }
}