use std::fs::File;
use std::io::prelude::*;
use super::records::*;
use std::io::SeekFrom;
use std::collections::HashMap;

pub struct NodesStore {
    node_records_file: File,
    in_use_nodes: Vec<bool>,
    not_in_use_map: HashMap<u32, bool>,
}

impl NodesStore {
    pub fn new(file: &str) -> Self {
        let mut f = File::create(file).expect("Cannot open the nodes store file");
        let mut store = NodesStore { node_records_file: f, in_use_nodes: Vec::new(), in_use_map: HashMap::new() };
        store.scan();
        store
    }
    pub fn save(&mut self, node: NodeRecord) -> std::io::Result<()> {
        let mut pos = 0;
        let data = nr_to_bytes(node);
        self.node_records_file.seek(SeekFrom::Start(id as u64))?;
        while pos < data.len() {
            let bytes_written = self.node_records_file.write(&data[pos..])?;
            pos += bytes_written;
        }
        Ok(())
    }
    fn scan(&mut self) -> std::io::Result<()> {
        let len = self.node_records_file.metadata()?.len();
        let mut data = [0u8; 1];
        let mut index = 0u64;
        while index < len {
            self.read_at(index, &mut data)?;
            let in_use = data[0] & 0b0000_0001 > 0;
            self.in_use_nodes.push(in_use);
            self.in_use_map.insert(self.in_use_nodes.len() - 1, v)
            index = 9 * self.in_use_nodes.len() as u64;
        }
        Ok(())
    }
    fn read_at(&mut self, pos: u64 , mut data:&mut [u8]) -> std::io::Result<()> {
        self.node_records_file.seek(SeekFrom::Start(pos))?;
        self.node_records_file.read_exact(&mut data)?;
        Ok(())
    }
    pub fn load(&mut self, node_id: u32) -> Option<NodeRecord> {
        let mut data = [0u8; 9];
        let res = self.read_at(node_id as u64 * 9, &mut data);
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