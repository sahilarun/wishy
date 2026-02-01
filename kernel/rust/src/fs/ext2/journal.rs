use alloc::vec::Vec;
use spin::Mutex;

struct JournalEntry {
    device: usize,
    sector: u64,
    data: [u8; 512],
}

static JOURNAL: Mutex<Vec<JournalEntry>> = Mutex::new(Vec::new());

pub fn begin_transaction() {
    JOURNAL.lock().clear();
}

pub fn log_write(device: usize, sector: u64, data: &[u8; 512]) {
    JOURNAL.lock().push(JournalEntry {
        device,
        sector,
        data: *data,
    });
}

pub fn commit_transaction() {
    let journal = JOURNAL.lock();
    for entry in journal.iter() {
        let _ = crate::drivers::block::write_sectors(
            entry.device,
            entry.sector,
            1,
            &entry.data
        );
    }
}

pub fn rollback_transaction() {
    JOURNAL.lock().clear();
}
