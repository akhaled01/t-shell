use std::process::Command;
use std::path::Path;
use std::env;

fn split_path(path: &str) -> Vec<&str> {
    path.split(':').collect()
}

fn lookup_bin(bin_name: &str, path: &str, is_prod: bool) -> Option<String> {
    if !is_prod {
        // In non-prod mode, check the local coreutils first
        let home = env::var("HOME").unwrap();
        let local_bin = format!("{}/01/tsh/target/debug/{}", home, bin_name);
        if Path::new(&local_bin).exists() {
            return Some(local_bin);
        }
    }
    
    // Fall back to path lookup
    let paths = split_path(path);
    for p in paths {
        let path = format!("{}/{}", p, bin_name);
        if Path::new(&path).exists() {
            return Some(path);
        }
    }
    None
}

pub fn execute_command(bin_name: &str, args: &[&str], path: &str, is_prod: bool) {
    let bin_path = lookup_bin(bin_name, path, is_prod);

    if bin_path.is_none() {
        println!("Command '{}' not found", bin_name);
        return;
    }

    let output = Command::new(bin_path.unwrap())
        .args(args)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}
