use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use log::error;

pub struct FileAccess {
    file: File
}

impl FileAccess {
    pub fn new(file: &str) -> Self {
        let f = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file)
            .expect("Cannot open file");
        FileAccess {file: f}
    }
    fn _write_at(&mut self, pos: u64, data: &[u8]) -> std::io::Result<()> {
        let mut written = 0;
        self.file.seek(SeekFrom::Start(pos))?;
        while written < data.len() {
            let bytes_written = self.file.write(&data[written..])?;
            written += bytes_written;
        }
        self.file.sync_all()?;
        Ok(())
    }
    fn _read_at(&mut self, pos: u64 , mut data: &mut [u8]) -> std::io::Result<()> {
        self.file.seek(SeekFrom::Start(pos))?;
        self.file.read_exact(&mut data)?;
        Ok(())
    }

    pub fn write_at(&mut self, pos: u64, data: &[u8]) {
        match self._write_at(pos, data) {
            Err(_msg) => {
                error!("writing file");
            },
            _ => {}
        }
    }
    pub fn read_at(&mut self, pos: u64 , data: &mut [u8]) {
        match self._read_at(pos, data) {
            Err(msg) => {
                error!("reading file {}", msg);
            },
            _ => {}
        }
    }
    pub fn get_file_len(&self) -> u64 {
        match self.file.metadata() {
            Err(_msg) => {
                error!("retrieving file size");
                0
            },
            Ok(md) => {md.len()}
        }
    }
}