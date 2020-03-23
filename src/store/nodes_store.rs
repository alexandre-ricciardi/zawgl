use std::fs::File;
use std::io::prelude::*;
use super::records::*;
use std::io::SeekFrom;

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
    fn read_at(&mut self, id: u32 , mut data:&mut [u8]) -> std::io::Result<()> {
        self.node_records_file.seek(SeekFrom::Start(id as u64))?;
        self.node_records_file.read_exact(&mut data)?;
        Ok(())
    }
    pub fn retrieve(&mut self, node_id: u32) -> Option<NodeRecord> {
        let mut data = [0u8; 9];
        let res = self.read_at(node_id, &mut data);
        res.ok().map(|_|nr_from_bytes(data))
    }
}

#[cfg(test)]
mod test_nodes_store {
    use super::*;
    #[test]
    fn test_create_node_0() {
        let mut store = NodesStore::new("C:\\Temp\\nodes.db");
        let nr = NodeRecord {
            in_use: true,
            next_rel_id: 1,
            next_prop_id: 8,
        };
        store.save(nr);

    }
}