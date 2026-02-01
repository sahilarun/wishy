pub mod superblock;
pub mod inode;
pub mod directory;
pub mod block;
pub mod read;
pub mod write;
pub mod bitmap;
pub mod journal;
pub mod permissions;

use crate::fs::{Filesystem, FileStat, DirEntry};
use alloc::vec::Vec;
use spin::Mutex;

pub struct Ext2Filesystem {
    device: usize,
    superblock: Mutex<superblock::Superblock>,
}

impl Ext2Filesystem {
    pub fn new(device: usize) -> Self {
        let sb = superblock::read_superblock(device);
        Self {
            device,
            superblock: Mutex::new(sb),
        }
    }
}

impl Filesystem for Ext2Filesystem {
    fn read_file(&self, path: &str) -> Result<Vec<u8>, ()> {
        read::read_file(self.device, path)
    }
    
    fn write_file(&self, path: &str, data: &[u8]) -> Result<(), ()> {
        write::write_file(self.device, path, data)
    }
    
    fn create_file(&self, path: &str, mode: u16, uid: u32, gid: u32) -> Result<(), ()> {
        write::create_file(self.device, path, mode, uid, gid)
    }
    
    fn delete_file(&self, path: &str) -> Result<(), ()> {
        write::delete_file(self.device, path)
    }
    
    fn stat(&self, path: &str) -> Result<FileStat, ()> {
        let inode_num = directory::lookup(self.device, path)?;
        let inode = inode::read_inode(self.device, inode_num)?;
        
        Ok(FileStat {
            size: inode.size as u64,
            mode: inode.mode,
            uid: inode.uid,
            gid: inode.gid,
            is_dir: (inode.mode & 0x4000) != 0,
        })
    }
    
    fn read_dir(&self, path: &str) -> Result<Vec<DirEntry>, ()> {
        directory::read_directory(self.device, path)
    }
}
