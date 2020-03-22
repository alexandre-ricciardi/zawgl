use super::records::NodeRecord;
use std::fs::File;
use std::io::prelude::*;
pub struct NodesStore {
    node_records_file: File
}

impl NodesStore {
    pub fn new(file: &str) -> Self {
        let mut f = File::create(file).expect("Cannot open the nodes store file");
        NodeStore { node_records_file: f }
    }
    pub fn save(&mut self, node: NodeRecord) {

    }
}