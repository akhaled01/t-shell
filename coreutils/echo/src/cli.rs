#[derive(clap::Parser)]
pub struct Args {
    #[clap(short = 'n', long)]
    pub no_newline: bool,

    #[clap(short = 'e', long)]
    pub enable_escaping: bool,

    #[clap(short = 'E', long)]
    pub disable_escaping: bool,

    /// The text to echo
    #[clap(value_parser)]
    pub text: Vec<String>,
}
