use alloc::collections::BTreeMap;
use spin::Mutex;

static SHM_POOLS: Mutex<BTreeMap<i32, ShmPool>> = Mutex::new(BTreeMap::new());

struct ShmPool {
    fd: i32,
    size: usize,
    mapping: usize,
}

pub fn create_pool(fd: i32, size: usize) -> Result<(), ()> {
    let mapping = allocate_shm_mapping(size);
    
    let mut pools = SHM_POOLS.lock();
    pools.insert(fd, ShmPool {
        fd,
        size,
        mapping,
    });
    
    Ok(())
}

pub fn get_pool_data(fd: i32, offset: usize) -> Option<usize> {
    let pools = SHM_POOLS.lock();
    pools.get(&fd).map(|pool| pool.mapping + offset)
}

pub fn destroy_pool(fd: i32) {
    let mut pools = SHM_POOLS.lock();
    pools.remove(&fd);
}

fn allocate_shm_mapping(size: usize) -> usize {
    static mut NEXT_MAPPING: usize = 0x70000000;
    unsafe {
        let addr = NEXT_MAPPING;
        NEXT_MAPPING += (size + 4095) & !4095;
        addr
    }
  }
