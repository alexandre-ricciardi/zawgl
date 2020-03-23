use std::fs::File;
use std::io::prelude::*;
use super::records::*;

pub struct NodesStore {
    node_records_file: File
}

impl NodesStore {
    pub fn new(file: &str) -> Self {
        let mut f = File::create(file).expect("Cannot open the nodes store file");
        NodesStore { node_records_file: f }
    }
    pub fn save(&mut self, node: NodeRecord) -> std::io::Result<()> {
        let mut pos = 0;
        let data = nr_to_bytes(node);
        while pos < data.len() {
            let bytes_written = self.node_records_file.write(&data[pos..])?;
            pos += bytes_written;
        }
        Ok(())
    }
}