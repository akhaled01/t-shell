use std::fs::{self, File, Metadata};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use clap::Parser;

mod cli;

fn copy_attributes(source: &Metadata, dest_path: &Path) -> io::Result<()> {
    let perms = source.permissions();
    fs::set_permissions(dest_path, perms)?;
    Ok(())
}

fn copy_file(source: &Path, dest: &Path, args: &cli::Args) -> io::Result<()> {
    if dest.exists() && !args.force {
        if args.interactive {
            print!("cp: overwrite '{}'? ", dest.display());
            io::stdout().flush()?;
            let mut response = String::new();
            io::stdin().read_line(&mut response)?;
            if !response.trim().eq_ignore_ascii_case("y") {
                return Ok(());
            }
        } else if args.no_clobber {
            return Ok(());
        }
    }

    let mut source_file = File::open(source)?;
    let mut dest_file = File::create(dest)?;
    io::copy(&mut source_file, &mut dest_file)?;

    if args.preserve {
        let metadata = fs::metadata(source)?;
        copy_attributes(&metadata, dest)?;
    }

    Ok(())
}

fn copy_recursive(source: &Path, dest: &Path, args: &cli::Args) -> io::Result<()> {
    let metadata = if args.follow {
        fs::metadata(source)?
    } else {
        fs::symlink_metadata(source)?
    };

    if metadata.is_dir() {
        if !args.recursive {
            eprintln!("cp: -R not specified; omitting directory '{}'", source.display());
            return Ok(());
        }

        fs::create_dir_all(dest)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let new_dest = dest.join(entry.file_name());
            copy_recursive(&entry.path(), &new_dest, args)?;
        }
        if args.preserve {
            copy_attributes(&metadata, dest)?;
        }
    } else {
        copy_file(source, dest, args)?;
    }

    Ok(())
}

fn main() {
    let args = cli::Args::parse();
    
    if args.source == args.destination {
        eprintln!("cp: cannot copy {} to itself", args.source);
        std::process::exit(1);
    }

    let source = Path::new(&args.source);
    let destination = Path::new(&args.destination);

    if !source.exists() {
        eprintln!("cp: cannot access '{}': No such file or directory", args.source);
        std::process::exit(1);
    }

    let dest_path = if destination.is_dir() {
        destination.join(source.file_name().unwrap_or_default())
    } else {
        PathBuf::from(destination)
    };

    if let Err(e) = copy_recursive(source, &dest_path, &args) {
        eprintln!("cp: {}", e);
        std::process::exit(1);
    }
}
