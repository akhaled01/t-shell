#[derive(clap::Parser, Clone)]
pub struct Args {
    #[clap(short = 'l', long)]
    pub long: bool,

    #[clap(short = 'a', long)]
    pub all: bool,

    #[clap(short = 'F', long)]
    pub classify: bool,

    /// Files or directories to list
    #[arg(default_value = ".", trailing_var_arg = true)]
    pub paths: Vec<String>,
}