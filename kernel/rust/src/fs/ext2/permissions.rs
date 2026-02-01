use crate::fs::ext2::inode::Inode;

pub const S_IRUSR: u16 = 0o400;
pub const S_IWUSR: u16 = 0o200;
pub const S_IXUSR: u16 = 0o100;
pub const S_IRGRP: u16 = 0o040;
pub const S_IWGRP: u16 = 0o020;
pub const S_IXGRP: u16 = 0o010;
pub const S_IROTH: u16 = 0o004;
pub const S_IWOTH: u16 = 0o002;
pub const S_IXOTH: u16 = 0o001;

pub const S_IFDIR: u16 = 0x4000;
pub const S_IFREG: u16 = 0x8000;

pub fn check_permission(inode: &Inode, uid: u32, gid: u32, read: bool, write: bool, execute: bool) -> bool {
    let mode = inode.mode;
    
    if uid == 0 {
        return true;
    }
    
    let perms = if uid == inode.uid {
        (mode >> 6) & 0o7
    } else if gid == inode.gid {
        (mode >> 3) & 0o7
    } else {
        mode & 0o7
    };
    
    if read && (perms & 0o4) == 0 {
        return false;
    }
    if write && (perms & 0o2) == 0 {
        return false;
    }
    if execute && (perms & 0o1) == 0 {
        return false;
    }
    
    true
}

pub fn set_permissions(mode: &mut u16, perms: u16) {
    *mode = (*mode & !0o777) | (perms & 0o777);
}

pub fn set_owner(inode: &mut Inode, uid: u32, gid: u32) {
    inode.uid = uid;
    inode.gid = gid;
}
