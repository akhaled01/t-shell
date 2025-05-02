use clap::Parser;
use echo::cli::Args;

fn main() {
    let args = Args::parse();

    let res = args
        .text
        .iter()
        .map(|s| {
            if args.enable_escaping && !args.disable_escaping {
                s.chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if c == '\\' && i + 1 < s.len() {
                            match s.chars().nth(i + 1) {
                                Some('n') => "\n",
                                Some('t') => "\t",
                                Some('r') => "\r",
                                Some('\\') => "\\",
                                Some(_) => &s[i..i + 2],
                                None => "\\",
                            }
                        } else if i > 0 && s.chars().nth(i - 1) == Some('\\') {
                            ""
                        } else {
                            &s[i..i + 1]
                        }
                    })
                    .collect()
            } else {
                s.clone()
            }
        })
        .collect::<Vec<String>>();

    if args.no_newline {
        print!("{}", res.join(" "));
    } else {
        print!("{}", res.join(" "));
        println!();
    }
}
