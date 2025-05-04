use std::process::Command;

fn split_path(path: &str) -> Vec<&str> {
    path.split(':').collect()
}

fn lookup_bin(bin_name: &str, path: &str) -> Option<String> {
    let paths = split_path(path);
    for p in paths {
        let path = format!("{}/{}", p, bin_name);
        if std::path::Path::new(&path).exists() {
            return Some(path);
        }
    }
    None
}

pub fn execute_command(bin_name: &str, args: &[&str], path: &str) {
    let bin_path = lookup_bin(bin_name, path);

    if bin_path.is_none() {
        println!("Command {} not found", bin_name);
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
