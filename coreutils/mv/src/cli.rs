use clap::Parser;

#[derive(Parser, Clone)]
#[command(about = "Move (rename) files and directories", author = "Abdulrahman Idrees", version = "0.1.0")]
pub struct Args {
    /// Source file(s) or directory
    #[arg(required = true)]
    pub sources: Vec<String>,

    /// Destination file or directory
    #[arg(required = true, last = true)]
    pub destination: String,

    /// Do not prompt before overwriting
    #[arg(short = 'f', long = "force")]
    pub force: bool,

    /// Prompt before overwrite
    #[arg(short = 'i', long = "interactive", conflicts_with = "force")]
    pub interactive: bool,

    /// Do not overwrite an existing file
    #[arg(short = 'n', long = "no-clobber", conflicts_with_all = ["force", "interactive"])]
    pub no_clobber: bool,

    /// Print moved files
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,
}
