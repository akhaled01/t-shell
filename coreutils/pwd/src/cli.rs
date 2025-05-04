use clap::Parser;

#[derive(Parser, Clone)]
#[command(about = "Print the current working directory", author = "Abdulrahman Idrees", version = "0.1.0")]
pub struct Args {
    /// Use physical directory structure instead of following symbolic links
    #[arg(short = 'P', long)]
    pub physical: bool,

    /// Use logical directory structure (follow symbolic links)
    #[arg(short = 'L', long, conflicts_with = "physical")]
    pub logical: bool,
}
