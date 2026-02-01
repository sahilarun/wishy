use crate::drivers::block::BlockDevice;
use spin::Mutex;

const ATA_PRIMARY_IO: u16 = 0x1F0;
const ATA_PRIMARY_CTRL: u16 = 0x3F6;

struct AtaDrive {
    base: u16,
    ctrl: u16,
}

static PRIMARY_DRIVE: Mutex<AtaDrive> = Mutex::new(AtaDrive {
    base: ATA_PRIMARY_IO,
    ctrl: ATA_PRIMARY_CTRL,
});

pub fn init() {
    crate::drivers::block::register(&AtaBlockDevice);
}

impl AtaDrive {
    fn wait_ready(&self) {
        while (inb(self.base + 7) & 0x80) != 0 {}
        while (inb(self.base + 7) & 0x08) == 0 {}
    }
    
    fn read_sector(&mut self, lba: u64, buffer: &mut [u8]) {
        self.wait_ready();
        
        outb(self.base + 6, 0xE0 | ((lba >> 24) & 0x0F) as u8);
        outb(self.base + 2, 1);
        outb(self.base + 3, (lba & 0xFF) as u8);
        outb(self.base + 4, ((lba >> 8) & 0xFF) as u8);
        outb(self.base + 5, ((lba >> 16) & 0xFF) as u8);
        outb(self.base + 7, 0x20);
        
        self.wait_ready();
        
        for i in (0..512).step_by(2) {
            let data = inw(self.base);
            buffer[i] = (data & 0xFF) as u8;
            buffer[i + 1] = ((data >> 8) & 0xFF) as u8;
        }
    }
    
    fn write_sector(&mut self, lba: u64, buffer: &[u8]) {
        self.wait_ready();
        
        outb(self.base + 6, 0xE0 | ((lba >> 24) & 0x0F) as u8);
        outb(self.base + 2, 1);
        outb(self.base + 3, (lba & 0xFF) as u8);
        outb(self.base + 4, ((lba >> 8) & 0xFF) as u8);
        outb(self.base + 5, ((lba >> 16) & 0xFF) as u8);
        outb(self.base + 7, 0x30);
        
        self.wait_ready();
        
        for i in (0..512).step_by(2) {
            let data = (buffer[i] as u16) | ((buffer[i + 1] as u16) << 8);
            outw(self.base, data);
        }
        
        outb(self.base + 7, 0xE7);
        self.wait_ready();
    }
}

struct AtaBlockDevice;

impl BlockDevice for AtaBlockDevice {
    fn read(&self, sector: u64, count: usize, buffer: &mut [u8]) -> Result<(), ()> {
        let mut drive = PRIMARY_DRIVE.lock();
        for i in 0..count {
            drive.read_sector(sector + i as u64, &mut buffer[i * 512..(i + 1) * 512]);
        }
        Ok(())
    }
    
    fn write(&self, sector: u64, count: usize, buffer: &[u8]) -> Result<(), ()> {
        let mut drive = PRIMARY_DRIVE.lock();
        for i in 0..count {
            drive.write_sector(sector + i as u64, &buffer[i * 512..(i + 1) * 512]);
        }
        Ok(())
    }
    
    fn sector_size(&self) -> usize {
        512
    }
}

fn inb(port: u16) -> u8 {
    unsafe {
        let result: u8;
        core::arch::asm!("in al, dx", out("al") result, in("dx") port);
        result
    }
}

fn inw(port: u16) -> u16 {
    unsafe {
        let result: u16;
        core::arch::asm!("in ax, dx", out("ax") result, in("dx") port);
        result
    }
}

fn outb(port: u16, value: u8) {
    unsafe {
        core::arch::asm!("out dx, al", in("dx") port, in("al") value);
    }
}

fn outw(port: u16, value: u16) {
    unsafe {
        core::arch::asm!("out dx, ax", in("dx") port, in("ax") value);
    }
      }
