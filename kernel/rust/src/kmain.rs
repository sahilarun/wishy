use crate::drivers::{ata, fb, block, initrd};
use crate::fs::{mount, cache};
use crate::gui::compositor;
use crate::memory::{paging, alloc};
use crate::exec::loader;
use crate::syscall;

extern "C" {
    fn load_idt(idt_ptr: *const u64);
    static isr_stub_table: [u32; 48];
}

pub fn kernel_main() {
    print_vga("Wishy OS v0.1.0", 0, 0x0F);
    print_vga("Kernel started successfully!", 1, 0x0A);
    print_vga("System is running...", 2, 0x0E);
    
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

fn print_vga(s: &str, row: usize, color: u8) {
    unsafe {
        let vga = 0xB8000 as *mut u16;
        for (i, byte) in s.bytes().enumerate() {
            *vga.add(row * 80 + i) = ((color as u16) << 8) | byte as u16;
        }
    }
}

fn setup_keyboard() {
    unsafe {
        core::arch::asm!("out 0x21, al", in("al") 0xFDu8);
        core::arch::asm!("out 0xA1, al", in("al") 0xFFu8);
    }
}

fn setup_idt() {
    static mut IDT: [u64; 256] = [0; 256];
    
    unsafe {
        for i in 0..48 {
            let offset = isr_stub_table[i];
            IDT[i] = ((offset as u64) & 0xFFFF) |
                     (0x08 << 16) |
                     (0x8E00 << 32) |
                     (((offset as u64) & 0xFFFF0000) << 32);
        }
        
        let idt_ptr: u64 = ((core::mem::size_of_val(&IDT) as u64 - 1) << 48) |
                           (IDT.as_ptr() as u64);
        load_idt(&idt_ptr);
        
        core::arch::asm!("sti");
    }
}

#[no_mangle]
pub extern "C" fn interrupt_handler(_frame: *const u8) {
}
