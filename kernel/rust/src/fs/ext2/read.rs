use crate::fs::ext2::{directory, inode, block};
use alloc::vec::Vec;

pub fn read_file(device: usize, path: &str) -> Result<Vec<u8>, ()> {
    let inode_num = directory::lookup(device, path)?;
    let inode_data = inode::read_inode(device, inode_num)?;
    
    if (inode_data.mode & 0x4000) != 0 {
        return Err(());
    }
    
    block::read_inode_data(device, &inode_data)
}
