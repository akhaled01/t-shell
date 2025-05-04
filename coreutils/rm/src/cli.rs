use clap::Parser;

#[derive(Parser, Clone)]
#[command(about = "Remove files or directories", author = "Abdulrahman Idrees", version = "0.1.0")]
pub struct Args {
    /// Files or directories to remove
    #[arg(required = true)]
    pub paths: Vec<String>,

    /// Remove directories and their contents recursively
    #[arg(short = 'r', long = "recursive")]
    pub recursive: bool,
}
