use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use clap::Parser;

mod cli;

fn prompt_overwrite(path: &Path) -> bool {
    print!("mv: overwrite '{}'? ", path.display());
    io::stdout().flush().unwrap();
    let mut response = String::new();
    io::stdin().read_line(&mut response).unwrap();
    response.trim().eq_ignore_ascii_case("y")
}

fn move_item(source: &Path, dest: &Path, args: &cli::Args) -> io::Result<()> {
    if !source.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("cannot stat '{}': No such file or directory", source.display())
        ));
    }

    if source == dest {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("'{}' and '{}' are the same file", source.display(), dest.display())
        ));
    }

    if dest.exists() && !args.force {
        if args.no_clobber {
            return Ok(());
        }
        if args.interactive && !prompt_overwrite(dest) {
            return Ok(());
        }
    }

    // Try rename first (fast path for same filesystem)
    match fs::rename(source, dest) {
        Ok(_) => {
            if args.verbose {
                println!("renamed '{}' -> '{}'", source.display(), dest.display());
            }
            Ok(())
        }
        Err(e) => {
            if e.kind() == io::ErrorKind::CrossesDevices {
                // Fall back to copy + remove for cross-device moves
                fs::copy(source, dest)?;
                fs::remove_file(source)?;
                if args.verbose {
                    println!("moved '{}' -> '{}'", source.display(), dest.display());
                }
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

fn main() {
    let args = cli::Args::parse();
    let mut had_error = false;
    let dest = Path::new(&args.destination);

    // If we have multiple sources, destination must be a directory
    if args.sources.len() > 1 && !dest.is_dir() {
        eprintln!("mv: target '{}' is not a directory", args.destination);
        std::process::exit(1);
    }

    for source in &args.sources {
        let source_path = Path::new(source);
        let dest_path = if dest.is_dir() {
            dest.join(source_path.file_name().unwrap_or_default())
        } else {
            PathBuf::from(dest)
        };

        if let Err(e) = move_item(source_path, &dest_path, &args) {
            eprintln!("mv: {}", e);
            had_error = true;
        }
    }

    if had_error {
        std::process::exit(1);
    }
}
