mod cli;

use std::env;
use std::path::PathBuf;
use clap::Parser;

fn get_current_dir(physical: bool) -> std::io::Result<PathBuf> {
    if physical {
        env::current_dir()?.canonicalize()
    } else {
        env::current_dir()
    }
}

fn main() {
    let args = cli::Args::parse();
    
    let physical = args.physical || !args.logical;
    
    match get_current_dir(physical) {
        Ok(path) => println!("{}", path.display()),
        Err(e) => {
            eprintln!("pwd: {}", e);
            std::process::exit(1);
        }
    }
}
