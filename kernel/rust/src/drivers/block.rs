use spin::Mutex;
use alloc::vec::Vec;

pub trait BlockDevice: Send + Sync {
    fn read(&self, sector: u64, count: usize, buffer: &mut [u8]) -> Result<(), ()>;
    fn write(&self, sector: u64, count: usize, buffer: &[u8]) -> Result<(), ()>;
    fn sector_size(&self) -> usize;
}

static DEVICES: Mutex<Vec<&'static dyn BlockDevice>> = Mutex::new(Vec::new());

pub fn init() {}

pub fn register(device: &'static dyn BlockDevice) {
    DEVICES.lock().push(device);
}

pub fn get_device(index: usize) -> Option<&'static dyn BlockDevice> {
    DEVICES.lock().get(index).copied()
}

pub fn read_sectors(device: usize, sector: u64, count: usize, buffer: &mut [u8]) -> Result<(), ()> {
    let dev = get_device(device).ok_or(())?;
    dev.read(sector, count, buffer)
}

pub fn write_sectors(device: usize, sector: u64, count: usize, buffer: &[u8]) -> Result<(), ()> {
    let dev = get_device(device).ok_or(())?;
    dev.write(sector, count, buffer)
}
