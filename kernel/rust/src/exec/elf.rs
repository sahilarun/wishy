#[repr(C)]
pub struct ElfHeader {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u32,
    pub e_phoff: u32,
    pub e_shoff: u32,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[repr(C)]
pub struct ProgramHeader {
    pub p_type: u32,
    pub p_offset: u32,
    pub p_vaddr: u32,
    pub p_paddr: u32,
    pub p_filesz: u32,
    pub p_memsz: u32,
    pub p_flags: u32,
    pub p_align: u32,
}

pub fn parse_elf(data: &[u8]) -> Result<(u32, alloc::vec::Vec<ProgramHeader>), ()> {
    if data.len() < core::mem::size_of::<ElfHeader>() {
        return Err(());
    }
    
    let header = unsafe {
        &*(data.as_ptr() as *const ElfHeader)
    };
    
    if &header.e_ident[0..4] != b"\x7FELF" {
        return Err(());
    }
    
    let mut program_headers = alloc::vec::Vec::new();
    
    for i in 0..header.e_phnum {
        let offset = (header.e_phoff as usize) + (i as usize * header.e_phentsize as usize);
        if offset + core::mem::size_of::<ProgramHeader>() > data.len() {
            break;
        }
        
        let ph = unsafe {
            core::ptr::read((data.as_ptr().add(offset)) as *const ProgramHeader)
        };
        
        program_headers.push(ph);
    }
    
    Ok((header.e_entry, program_headers))
}
