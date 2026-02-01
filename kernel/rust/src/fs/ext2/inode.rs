use crate::fs::cache;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Inode {
    pub mode: u16,
    pub uid: u32,
    pub size: u32,
    pub atime: u32,
    pub ctime: u32,
    pub mtime: u32,
    pub dtime: u32,
    pub gid: u32,
    pub links_count: u16,
    pub blocks: u32,
    pub flags: u32,
    pub osd1: u32,
    pub block: [u32; 15],
    pub generation: u32,
    pub file_acl: u32,
    pub dir_acl: u32,
    pub faddr: u32,
    pub osd2: [u8; 12],
}

pub fn read_inode(device: usize, inode_num: u32) -> Result<Inode, ()> {
    let sb = super::superblock::read_superblock(device);
    let inodes_per_group = sb.inodes_per_group();
    let inode_size = 128;
    
    let group = (inode_num - 1) / inodes_per_group;
    let index = (inode_num - 1) % inodes_per_group;
    
    let inode_table_block = get_inode_table(device, group);
    let block_offset = (index * inode_size) / 512;
    let byte_offset = ((index * inode_size) % 512) as usize;
    
    let mut buffer = [0u8; 512];
    cache::read_block(device, (inode_table_block + block_offset) as u64, &mut buffer)?;
    
    unsafe {
        Ok(core::ptr::read((buffer.as_ptr().add(byte_offset)) as *const Inode))
    }
}

pub fn write_inode(device: usize, inode_num: u32, inode: &Inode) -> Result<(), ()> {
    let sb = super::superblock::read_superblock(device);
    let inodes_per_group = sb.inodes_per_group();
    let inode_size = 128;
    
    let group = (inode_num - 1) / inodes_per_group;
    let index = (inode_num - 1) % inodes_per_group;
    
    let inode_table_block = get_inode_table(device, group);
    let block_offset = (index * inode_size) / 512;
    let byte_offset = ((index * inode_size) % 512) as usize;
    
    let mut buffer = [0u8; 512];
    cache::read_block(device, (inode_table_block + block_offset) as u64, &mut buffer)?;
    
    unsafe {
        core::ptr::copy_nonoverlapping(
            inode as *const Inode as *const u8,
            buffer.as_mut_ptr().add(byte_offset),
            core::mem::size_of::<Inode>()
        );
    }
    
    cache::write_block(device, (inode_table_block + block_offset) as u64, &buffer)?;
    Ok(())
}

fn get_inode_table(device: usize, group: u32) -> u32 {
    let mut buffer = [0u8; 512];
    let bgd_block = 4 + (group / 16);
    cache::read_block(device, bgd_block as u64, &mut buffer).expect("Failed to read BGD");
    
    let offset = ((group % 16) * 32) as usize;
    let inode_table = u32::from_le_bytes([
        buffer[offset + 8],
        buffer[offset + 9],
        buffer[offset + 10],
        buffer[offset + 11],
    ]);
    
    inode_table
}
