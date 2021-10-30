use super::super::config::*;
use std::path;
use std::env;

#[derive(Debug, Clone)]
pub struct InitContext<'a> {
    db_dir: String,
    node_store_name: &'a str,
    relationships_store_name: &'a str,
    properties_store_name: &'a str,
    dynamic_store_name: &'a str,
    nodes_labels_index_name: &'a str,
    relationships_types_index_name: &'a str,
    labels_store_name: &'a str,
}

fn build_path(dir: &str, file: &str) -> Option<String> {
    let mut file_path = path::PathBuf::new();
    file_path.push(dir);
    file_path.push(file);
    file_path.to_str().map(String::from)
}

impl <'a> InitContext<'a> {
    pub fn new(dir: &str) -> Option<Self> {
        let mut dir_path_buf = path::PathBuf::new();
        let dir_path = path::Path::new(dir);
        if dir_path.is_absolute() {
            dir_path_buf.push(dir)
        } else {
            let current_dir = env::current_dir().ok()?;
            dir_path_buf.push(current_dir);
            dir_path_buf.push(dir);
        }
        std::fs::create_dir_all(dir_path_buf.clone()).ok()?;
        let os_str = dir_path_buf.as_os_str();
        Some(InitContext{db_dir: String::from(os_str.to_str()?), node_store_name: NODES_FILE_NAME,
            relationships_store_name: RELATIONSHIPS_FILE_NAME, 
            properties_store_name: PROPERTIES_FILE_NAME,
            dynamic_store_name: DYN_FILE_NAME,
            nodes_labels_index_name: NODES_LABELS_INDEX_FILE_NAME,
            relationships_types_index_name: RELATIONSHIPS_TYPES_INDEX_FILE_NAME,
            labels_store_name: LABELS_FILE_NAME,
        })
    }

    pub fn get_nodes_store_path(&self) -> Option<String> {
        build_path(self.db_dir.as_str(), self.node_store_name)
    }
    pub fn get_relationships_store_path(&self) -> Option<String> {
        build_path(self.db_dir.as_str(), self.relationships_store_name)
    }
    pub fn get_properties_store_path(&self) -> Option<String> {
        build_path(self.db_dir.as_str(), self.properties_store_name)
    }
    pub fn get_dynamic_store_path(&self) -> Option<String> {   
        build_path(self.db_dir.as_str(), self.dynamic_store_name)
    }
    
    pub fn get_nodes_labels_index_path(&self) -> Option<String> {
        build_path(self.db_dir.as_str(), self.nodes_labels_index_name)
    }

    pub fn get_relationships_types_index_path(&self) -> Option<String> {
        build_path(self.db_dir.as_str(), self.relationships_types_index_name)
    }

    pub fn get_labels_store_path(&self) -> Option<String> {
        build_path(self.db_dir.as_str(), self.labels_store_name)
    }
}