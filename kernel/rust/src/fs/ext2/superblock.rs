use crate::fs::cache;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Superblock {
    pub inodes_count: u32,
    pub blocks_count: u32,
    pub reserved_blocks: u32,
    pub free_blocks: u32,
    pub free_inodes: u32,
    pub first_data_block: u32,
    pub log_block_size: u32,
    pub log_frag_size: u32,
    pub blocks_per_group: u32,
    pub frags_per_group: u32,
    pub inodes_per_group: u32,
    pub mtime: u32,
    pub wtime: u32,
    pub mnt_count: u16,
    pub max_mnt_count: u16,
    pub magic: u16,
    pub state: u16,
    pub errors: u16,
    pub minor_rev: u16,
    pub lastcheck: u32,
    pub checkinterval: u32,
    pub creator_os: u32,
    pub rev_level: u32,
    pub def_resuid: u16,
    pub def_resgid: u16,
}

pub fn read_superblock(device: usize) -> Superblock {
    let mut buffer = [0u8; 512];
    cache::read_block(device, 2, &mut buffer).expect("Failed to read superblock");
    
    unsafe { core::ptr::read(buffer.as_ptr() as *const Superblock) }
}

pub fn write_superblock(device: usize, sb: &Superblock) {
    let mut buffer = [0u8; 512];
    unsafe {
        core::ptr::copy_nonoverlapping(
            sb as *const Superblock as *const u8,
            buffer.as_mut_ptr(),
            core::mem::size_of::<Superblock>()
        );
    }
    cache::write_block(device, 2, &buffer).expect("Failed to write superblock");
}

impl Superblock {
    pub fn block_size(&self) -> usize {
        1024 << self.log_block_size
    }
    
    pub fn blocks_per_group(&self) -> u32 {
        self.blocks_per_group
    }
    
    pub fn inodes_per_group(&self) -> u32 {
        self.inodes_per_group
    }
          }
