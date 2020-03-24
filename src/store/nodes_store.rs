use std::fs::File;
use std::io::prelude::*;
use super::records::*;
use std::io::SeekFrom;

pub struct NodesStore {
    node_records_file: File,
    not_in_use_nodes: Vec<u64>,
}

impl NodesStore {
    pub fn new(file: &str) -> Self {
        let mut f = File::create(file).expect("Cannot open the nodes store file");
        let mut store = NodesStore { node_records_file: f, not_in_use_nodes: Vec::new()};
        store.scan();
        store
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
    fn scan(&mut self) -> std::io::Result<()> {
        let len = self.node_records_file.metadata()?.len();
        let mut data = [0u8; 1];
        let mut index = 0u64;
        while index < len {
            self.read_at(index * 17, &mut data)?;
            let in_use = data[0] & 0b0000_0001 > 0;
            self.not_in_use_nodes.push(index * 17);
            index += 17;
        }
        Ok(())
    }
    fn read_at(&mut self, pos: u64 , mut data:&mut [u8]) -> std::io::Result<()> {
        self.node_records_file.seek(SeekFrom::Start(pos))?;
        self.node_records_file.read_exact(&mut data)?;
        Ok(())
    }
    pub fn load(&mut self, node_id: u64) -> Option<NodeRecord> {
        let mut data = [0u8; 17];
        let res = self.read_at(node_id * 17, &mut data);
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
            next_rel_id: 11287665,
            next_prop_id: 89089807,
        };
        store.save(nr);
        let load = store.load(0);
        if let Some(r) = load {
            assert_eq!(r.in_use, true);
            assert_eq!(r.next_rel_id, 11287665);
            assert_eq!(r.next_prop_id, 89089807);
        }
    }
}