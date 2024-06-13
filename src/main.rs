use std::fs;
use std::io;



fn main() {
    std::process::exit(real_main());
}


fn real_main() -> i32 {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        
        return 1;
    }

    let filename = std::path::Path::new(&*args[1]);
    let file = fs::File::open(&filename).unwrap();

    
}