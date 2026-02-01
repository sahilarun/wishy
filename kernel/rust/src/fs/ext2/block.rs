use crate::fs::cache;
use crate::fs::ext2::inode::Inode;
use alloc::vec::Vec;

pub fn read_inode_data(device: usize, inode: &Inode) -> Result<Vec<u8>, ()> {
    let mut data = Vec::with_capacity(inode.size as usize);
    let block_size = 1024;
    let blocks_needed = ((inode.size + block_size - 1) / block_size) as usize;
    
    for i in 0..blocks_needed {
        let block_num = get_block_number(device, inode, i)?;
        if block_num == 0 {
            break;
        }
        
        let sectors = (block_size / 512) as usize;
        for s in 0..sectors {
            let mut buffer = [0u8; 512];
            cache::read_block(device, (block_num as u64 * 2) + s as u64, &mut buffer)?;
            
            let remaining = (inode.size as usize).saturating_sub(data.len());
            let to_copy = remaining.min(512);
            data.extend_from_slice(&buffer[..to_copy]);
        }
    }
    
    Ok(data)
}

pub fn write_inode_data(device: usize, inode: &Inode, data: &[u8]) -> Result<(), ()> {
    let block_size = 1024;
    let blocks_needed = ((data.len() + block_size - 1) / block_size) as usize;
    
    for i in 0..blocks_needed {
        let block_num = get_block_number(device, inode, i)?;
        if block_num == 0 {
            return Err(());
        }
        
        let start = i * block_size;
        let end = ((i + 1) * block_size).min(data.len());
        let block_data = &data[start..end];
        
        let sectors = (block_size / 512) as usize;
        for s in 0..sectors {
            let mut buffer = [0u8; 512];
            let data_start = s * 512;
            let data_end = (data_start + 512).min(block_data.len());
            
            if data_start < block_data.len() {
                let copy_len = data_end - data_start;
                buffer[..copy_len].copy_from_slice(&block_data[data_start..data_end]);
            }
            
            cache::write_block(device, (block_num as u64 * 2) + s as u64, &buffer)?;
        }
    }
    
    Ok(())
}

fn get_block_number(device: usize, inode: &Inode, index: usize) -> Result<u32, ()> {
    if index < 12 {
        return Ok(inode.block[index]);
    }
    
    let index = index - 12;
    let entries_per_block = 256;
    
    if index < entries_per_block {
        return read_indirect_block(device, inode.block[12], index);
    }
    
    let index = index - entries_per_block;
    if index < entries_per_block * entries_per_block {
        let first_level = index / entries_per_block;
        let second_level = index % entries_per_block;
        let first_block = read_indirect_block(device, inode.block[13], first_level)?;
        return read_indirect_block(device, first_block, second_level);
    }
    
    Err(())
}

fn read_indirect_block(device: usize, block: u32, index: usize) -> Result<u32, ()> {
    if block == 0 {
        return Err(());
    }
    
    let mut buffer = [0u8; 512];
    cache::read_block(device, (block as u64 * 2) + (index / 128) as u64, &mut buffer)?;
    
    let offset = (index % 128) * 4;
    Ok(u32::from_le_bytes([
        buffer[offset],
        buffer[offset + 1],
        buffer[offset + 2],
        buffer[offset + 3],
    ]))
}
