use crate::fs::ext2::{directory, inode, block, bitmap};

pub fn write_file(device: usize, path: &str, data: &[u8]) -> Result<(), ()> {
    let inode_num = directory::lookup(device, path)?;
    let mut inode_data = inode::read_inode(device, inode_num)?;
    
    if (inode_data.mode & 0x4000) != 0 {
        return Err(());
    }
    
    let blocks_needed = ((data.len() + 1023) / 1024) as usize;
    let current_blocks = ((inode_data.size + 1023) / 1024) as usize;
    
    if blocks_needed > current_blocks {
        allocate_blocks(device, &mut inode_data, blocks_needed - current_blocks)?;
    }
    
    inode_data.size = data.len() as u32;
    block::write_inode_data(device, &inode_data, data)?;
    inode::write_inode(device, inode_num, &inode_data)?;
    
    Ok(())
}

pub fn create_file(device: usize, path: &str, mode: u16, uid: u32, gid: u32) -> Result<(), ()> {
    let parts: alloc::vec::Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.is_empty() {
        return Err(());
    }
    
    let filename = parts[parts.len() - 1];
    let parent_path = if parts.len() > 1 {
        alloc::format!("/{}", parts[..parts.len()-1].join("/"))
    } else {
        alloc::string::String::from("/")
    };
    
    let parent_inode = directory::lookup(device, &parent_path)?;
    let new_inode = bitmap::allocate_inode(device)?;
    
    let mut inode_data = inode::Inode {
        mode,
        uid,
        size: 0,
        atime: 0,
        ctime: 0,
        mtime: 0,
        dtime: 0,
        gid,
        links_count: 1,
        blocks: 0,
        flags: 0,
        osd1: 0,
        block: [0; 15],
        generation: 0,
        file_acl: 0,
        dir_acl: 0,
        faddr: 0,
        osd2: [0; 12],
    };
    
    inode::write_inode(device, new_inode, &inode_data)?;
    directory::add_entry(device, parent_inode, filename, new_inode, 1)?;
    
    Ok(())
}

pub fn delete_file(device: usize, path: &str) -> Result<(), ()> {
    let inode_num = directory::lookup(device, path)?;
    bitmap::free_inode(device, inode_num)?;
    Ok(())
}

fn allocate_blocks(device: usize, inode: &mut inode::Inode, count: usize) -> Result<(), ()> {
    let current = ((inode.size + 1023) / 1024) as usize;
    
    for i in 0..count {
        let block_index = current + i;
        if block_index < 12 {
            inode.block[block_index] = bitmap::allocate_block(device)?;
        } else {
            return Err(());
        }
    }
    
    Ok(())
}
