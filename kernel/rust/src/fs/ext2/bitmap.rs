use crate::fs::cache;
use crate::fs::ext2::superblock;

pub fn allocate_inode(device: usize) -> Result<u32, ()> {
    let sb = superblock::read_superblock(device);
    let groups = (sb.inodes_count + sb.inodes_per_group - 1) / sb.inodes_per_group;
    
    for group in 0..groups {
        let bitmap_block = get_inode_bitmap_block(device, group);
        let mut buffer = [0u8; 512];
        
        for sector in 0..2 {
            cache::read_block(device, bitmap_block as u64 + sector, &mut buffer)?;
            
            for byte_idx in 0..512 {
                let byte = buffer[byte_idx];
                if byte != 0xFF {
                    for bit in 0..8 {
                        if (byte & (1 << bit)) == 0 {
                            buffer[byte_idx] |= 1 << bit;
                            cache::write_block(device, bitmap_block as u64 + sector, &buffer)?;
                            
                            let inode = (group * sb.inodes_per_group) + 
                                       (sector as u32 * 512 * 8) + 
                                       (byte_idx as u32 * 8) + 
                                       bit as u32 + 1;
                            return Ok(inode);
                        }
                    }
                }
            }
        }
    }
    
    Err(())
}

pub fn free_inode(device: usize, inode: u32) -> Result<(), ()> {
    let sb = superblock::read_superblock(device);
    let group = (inode - 1) / sb.inodes_per_group;
    let index = (inode - 1) % sb.inodes_per_group;
    
    let bitmap_block = get_inode_bitmap_block(device, group);
    let sector = (index / (512 * 8)) as u64;
    let byte_idx = ((index / 8) % 512) as usize;
    let bit = (index % 8) as usize;
    
    let mut buffer = [0u8; 512];
    cache::read_block(device, bitmap_block as u64 + sector, &mut buffer)?;
    
    buffer[byte_idx] &= !(1 << bit);
    cache::write_block(device, bitmap_block as u64 + sector, &buffer)?;
    
    Ok(())
}

pub fn allocate_block(device: usize) -> Result<u32, ()> {
    let sb = superblock::read_superblock(device);
    let groups = (sb.blocks_count + sb.blocks_per_group - 1) / sb.blocks_per_group;
    
    for group in 0..groups {
        let bitmap_block = get_block_bitmap_block(device, group);
        let mut buffer = [0u8; 512];
        
        for sector in 0..2 {
            cache::read_block(device, bitmap_block as u64 + sector, &mut buffer)?;
            
            for byte_idx in 0..512 {
                let byte = buffer[byte_idx];
                if byte != 0xFF {
                    for bit in 0..8 {
                        if (byte & (1 << bit)) == 0 {
                            buffer[byte_idx] |= 1 << bit;
                            cache::write_block(device, bitmap_block as u64 + sector, &buffer)?;
                            
                            let block = (group * sb.blocks_per_group) + 
                                       (sector as u32 * 512 * 8) + 
                                       (byte_idx as u32 * 8) + 
                                       bit as u32;
                            return Ok(block);
                        }
                    }
                }
            }
        }
    }
    
    Err(())
}

fn get_inode_bitmap_block(device: usize, group: u32) -> u32 {
    let mut buffer = [0u8; 512];
    let bgd_block = 4 + (group / 16);
    cache::read_block(device, bgd_block as u64, &mut buffer).expect("Failed to read BGD");
    
    let offset = ((group % 16) * 32) as usize;
    u32::from_le_bytes([
        buffer[offset + 4],
        buffer[offset + 5],
        buffer[offset + 6],
        buffer[offset + 7],
    ])
}

fn get_block_bitmap_block(device: usize, group: u32) -> u32 {
    let mut buffer = [0u8; 512];
    let bgd_block = 4 + (group / 16);
    cache::read_block(device, bgd_block as u64, &mut buffer).expect("Failed to read BGD");
    
    let offset = ((group % 16) * 32) as usize;
    u32::from_le_bytes([
        buffer[offset],
        buffer[offset + 1],
        buffer[offset + 2],
        buffer[offset + 3],
    ])
}
