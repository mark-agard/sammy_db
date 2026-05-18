use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use std::path::Path;

pub struct Schema{
    pub fields: Vec<HashMap<String, String>>
}

impl Schema {
    // I would like to do input cleaning here
    pub fn new(fields: Vec<HashMap<String, String>>) -> Schema {
        Schema { fields }
    }

    pub fn parse_from_bytes(schema_bytes: &[u8]) -> Schema {
        let mut fields = Vec::new();
        let mut cursor = 0;
        
        while cursor < schema_bytes.len() {
            // Read field name length
            let field_name_len = u16::from_le_bytes([schema_bytes[cursor], schema_bytes[cursor + 1]]) as usize;
            cursor += 2;

            // Read field name
            let field_name = String::from_utf8(schema_bytes[cursor..cursor + field_name_len].to_vec()).unwrap();
            cursor += field_name_len;

            // Read field type
            let field_type = String::from_utf8(schema_bytes[cursor..cursor + 3].to_vec()).unwrap();
            cursor += 3;

            fields.push(HashMap::from([("name".to_string(), field_name), ("type".to_string(), field_type)]));
        }
        Schema { fields }
    }
}

pub struct Table {
    pub name: String,
    pub schema: Schema
}

impl Table {
    pub fn new(path: &Path, schema: Schema) -> Table {
        // Create table directory
        std::fs::create_dir_all(path).unwrap();
        
        // Create schema bytes
        let mut schemaBytes = Vec::new();
        for field in &schema.fields {
            // push field name
            let field_name_bytes = field.get("name").unwrap().as_bytes();
            schemaBytes.extend_from_slice(&(field_name_bytes.len() as u16).to_le_bytes());
            schemaBytes.extend_from_slice(field_name_bytes);

            // Access field type
            let field_type = field.get("type").unwrap();
            schemaBytes.extend_from_slice(field_type.as_bytes());
        }
        
        // Create schema file
        let mut schema_file = File::create(path.join("schema.bin")).unwrap();
        schema_file.write_all(&schemaBytes).unwrap();
        
        Table {
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            schema
        }
    }

    pub fn append_rows(&self, rows: Vec<Vec<String>>) -> Result<(), String> {
        // TODO: the writing!
        Ok(())
    }

    pub fn read_rows(&self) -> Result<Vec<Vec<String>>, String> {
        let rows = Vec::new();
        // TODO: the reading and parsing LOL
        Ok(rows)
    }

    pub fn delete_rows(&self) -> Result<(), String> {
        Ok(())
    }

    pub fn update_rows(&self) -> Result<(), String> {
        Ok(())
    }
    
}
