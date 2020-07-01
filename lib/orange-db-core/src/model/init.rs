use super::super::conf::*;
use super::super::config::*;
use std::path;

pub struct InitContext {
    pub conf: Conf,
}

impl InitContext {
    pub fn get_nodes_store_path(&self) -> Option<String> {
        let mut file_path = path::PathBuf::new();
        file_path.push(&self.conf.db_dir);
        file_path.push(&self.conf.stores.nodes_store);
        file_path.to_str().map(String::from)
    }
    pub fn get_relationships_store_path(&self) -> Option<String> {
        let mut file_path = path::PathBuf::new();
        file_path.push(&self.conf.db_dir);
        file_path.push(&self.conf.stores.relationships_store);
        file_path.to_str().map(String::from)
    }
    pub fn get_properties_store_path(&self) -> Option<String> {
        let mut file_path = path::PathBuf::new();
        file_path.push(&self.conf.db_dir);
        file_path.push(&self.conf.stores.properties_store);
        file_path.to_str().map(String::from)
    }
    pub fn get_dynamic_store_path(&self) -> Option<String> {
        let mut file_path = path::PathBuf::new();
        file_path.push(&self.conf.db_dir);
        file_path.push(&self.conf.stores.dynamic_store);
        file_path.to_str().map(String::from)
    }
    
    pub fn get_nodes_labels_index_path(&self) -> Option<String> {
        let mut file_path = path::PathBuf::new();
        file_path.push(&self.conf.db_dir);
        file_path.push(&self.conf.stores.nodes_labels_index);
        file_path.to_str().map(String::from)
    }
}