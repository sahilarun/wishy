use crate::fs::{Filesystem, FileStat, DirEntry};
use crate::fs::ext2::Ext2Filesystem;
use spin::Mutex;
use alloc::vec::Vec;
use alloc::boxed::Box;

static ROOT_FS: Mutex<Option<Box<dyn Filesystem>>> = Mutex::new(None);

pub fn mount_root() {
    let fs = Ext2Filesystem::new(0);
    *ROOT_FS.lock() = Some(Box::new(fs));
}

pub fn read_file(path: &str) -> Result<Vec<u8>, ()> {
    ROOT_FS.lock().as_ref().ok_or(())?.read_file(path)
}

pub fn write_file(path: &str, data: &[u8]) -> Result<(), ()> {
    ROOT_FS.lock().as_ref().ok_or(())?.write_file(path, data)
}

pub fn create_file(path: &str, mode: u16, uid: u32, gid: u32) -> Result<(), ()> {
    ROOT_FS.lock().as_ref().ok_or(())?.create_file(path, mode, uid, gid)
}

pub fn delete_file(path: &str) -> Result<(), ()> {
    ROOT_FS.lock().as_ref().ok_or(())?.delete_file(path)
}

pub fn stat(path: &str) -> Result<FileStat, ()> {
    ROOT_FS.lock().as_ref().ok_or(())?.stat(path)
}

pub fn read_dir(path: &str) -> Result<Vec<DirEntry>, ()> {
    ROOT_FS.lock().as_ref().ok_or(())?.read_dir(path)
}