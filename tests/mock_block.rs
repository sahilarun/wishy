use std::collections::HashMap;
use std::sync::Mutex;

pub struct MockBlockDevice {
    sectors: Mutex<HashMap<u64, [u8; 512]>>,
}

impl MockBlockDevice {
    pub fn new() -> Self {
        Self {
            sectors: Mutex::new(HashMap::new()),
        }
    }
    
    pub fn read(&self, sector: u64, count: usize, buffer: &mut [u8]) -> Result<(), ()> {
        let sectors = self.sectors.lock().unwrap();
        
        for i in 0..count {
            let sector_num = sector + i as u64;
            if let Some(data) = sectors.get(&sector_num) {
                buffer[i * 512..(i + 1) * 512].copy_from_slice(data);
            } else {
                buffer[i * 512..(i + 1) * 512].fill(0);
            }
        }
        
        Ok(())
    }
    
    pub fn write(&self, sector: u64, count: usize, buffer: &[u8]) -> Result<(), ()> {
        let mut sectors = self.sectors.lock().unwrap();
        
        for i in 0..count {
            let sector_num = sector + i as u64;
            let mut data = [0u8; 512];
            data.copy_from_slice(&buffer[i * 512..(i + 1) * 512]);
            sectors.insert(sector_num, data);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_read_write() {
        let device = MockBlockDevice::new();
        let data = [0x42u8; 512];
        
        device.write(0, 1, &data).unwrap();
        
        let mut buffer = [0u8; 512];
        device.read(0, 1, &mut buffer).unwrap();
        
        assert_eq!(buffer, data);
    }
      }
