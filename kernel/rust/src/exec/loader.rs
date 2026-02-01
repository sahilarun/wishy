use crate::exec::elf;
use crate::fs::mount;
use crate::memory::paging;

pub fn load_init() {
    if let Ok(data) = mount::read_file("/sbin/init") {
        let _ = load_elf(&data);
    }
}

pub fn load_elf(data: &[u8]) -> Result<u32, ()> {
    let (entry, program_headers) = elf::parse_elf(data)?;
    
    for ph in program_headers {
        if ph.p_type == 1 {
            load_segment(data, &ph)?;
        }
    }
    
    Ok(entry)
}

fn load_segment(data: &[u8], ph: &elf::ProgramHeader) -> Result<(), ()> {
    let vaddr = ph.p_vaddr as usize;
    let filesz = ph.p_filesz as usize;
    let memsz = ph.p_memsz as usize;
    let offset = ph.p_offset as usize;
    
    let page_aligned = vaddr & !0xFFF;
    let page_offset = vaddr & 0xFFF;
    let pages_needed = ((page_offset + memsz) + 4095) / 4096;
    
    for i in 0..pages_needed {
        let phys = allocate_page();
        paging::map_page(page_aligned + i * 4096, phys, 0x7);
    }
    
    let dest = vaddr as *mut u8;
    let src = &data[offset..offset + filesz];
    
    unsafe {
        core::ptr::copy_nonoverlapping(src.as_ptr(), dest, filesz);
        
        if memsz > filesz {
            core::ptr::write_bytes(dest.add(filesz), 0, memsz - filesz);
        }
    }
    
    Ok(())
}

fn allocate_page() -> usize {
    static mut NEXT_PAGE: usize = 0x2000000;
    unsafe {
        let page = NEXT_PAGE;
        NEXT_PAGE += 4096;
        page
    }
  }
