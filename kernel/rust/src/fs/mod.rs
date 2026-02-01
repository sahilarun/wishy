pub mod mount;
pub mod cache;
pub mod ext2;

use alloc::vec::Vec;

pub trait Filesystem: Send + Sync {
    fn read_file(&self, path: &str) -> Result<Vec<u8>, ()>;
    fn write_file(&self, path: &str, data: &[u8]) -> Result<(), ()>;
    fn create_file(&self, path: &str, mode: u16, uid: u32, gid: u32) -> Result<(), ()>;
    fn delete_file(&self, path: &str) -> Result<(), ()>;
    fn stat(&self, path: &str) -> Result<FileStat, ()>;
    fn read_dir(&self, path: &str) -> Result<Vec<DirEntry>, ()>;
}

#[derive(Clone, Copy)]
pub struct FileStat {
    pub size: u64,
    pub mode: u16,
    pub uid: u32,
    pub gid: u32,
    pub is_dir: bool,
}

pub struct DirEntry {
    pub name: [u8; 256],
    pub name_len: usize,
    pub inode: u32,
}
