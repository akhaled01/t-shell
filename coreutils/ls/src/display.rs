use crate::fsops::{Entry, EntryType};
use crate::utils::lookup::LookupCtx;
use crate::utils::permissions::PermissionsWrapper;
use std::os::unix::fs::MetadataExt;

pub struct DisplayContext {
    long: bool,
    classify: bool,
}

impl DisplayContext {
    fn get_type_indicator(&self, entry_type: &EntryType) -> &'static str {
        if !self.classify {
            return "";
        }
        match entry_type {
            EntryType::Directory => "/",
            EntryType::RegularFile => "",
            EntryType::ExecutableFile => "*",
            EntryType::SymbolicLink => "@",
            EntryType::Socket => "=",
            EntryType::Whiteout => "%",
            EntryType::FIFO => "|",
        }
    }

    fn calculate_blocks(&self, entries: &Vec<Entry>) -> u64 {
        let total: u64 = entries
            .iter()
            .map(|entry| (entry.metadata.blocks() * 512 / 1024) * 2)
            .sum();
        total
    }

    pub fn new(long: bool, classify: bool) -> Self {
        Self { long, classify }
    }

    pub fn display_entry_long(&self, entry: &Entry) {
        let metadata = entry.metadata.clone();
        let permissions = metadata.permissions();
        let owner = metadata.uid();
        let group = metadata.gid();
        let size = metadata.len();
        let modified = metadata.modified().unwrap();
        let name = entry.path.clone();

        let name_with_indicator = format!("{}{}", name, self.get_type_indicator(&entry.entry_type));
        let display_name = if let Some(target) = &entry.symlink_target {
            format!("{} -> {}", name_with_indicator, target)
        } else {
            name_with_indicator
        };

        println!(
            "{} {} {} {} {} {}",
            PermissionsWrapper::from(permissions).to_ls_string(),
            LookupCtx.lookup_owner(owner),
            LookupCtx.lookup_group(group),
            size,
            modified.elapsed().unwrap().as_secs(),
            display_name
        );
    }

    pub fn display_entries_long(&self, entries: &Vec<Entry>) {
        println!("total {}", self.calculate_blocks(entries));
        for entry in entries {
            self.display_entry_long(entry);
        }
    }

    pub fn display_entry(&self, entry: &Entry) {
        if self.long {
            self.display_entry_long(entry);
        } else {
            self.display_entry_short(entry);
        }
    }

    pub fn display_entry_short(&self, entry: &Entry) {
        println!(
            "{}{}",
            entry.path,
            self.get_type_indicator(&entry.entry_type)
        );
    }
}
