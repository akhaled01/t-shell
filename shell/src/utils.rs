use std::{env, path::Path};

pub fn get_current_path() -> String {
    env::current_dir()
        .map(|path| path.to_string_lossy().to_string())
        .unwrap_or_else(|_| String::from("?"))
}

pub fn get_user_and_host() -> String {
    let username = env::var("USER").unwrap_or_else(|_| String::from("user"));
    let hostname = hostname::get()
        .ok()
        .and_then(|h| h.into_string().ok())
        .unwrap_or_else(|| String::from("host"));
    format!("{username}@{hostname}")
}

pub fn cd(path: &str) {
    if path == "-" {
        env::set_current_dir(env::var("OLDPWD").unwrap()).unwrap();
    } else {
        if Path::new(path).exists() {
            env::set_current_dir(path).unwrap();
        } else {
            if path == "~" || path.trim().is_empty() {
                env::set_current_dir(env::var("HOME").unwrap()).unwrap();
            } else {
                println!("Path {} does not exist", path);
            }
        }
    }
    unsafe {
        env::set_var("OLDPWD", env::current_dir().unwrap());
    }
}
