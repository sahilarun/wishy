use spin::Mutex;

const PAGE_SIZE: usize = 4096;
const PAGE_DIRECTORY: usize = 0x100000;
const PAGE_TABLES: usize = 0x101000;

static PAGING: Mutex<()> = Mutex::new(());

pub fn init() {
    let _lock = PAGING.lock();
    
    let page_dir = PAGE_DIRECTORY as *mut u32;
    let page_tables = PAGE_TABLES as *mut u32;
    
    unsafe {
        for i in 0..1024 {
            core::ptr::write_volatile(page_dir.add(i), 0);
        }
        
        for i in 0..256 {
            core::ptr::write_volatile(
                page_dir.add(i),
                (PAGE_TABLES + i * PAGE_SIZE) as u32 | 0x3
            );
            
            let table = page_tables.add(i * 1024);
            for j in 0..1024 {
                let phys_addr = (i * 1024 + j) * PAGE_SIZE;
                core::ptr::write_volatile(table.add(j), phys_addr as u32 | 0x3);
            }
        }
        
        core::arch::asm!(
            "mov cr3, eax",
            "mov eax, cr0",
            "or eax, 0x80000000",
            "mov cr0, eax",
            in("eax") PAGE_DIRECTORY
        );
    }
}

pub fn map_page(virt: usize, phys: usize, flags: u32) {
    let _lock = PAGING.lock();
    
    let pd_index = (virt >> 22) & 0x3FF;
    let pt_index = (virt >> 12) & 0x3FF;
    
    let page_dir = PAGE_DIRECTORY as *mut u32;
    let pd_entry = unsafe { core::ptr::read_volatile(page_dir.add(pd_index)) };
    
    let page_table = if (pd_entry & 0x1) == 0 {
        let new_table = allocate_page_table();
        unsafe {
            core::ptr::write_volatile(page_dir.add(pd_index), new_table as u32 | flags | 0x1);
        }
        new_table as *mut u32
    } else {
        (pd_entry & !0xFFF) as *mut u32
    };
    
    unsafe {
        core::ptr::write_volatile(page_table.add(pt_index), phys as u32 | flags | 0x1);
    }
}

fn allocate_page_table() -> usize {
    static mut NEXT_TABLE: usize = PAGE_TABLES + 256 * PAGE_SIZE;
    unsafe {
        let addr = NEXT_TABLE;
        NEXT_TABLE += PAGE_SIZE;
        
        let ptr = addr as *mut u8;
        for i in 0..PAGE_SIZE {
            core::ptr::write_volatile(ptr.add(i), 0);
        }
        
        addr
    }
}
