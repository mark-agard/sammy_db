use std::fs::File;
use std::io::Write;

pub struct Field {
    pub name: String,
    pub type_: String,
    pub length: usize,

}

impl Field {
    pub fn new(name: String, type_: String, length: usize) -> Field {
        Field {
            name,
            type_,
            length
        }
    }
}

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