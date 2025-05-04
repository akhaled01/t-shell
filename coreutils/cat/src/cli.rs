#[derive(clap::Parser, Clone)]
#[command(about = "Concatenate files and print to standard output", author = "Abdulrahman Idrees", version = "0.1.0")]
pub struct Args {
    /// Files to concatenate
    #[arg(required = true)]
    pub files: Vec<String>,
}
