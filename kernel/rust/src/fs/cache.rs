use spin::Mutex;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

const CACHE_SIZE: usize = 256;

struct CacheEntry {
    device: usize,
    sector: u64,
    data: [u8; 512],
    dirty: bool,
}

static CACHE: Mutex<BlockCache> = Mutex::new(BlockCache::new());

struct BlockCache {
    entries: Vec<CacheEntry>,
    map: BTreeMap<(usize, u64), usize>,
}

impl BlockCache {
    const fn new() -> Self {
        Self {
            entries: Vec::new(),
            map: BTreeMap::new(),
        }
    }
    
    fn get(&mut self, device: usize, sector: u64) -> Option<&mut [u8; 512]> {
        if let Some(&idx) = self.map.get(&(device, sector)) {
            return Some(&mut self.entries[idx].data);
        }
        None
    }
    
    fn insert(&mut self, device: usize, sector: u64, data: &[u8; 512]) {
        if self.entries.len() >= CACHE_SIZE {
            self.evict();
        }
        
        let idx = self.entries.len();
        self.entries.push(CacheEntry {
            device,
            sector,
            data: *data,
            dirty: false,
        });
        self.map.insert((device, sector), idx);
    }
    
    fn mark_dirty(&mut self, device: usize, sector: u64) {
        if let Some(&idx) = self.map.get(&(device, sector)) {
            self.entries[idx].dirty = true;
        }
    }
    
    fn evict(&mut self) {
        if let Some(entry) = self.entries.first() {
            if entry.dirty {
                let _ = crate::drivers::block::write_sectors(
                    entry.device,
                    entry.sector,
                    1,
                    &entry.data
                );
            }
            self.map.remove(&(entry.device, entry.sector));
        }
        if !self.entries.is_empty() {
            self.entries.remove(0);
        }
    }
    
    fn flush(&mut self) {
        for entry in &self.entries {
            if entry.dirty {
                let _ = crate::drivers::block::write_sectors(
                    entry.device,
                    entry.sector,
                    1,
                    &entry.data
                );
            }
        }
    }
}

pub fn init() {}

pub fn read_block(device: usize, sector: u64, buffer: &mut [u8; 512]) -> Result<(), ()> {
    let mut cache = CACHE.lock();
    
    if let Some(data) = cache.get(device, sector) {
        buffer.copy_from_slice(data);
        return Ok(());
    }
    
    crate::drivers::block::read_sectors(device, sector, 1, buffer)?;
    cache.insert(device, sector, buffer);
    Ok(())
}

pub fn write_block(device: usize, sector: u64, buffer: &[u8; 512]) -> Result<(), ()> {
    let mut cache = CACHE.lock();
    
    if let Some(data) = cache.get(device, sector) {
        data.copy_from_slice(buffer);
        cache.mark_dirty(device, sector);
        return Ok(());
    }
    
    cache.insert(device, sector, buffer);
    cache.mark_dirty(device, sector);
    Ok(())
}

pub fn flush() {
    CACHE.lock().flush();
}
