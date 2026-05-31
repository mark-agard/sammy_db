use std::path::{Path, PathBuf};
use crate::table::{Table, Schema};

pub struct Database {
    pub path: PathBuf,
}   

impl Database {
    pub fn new(path: &Path) -> Database {
        std::fs::create_dir_all(path).unwrap();
        Database { path: path.to_path_buf() }
    }

    pub fn create_table(&self, name: &str, schema: Schema) -> Result<Table, String> {
        let table_path = self.path.join(name);
        if table_path.exists() {
            return Err("Table already exists".to_string());
        }
        Ok(Table::new(name, &table_path, schema))
    }

    pub fn load_table(&self, name: &str) -> Result<Table, String> {
       Table::load(&self.path.join(name))
    }

    pub fn list_tables(&self) -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }
}