use clap::Parser;

#[derive(Parser, Clone)]
#[command(about = "Create directories", author = "Abdulrahman Idrees", version = "0.1.0")]
pub struct Args {
    /// Directories to create
    #[arg(required = true)]
    pub directories: Vec<String>,

    /// Create parent directories as needed
    #[arg(short = 'p', long)]
    pub parents: bool,

    /// Set file mode (as in chmod), not a=rwx - umask
    #[arg(short = 'm', long = "mode")]
    pub mode: Option<String>,

    /// Print a message for each created directory
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,
}
