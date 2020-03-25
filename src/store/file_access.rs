use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
pub struct FileAccess {
    file: File,
}

impl FileAccess {
    pub fn new(file: &str) -> Self {
        let mut f = File::create(file).expect("Cannot open file");
        FileAccess {file: f}
    }
    fn write_at(&mut self, pos: u64, data: &[u8]) -> std::io::Result<()> {
        let mut written = 0;
        self.file.seek(SeekFrom::Start(pos))?;
        while written < data.len() {
            let bytes_written = self.file.write(&data[written..])?;
            written += bytes_written;
        }
        Ok(())
    }
    fn read_at(&mut self, pos: u64 , mut data:&mut [u8]) -> std::io::Result<()> {
        self.file.seek(SeekFrom::Start(pos))?;
        self.file.read_exact(&mut data)?;
        Ok(())
    }
}