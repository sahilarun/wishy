use crate::drivers::block::BlockDevice;
use spin::Mutex;

const INITRD_START: usize = 0x400000;
const INITRD_SIZE: usize = 0x100000;

struct InitrdDevice {
    data: &'static [u8],
}

static INITRD: Mutex<Option<InitrdDevice>> = Mutex::new(None);

pub fn init() {
    let data = unsafe {
        core::slice::from_raw_parts(INITRD_START as *const u8, INITRD_SIZE)
    };
    
    *INITRD.lock() = Some(InitrdDevice { data });
    crate::drivers::block::register(&InitrdBlockDevice);
}

struct InitrdBlockDevice;

impl BlockDevice for InitrdBlockDevice {
    fn read(&self, sector: u64, count: usize, buffer: &mut [u8]) -> Result<(), ()> {
        let device = INITRD.lock();
        let dev = device.as_ref().ok_or(())?;
        
        let start = (sector as usize) * 512;
        let end = start + (count * 512);
        
        if end > dev.data.len() {
            return Err(());
        }
        
        buffer[..count * 512].copy_from_slice(&dev.data[start..end]);
        Ok(())
    }
    
    fn write(&self, _sector: u64, _count: usize, _buffer: &[u8]) -> Result<(), ()> {
        Err(())
    }
    
    fn sector_size(&self) -> usize {
        512
    }
}
