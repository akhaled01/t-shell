mod cli;

use clap::Parser;
use std::fs;
use std::fs::File;
use std::io::Read;

fn main() {
    let args = cli::Args::parse();

    let mut contents = String::new();

    args.files.iter().for_each(|file| {
        if fs::metadata(file).map(|m| m.is_dir()).unwrap_or(false) {
            println!("cat: {}: is a directory", file.clone());
            return;
        }
        let mut file = File::open(file.clone()).unwrap();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
        contents.clear();
    });
}
