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

        if file.name().ends_with('/') {
            println!("Directory extracted to \"{}\"", outpath.display());
            fs::create_dir_all(&outpath).expect("Failed to create directory");
        } else {
            println!("File extracted to \"{}\" ({} bytes)", outpath.display(), file.size());
            let parent = outpath.parent().expect("Failed to get parent directory");
            fs::create_dir_all(parent).expect("Failed to create parent directories");

            let outpath_clone = outpath.clone();
            let mut outfile = File::create(outpath_clone).expect("Failed to create file");
            copy(&mut file, &mut outfile).expect("Failed to copy file content");
        }

        #[cfg(target_os = "linux")]
        if let Some(mode) = file.unix_mode() {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).expect("Failed to set permissions");
        }
    }

    return 0;
}