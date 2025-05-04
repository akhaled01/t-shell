use std::{fs::Permissions, os::unix::fs::PermissionsExt as _};

pub struct PermissionsWrapper(Permissions);

impl From<Permissions> for PermissionsWrapper {
    fn from(perms: Permissions) -> Self {
        PermissionsWrapper(perms)
    }
}

impl Into<Permissions> for PermissionsWrapper {
    fn into(self) -> Permissions {
        self.0
    }
}

impl PermissionsWrapper {
    pub fn mode(&self) -> u32 {
        self.0.mode()
    }

    pub fn to_ls_string(&self) -> String {
        let mode = self.mode();
        let mut result = String::with_capacity(10);

        // File type
        if mode & 0o170000 == 0o040000 {
            result.push('d');
        } else if mode & 0o170000 == 0o120000 {
            result.push('l');
        } else if mode & 0o170000 == 0o140000 {
            result.push('s');
        } else if mode & 0o170000 == 0o010000 {
            result.push('p');
        } else if mode & 0o170000 == 0o060000 {
            result.push('b');
        } else if mode & 0o170000 == 0o020000 {
            result.push('c');
        } else {
            result.push('-');
        }

        // Owner permissions
        result.push(if mode & 0o400 != 0 { 'r' } else { '-' });
        result.push(if mode & 0o200 != 0 { 'w' } else { '-' });
        result.push(if mode & 0o100 != 0 {
            if mode & 0o4000 != 0 { 's' } else { 'x' }
        } else {
            if mode & 0o4000 != 0 { 'S' } else { '-' }
        });

        // Group permissions
        result.push(if mode & 0o040 != 0 { 'r' } else { '-' });
        result.push(if mode & 0o020 != 0 { 'w' } else { '-' });
        result.push(if mode & 0o010 != 0 {
            if mode & 0o2000 != 0 { 's' } else { 'x' }
        } else {
            if mode & 0o2000 != 0 { 'S' } else { '-' }
        });

        // Others permissions
        result.push(if mode & 0o004 != 0 { 'r' } else { '-' });
        result.push(if mode & 0o002 != 0 { 'w' } else { '-' });
        result.push(if mode & 0o001 != 0 {
            if mode & 0o1000 != 0 { 't' } else { 'x' }
        } else {
            if mode & 0o1000 != 0 { 'T' } else { '-' }
        });

        result
    }
}