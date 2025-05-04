mod cli;

use std::fs::File;
use std::io::Read;
use clap::Parser;

fn main() {
    let args = cli::Args::parse();
    
    let mut contents = String::new();
    
    args.files.iter().for_each(|file| {
        let mut file = File::open(file.clone()).unwrap();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
        contents.clear();
    });
}
