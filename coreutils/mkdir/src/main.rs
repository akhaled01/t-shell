use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use clap::Parser;

mod cli;

fn create_directory(path: &Path, args: &cli::Args) -> io::Result<()> {
    if path.exists() && !args.parents {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("cannot create directory '{}': File exists", path.display())
        ));
    }

    if args.parents {
        fs::create_dir_all(path)
    } else {
        fs::create_dir(path)
    }?;

    if let Some(mode_str) = &args.mode {
        if let Ok(mode) = u32::from_str_radix(mode_str, 8) {
            fs::set_permissions(path, fs::Permissions::from_mode(mode))?;
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("invalid mode '{}'", mode_str)
            ));
        }
    }

    if args.verbose {
        println!("created directory '{}'", path.display());
    }

    Ok(())
}

fn main() {
    let args = cli::Args::parse();
    let mut had_error = false;

    for dir in &args.directories {
        let path = Path::new(dir);
        if let Err(e) = create_directory(path, &args) {
            eprintln!("mkdir: {}", e);
            had_error = true;
        }
    }

    if had_error {
        std::process::exit(1);
    }
}
