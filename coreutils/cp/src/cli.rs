use clap::Parser;

#[derive(Parser, Clone)]
#[command(about = "Copy files and directories", author = "Abdulrahman Idrees", version = "0.1.0")]
pub struct Args {
    /// Source file or directory
    #[arg(required = true)]
    pub source: String,

    /// Destination file or directory
    #[arg(required = true)]
    pub destination: String,

    /// Copy directories recursively
    #[arg(short = 'R', long = "recursive")]
    pub recursive: bool,

    /// Follow symbolic links in source
    #[arg(short = 'L', long = "follow")]
    pub follow: bool,

    /// Overwrite existing files without prompting
    #[arg(short = 'f', long = "force")]
    pub force: bool,

    /// Prompt before overwrite
    #[arg(short = 'i', long = "interactive", conflicts_with = "force")]
    pub interactive: bool,

    /// Don't overwrite existing files
    #[arg(short = 'n', long = "no-clobber", conflicts_with_all = ["force", "interactive"])]
    pub no_clobber: bool,

    /// Preserve attributes when possible
    #[arg(short = 'p', long = "preserve")]
    pub preserve: bool,
}
