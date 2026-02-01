use crate::fs::DirEntry;
use crate::fs::ext2::{inode, block};
use alloc::vec::Vec;

const ROOT_INODE: u32 = 2;

pub fn lookup(device: usize, path: &str) -> Result<u32, ()> {
    if path == "/" {
        return Ok(ROOT_INODE);
    }
    
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    let mut current_inode = ROOT_INODE;
    
    for part in parts {
        current_inode = find_in_directory(device, current_inode, part)?;
    }
    
    Ok(current_inode)
}

pub fn read_directory(device: usize, path: &str) -> Result<Vec<DirEntry>, ()> {
    let inode_num = lookup(device, path)?;
    let inode_data = inode::read_inode(device, inode_num)?;
    let data = block::read_inode_data(device, &inode_data)?;
    
    let mut entries = Vec::new();
    let mut offset = 0;
    
    while offset < data.len() {
        if offset + 8 > data.len() {
            break;
        }
        
        let inode = u32::from_le_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]);
        let rec_len = u16::from_le_bytes([data[offset+4], data[offset+5]]) as usize;
        let name_len = data[offset+6] as usize;
        
        if inode == 0 || rec_len == 0 {
            break;
        }
        
        if name_len > 0 && offset + 8 + name_len <= data.len() {
            let mut name = [0u8; 256];
            name[..name_len].copy_from_slice(&data[offset+8..offset+8+name_len]);
            
            entries.push(DirEntry {
                name,
                name_len,
                inode,
            });
        }
        
        offset += rec_len;
    }
    
    Ok(entries)
}

fn find_in_directory(device: usize, dir_inode: u32, name: &str) -> Result<u32, ()> {
    let inode_data = inode::read_inode(device, dir_inode)?;
    let data = block::read_inode_data(device, &inode_data)?;
    
    let mut offset = 0;
    
    while offset < data.len() {
        if offset + 8 > data.len() {
            break;
        }
        
        let inode = u32::from_le_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]);
        let rec_len = u16::from_le_bytes([data[offset+4], data[offset+5]]) as usize;
        let name_len = data[offset+6] as usize;
        
        if inode == 0 || rec_len == 0 {
            break;
        }
        
        if name_len > 0 && offset + 8 + name_len <= data.len() {
            let entry_name = &data[offset+8..offset+8+name_len];
            if entry_name == name.as_bytes() {
                return Ok(inode);
            }
        }
        
        offset += rec_len;
    }
    
    Err(())
}

pub fn add_entry(device: usize, dir_inode: u32, name: &str, inode: u32, file_type: u8) -> Result<(), ()> {
    let mut inode_data = inode::read_inode(device, dir_inode)?;
    let mut data = block::read_inode_data(device, &inode_data)?;
    
    let name_bytes = name.as_bytes();
    let name_len = name_bytes.len();
    let rec_len = ((8 + name_len + 3) / 4) * 4;
    
    let entry_start = data.len();
    data.resize(entry_start + rec_len, 0);
    
    data[entry_start..entry_start+4].copy_from_slice(&inode.to_le_bytes());
    data[entry_start+4..entry_start+6].copy_from_slice(&(rec_len as u16).to_le_bytes());
    data[entry_start+6] = name_len as u8;
    data[entry_start+7] = file_type;
    data[entry_start+8..entry_start+8+name_len].copy_from_slice(name_bytes);
    
    inode_data.size = data.len() as u32;
    block::write_inode_data(device, &inode_data, &data)?;
    inode::write_inode(device, dir_inode, &inode_data)?;
    
    Ok(())
          }
