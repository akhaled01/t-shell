use std::fs;
use std::io;
use std::path::Path;
use clap::Parser;

mod cli;

fn remove_path(path: &Path, recursive: bool) -> io::Result<()> {
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("cannot remove '{}': No such file or directory", path.display())
        ));
    }

    if path.is_dir() {
        if !recursive {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!("cannot remove '{}': Is a directory", path.display())
            ));
        }
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}

fn main() {
    let args = cli::Args::parse();
    let mut had_error = false;

    for path in &args.paths {
        let path = Path::new(path);
        if let Err(e) = remove_path(path, args.recursive) {
            eprintln!("rm: {}", e);
            had_error = true;
        }
    }

    if had_error {
        std::process::exit(1);
    }
}
