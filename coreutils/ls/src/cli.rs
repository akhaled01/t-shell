#[derive(clap::Parser, Clone)]
#[command(about = "List directory contents", author = "Abdulrahman Idrees", version = "0.1.0")]
pub struct Args {
    #[clap(short = 'l', long, help = "Use a long listing format")]
    pub long: bool,

    #[clap(short = 'a', long, help = "Do not ignore hidden entries starting with .")]
    pub all: bool,

    #[clap(short = 'F', long, help = "Append indicator (one of */=) to entries")]
    pub classify: bool,

    /// Files or directories to list
    #[arg(default_value = ".", trailing_var_arg = true)]
    pub paths: Vec<String>,
}