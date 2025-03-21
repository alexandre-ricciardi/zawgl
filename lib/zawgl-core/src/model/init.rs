// MIT License
// Copyright (c) 2022 Alexandre RICCIARDI
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use super::super::config::*;
use std::path;
use std::env;
use log::info;

#[derive(Debug, Clone)]
pub struct InitContext {
    pub root: String,
    pub dbs_ctx: Vec<DatabaseInitContext>,
}

impl InitContext {
    pub fn new(root: &str, names: &Vec<String>) -> Self {
        Self { root: root.to_string(), dbs_ctx: names.iter().map(|n| DatabaseInitContext::new(root, n).expect("can't create database context")).collect() }
    }
}

/// Zawgl database initialization context
#[derive(Debug, Clone)]
pub struct DatabaseInitContext {
    pub db_name: String,
    db_dir: String,
    node_store_name: String,
    relationships_store_name: String,
    properties_store_name: String,
    dynamic_store_name: String,
    nodes_labels_index_name: String,
    relationships_types_index_name: String,
    labels_store_name: String,
    meta_data_index_store_name: String,
    meta_data_values_store_name: String,
}

fn build_path(dir: &str, file: &str) -> Option<String> {
    let mut file_path = path::PathBuf::new();
    file_path.push(dir);
    file_path.push(file);
    file_path.to_str().map(String::from)
}

impl DatabaseInitContext {
    /// Builds an initialization context by providing the database data directory
    pub fn new(root: &str, db_name: &str) -> Option<Self> {
        let mut dir_path_buf = path::PathBuf::new();
        let root_dir = path::Path::new(root);
        if root_dir.is_absolute() {
            dir_path_buf.push(root_dir)
        } else {
            let current_dir = env::current_dir().ok()?;
            dir_path_buf.push(current_dir);
            dir_path_buf.push(root_dir);
        }
        dir_path_buf.push(db_name);
        std::fs::create_dir_all(dir_path_buf.clone()).ok()?;
        let os_str = dir_path_buf.as_os_str();
        info!("Database directory: {}", os_str.to_str()?);
        Some(DatabaseInitContext{db_name: db_name.to_string(), db_dir: String::from(os_str.to_str()?), node_store_name: NODES_FILE_NAME.to_string(),
            relationships_store_name: RELATIONSHIPS_FILE_NAME.to_string(), 
            properties_store_name: PROPERTIES_FILE_NAME.to_string(),
            dynamic_store_name: DYN_FILE_NAME.to_string(),
            nodes_labels_index_name: NODES_LABELS_INDEX_FILE_NAME.to_string(),
            relationships_types_index_name: RELATIONSHIPS_TYPES_INDEX_FILE_NAME.to_string(),
            labels_store_name: LABELS_FILE_NAME.to_string(),
            meta_data_index_store_name: META_DATA_INDEX_FILE_NAME.to_string(),
            meta_data_values_store_name: META_DATA_VALUES_FILE_NAME.to_string(),
        })
    }

    pub fn get_nodes_store_path(&self) -> Option<String> {
        build_path(self.db_dir.as_str(), &self.node_store_name)
    }
    pub fn get_relationships_store_path(&self) -> Option<String> {
        build_path(self.db_dir.as_str(), &self.relationships_store_name)
    }
    pub fn get_properties_store_path(&self) -> Option<String> {
        build_path(self.db_dir.as_str(), &self.properties_store_name)
    }
    pub fn get_dynamic_store_path(&self) -> Option<String> {   
        build_path(self.db_dir.as_str(), &self.dynamic_store_name)
    }
    
    pub fn get_nodes_labels_index_path(&self) -> Option<String> {
        build_path(self.db_dir.as_str(), &self.nodes_labels_index_name)
    }

    pub fn get_relationships_types_index_path(&self) -> Option<String> {
        build_path(self.db_dir.as_str(), &self.relationships_types_index_name)
    }

    pub fn get_labels_store_path(&self) -> Option<String> {
        build_path(self.db_dir.as_str(), &self.labels_store_name)
    }

    pub fn get_meta_data_index_store_path(&self) -> Option<String> {
        build_path(self.db_dir.as_str(), &self.meta_data_index_store_name)
    }

    pub fn get_meta_data_values_store_path(&self) -> Option<String> {
        build_path(self.db_dir.as_str(), &self.meta_data_values_store_name)
    }
}