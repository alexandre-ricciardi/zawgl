use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use log::error;

pub struct FileAccess {
    file: File,
    file_name: String
}

impl FileAccess {
    pub fn new(file: &str) -> Self {
        let mut f = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file)
            .expect("Cannot open file");
        FileAccess {file: f, file_name: file.to_owned()}
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
    fn _read_at(&mut self, pos: u64 , mut data:&mut [u8]) -> std::io::Result<()> {
        self.file.seek(SeekFrom::Start(pos))?;
        self.file.read_exact(&mut data)?;
        Ok(())
    }

    pub fn write_at(&mut self, pos: u64, data: &[u8]) {
        match self._write_at(pos, data) {
            Err(msg) => {
                error!("writing file {}", self.file_name);
            },
            _ => {}
        }
    }
    pub fn read_at(&mut self, pos: u64 , mut data:&mut [u8]) {
        match self._read_at(pos, data) {
            Err(msg) => {
                error!("reading file {}: {}", self.file_name, msg);
            },
            _ => {}
        }
    }
    pub fn get_file_len(&self) -> u64 {
        match self.file.metadata() {
            Err(msg) => {
                error!("retrieving file size {}", self.file_name);
                0
            },
            Ok(md) => {md.len()}
        }
    }
}