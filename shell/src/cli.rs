use clap::{Parser, arg};
use std::env;

#[derive(Parser)]
#[command(name = "shell", about = "T-shell is a UNIX compliant shell written in rust", long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = false)]
    pub production: bool,
}

impl Cli {
    pub fn get_path(&self) -> String {
        if self.production {
            env::var("PATH").unwrap()
        } else {
            let home = env::var("HOME").unwrap();
            format!("{}/target/debug", home)
        }
    }
}
