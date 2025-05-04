use std::{fs, os::unix::fs::{MetadataExt as _, FileTypeExt}};

use crate::cli;

pub struct SearchContext {
    pub args: cli::Args,
}

#[derive(Debug)]
pub enum EntryType {
    Directory,      // /
    RegularFile,    // (no suffix)
    ExecutableFile, // *
    SymbolicLink,   // @
    Socket,         // =
    Whiteout,       // %
    FIFO,           // |
}

#[derive(Debug)]
pub struct Entry {
    pub path: String,
    pub metadata: fs::Metadata,
    pub entry_type: EntryType,
    pub symlink_target: Option<String>,
}

impl SearchContext {
    fn create_entry(&self, path: String, metadata: fs::Metadata, full_path: &std::path::Path) -> Entry {
        let file_type = metadata.file_type();
        let mode = metadata.mode();
        let is_executable = mode & 0o111 != 0; // Check if any execute bit is set

        let entry_type = if file_type.is_dir() {
            EntryType::Directory
        } else if file_type.is_symlink() {
            EntryType::SymbolicLink
        } else if file_type.is_socket() {
            EntryType::Socket
        } else if file_type.is_fifo() {
            EntryType::FIFO
        } else if is_executable && file_type.is_file() {
            EntryType::ExecutableFile
        } else {
            EntryType::RegularFile
        };

        let symlink_target = if metadata.file_type().is_symlink() {
            fs::read_link(full_path).ok().map(|p| p.to_string_lossy().into_owned())
        } else {
            None
        };

        Entry {
            path,
            metadata,
            entry_type,
            symlink_target,
        }
    }
    pub fn new(args: cli::Args) -> Self {
        Self { args }
    }

    fn is_hidden(path: &str) -> bool {
        path.split('/').last()
            .map(|name| name.starts_with("."))
            .unwrap_or(false)
    }

    pub fn search(&self) -> Result<Vec<Entry>, std::io::Error> {
        let mut entries = Vec::new();
        for base_path in &self.args.paths {
            // If path is a directory, read its contents
            let metadata = fs::metadata(base_path)?;
            if metadata.is_dir() {
                // Add . and .. if showing all entries
                if self.args.all {
                    // Add . and .. first
                    if let Ok(dot_meta) = fs::metadata(base_path) {
                        entries.push(self.create_entry(".".to_string(), dot_meta, std::path::Path::new(base_path)));
                    }
                    // For .. entry, go up one level
                    let parent = if base_path == "." {
                        std::path::Path::new("..")
                    } else {
                        std::path::Path::new(base_path).parent()
                            .unwrap_or(std::path::Path::new(".."))
                    };
                    if let Ok(dotdot_meta) = fs::metadata(parent) {
                        entries.push(self.create_entry("..".to_string(), dotdot_meta, parent));
                    }
                }
                
                for entry in fs::read_dir(base_path)? {
                    let entry = entry?;
                    let path = entry.path();
                    let display_name = path.strip_prefix(base_path)
                        .unwrap_or(&path)
                        .to_string_lossy()
                        .into_owned();
                    if let Ok(metadata) = fs::symlink_metadata(&path) {
                        let entry = self.create_entry(display_name, metadata, &path);
                        if self.args.all || !Self::is_hidden(&path.to_string_lossy()) {
                            entries.push(entry);
                        }
                    }
                }
            } else {
                // Handle single file
                let display_name = base_path.strip_prefix("./").unwrap_or(base_path);
                let entry = self.create_entry(display_name.to_string(), metadata, std::path::Path::new(base_path));
                if self.args.all || !Self::is_hidden(base_path) {
                    entries.push(entry);
                }
            }
        }

        // Sort entries, but keep . and .. at the start
        entries.sort_by(|a, b| {
            // Special handling for . and ..
            if a.path == "." { return std::cmp::Ordering::Less; }
            if b.path == "." { return std::cmp::Ordering::Greater; }
            if a.path == ".." { return std::cmp::Ordering::Less; }
            if b.path == ".." { return std::cmp::Ordering::Greater; }

            // Then sort by type and name
            match (&a.entry_type, &b.entry_type) {
                (EntryType::Directory, EntryType::RegularFile) => std::cmp::Ordering::Less,
                (EntryType::RegularFile, EntryType::Directory) => std::cmp::Ordering::Greater,
                _ => a.path.cmp(&b.path),
            }
        });
        Ok(entries)
    }
}