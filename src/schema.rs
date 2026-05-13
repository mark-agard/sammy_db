use std::fs::File;
use std::io::Write;

use crate::field::Field;

pub struct Schema {
    pub name: &'static str,
    pub fields: Vec<Field>,
}

impl Schema {
    pub fn new(name: &'static str, fields: Vec<Field>) -> Schema {
        Schema {
            name,
            fields
        }
    }

    // this should probably not use newlines. should be properly written to binary
    pub fn make_file(&self) -> bool {
        let mut file = File::create(format!("{}/schema", self.name)).unwrap();
        // in my heart this for loop should not exist. 
        for field in &self.fields {
            file.write_all(field.name.as_bytes()).unwrap();
            file.write_all(b" ").unwrap();
            file.write_all(field.type_.as_bytes()).unwrap();
            file.write_all(b" ").unwrap();
            file.write_all(field.length.to_string().as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
        true
    }
}