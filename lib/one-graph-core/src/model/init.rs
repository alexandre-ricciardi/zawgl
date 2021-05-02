use super::super::config::*;
use std::path;

#[derive(Debug, Clone)]
pub struct InitContext<'a> {
    db_dir: String,
    node_store_name: &'a str,
    relationships_store_name: &'a str,
    properties_store_name: &'a str,
    dynamic_store_name: &'a str,
    nodes_labels_index_name: &'a str,
}

impl <'a> InitContext<'a> {
    pub fn new(dir: &str) -> Self {
        InitContext{db_dir: String::from(dir), node_store_name: NODES_FILE_NAME,
            relationships_store_name: RELATIONSHIPS_FILE_NAME, 
            properties_store_name: PROPERTIES_FILE_NAME,
            dynamic_store_name: DYN_FILE_NAME,
            nodes_labels_index_name: NODES_LABELS_INDEX_FILE_NAME,
        }
    }

    pub fn get_nodes_store_path(&self) -> Option<String> {
        let mut file_path = path::PathBuf::new();
        file_path.push(self.db_dir.as_str());
        file_path.push(self.node_store_name);
        file_path.to_str().map(String::from)
    }
    pub fn get_relationships_store_path(&self) -> Option<String> {
        let mut file_path = path::PathBuf::new();
        file_path.push(self.db_dir.as_str());
        file_path.push(self.relationships_store_name);
        file_path.to_str().map(String::from)
    }
    pub fn get_properties_store_path(&self) -> Option<String> {
        let mut file_path = path::PathBuf::new();
        file_path.push(self.db_dir.as_str());
        file_path.push(self.properties_store_name);
        file_path.to_str().map(String::from)
    }
    pub fn get_dynamic_store_path(&self) -> Option<String> {
        let mut file_path = path::PathBuf::new();
        file_path.push(self.db_dir.as_str());
        file_path.push(self.dynamic_store_name);
        file_path.to_str().map(String::from)
    }
    
    pub fn get_nodes_labels_index_path(&self) -> Option<String> {
        let mut file_path = path::PathBuf::new();
        file_path.push(self.db_dir.as_str());
        file_path.push(self.nodes_labels_index_name);
        file_path.to_str().map(String::from)
    }
}