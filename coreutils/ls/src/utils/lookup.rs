use nix::unistd::Group;
use nix::unistd::{Gid, Uid, User};

pub struct LookupCtx;

impl LookupCtx {
    pub fn lookup_owner(&self, uid: u32) -> String {
        let uid = Uid::from_raw(uid);
        User::from_uid(uid)
            .ok()
            .flatten()
            .map(|user| user.name)
            .unwrap_or(uid.to_string())
    }

    pub fn lookup_group(&self, gid: u32) -> String {
        let gid = Gid::from_raw(gid);
        Group::from_gid(gid)
            .ok()
            .flatten()
            .map(|group| group.name)
            .unwrap_or(gid.to_string())
    }
}
    