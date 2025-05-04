use clap::Parser;
use shell::utils::{get_current_path, get_user_and_host};
use std::io::{self, Write};
use std::process;

fn main() {
    let shell_args = shell::cli::Cli::parse();

    ctrlc::set_handler(move || {
        println!();
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    loop {
        let user_host = get_user_and_host();
        let current_path = get_current_path();
        print!("{user_host}:{current_path}$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            println!();
            break;
        }

        let input = input.trim();

        if input == "exit" {
            break;
        } else if input == "cd" {
            shell::utils::cd("-");
            continue;
        } else if input.starts_with("cd ") {
            let path = &input[3..];
            shell::utils::cd(path);
            continue;
        }

        let args: Vec<&str> = input.split_whitespace().collect();
        let bin_name = args[0];
        let args = &args[1..];

        shell::exec::execute_command(bin_name, args, &shell_args.get_path());
    }
}
