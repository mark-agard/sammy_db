use std::fs::{File, OpenOptions};
use std::io::Write;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct Schema{
    pub fields: Vec<HashMap<String, String>>
}

impl Schema {
    // I would like to do input cleaning here
    pub fn new(fields: Vec<HashMap<String, String>>) -> Result<Schema, String> {
        Ok(Schema { fields })
    }

    pub fn parse_from_bytes(schema_bytes: &[u8]) -> Result<Schema, String> {
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
        Ok(Schema { fields })
    }
}

pub struct Table {
    pub name: String, // Table name
    pub schema: Schema, // Table schema
    pub path: PathBuf, // Table path
    pub next_rowid: u64 // Next row ID
    
}

impl Table {
    // Create a new table, writing to disk and returning memory object
    pub fn new(name: &str, path: &Path, schema: Schema) -> Result<Table, String> {
        // Create table directory
        std::fs::create_dir_all(path).unwrap();
        
        // Create schema bytes
        let mut schema_bytes = Vec::new();
        for field in &schema.fields {
            // Push field name length and content
            let field_name_bytes = field.get("name").unwrap().as_bytes();
            schema_bytes.extend_from_slice(&(field_name_bytes.len() as u16).to_le_bytes());
            schema_bytes.extend_from_slice(field_name_bytes);

            // Push field type
            let field_type = field.get("type").unwrap();
            schema_bytes.extend_from_slice(field_type.as_bytes());
        }
        
        // Create schema file
        let mut schema_file = File::create(path.join("schema.bin")).unwrap();
        schema_file.write_all(&schema_bytes).unwrap();
        

        // Create meta file
        let mut meta_file = File::create(path.join("meta.bin")).unwrap();
        let mut meta_info = Vec::new();

        // Construct meta file content
        meta_info.extend_from_slice(&(name.as_bytes().len() as u16).to_le_bytes());
        meta_info.extend_from_slice(name.as_bytes());
        meta_info.extend_from_slice(&0u64.to_le_bytes());

        // Write to meta file
        meta_file.write_all(&meta_info).unwrap();
        
        // Return memory object.
        Ok(Table {
            name: name.to_string(),
            schema,
            path: path.to_path_buf(),
            next_rowid: 0
        })

    }
    
    // Create memory object from disk
    pub fn load(path: &Path) -> Result<Table, String> {
        // Read meta file which contains next row_id and table name
        let meta_info = std::fs::read(path.join("meta.bin")).unwrap();
        
        // Parse meta info
        let name_len = u16::from_le_bytes([meta_info[0], meta_info[1]]) as usize;
        let name = String::from_utf8(meta_info[2..2 + name_len].to_vec()).unwrap();
        let next_rowid = u64::from_le_bytes(meta_info[2 + name_len..2 + name_len + 8].try_into().unwrap());
        
        // Parse schema
        let schema_bytes = std::fs::read(path.join("schema.bin")).unwrap();
        let schema = Schema::parse_from_bytes(&schema_bytes);
        

        Ok(Table {
            name,
            schema: schema?,
            path: path.to_path_buf(),
            next_rowid
        })
    }

    // Write new "next rowid" to disk from memory. 
    pub fn save_meta(&self) {
         // Create meta file
        let mut meta_file = File::create(self.path.join("meta.bin")).unwrap();
        let mut meta_info = Vec::new();

        // Construct meta file content
        meta_info.extend_from_slice(&(self.name.as_bytes().len() as u16).to_le_bytes());
        meta_info.extend_from_slice(self.name.as_bytes());
        meta_info.extend_from_slice(&self.next_rowid.to_le_bytes());

        // Write to meta file
        meta_file.write_all(&meta_info).unwrap();
    }

    // Append rows to data.bin. Assigns rowid values.
    pub fn append_rows(&mut self, rows: Vec<Vec<String>>) -> Result<(), String> {
        // Initialize empty vec to hold row data
        let mut content = Vec::new();

        // Iterate over row data in vec
        for row in rows {
            let rowid = self.next_rowid;
            content.extend_from_slice(&rowid.to_le_bytes());
            self.next_rowid += 1;

            for (field, value) in self.schema.fields.iter().zip(row.iter()) {
                match field.get("type").unwrap().as_str() {
                    "str" => {
                        let bytes = value.as_bytes();
                        content.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
                        content.extend_from_slice(bytes);
                    }
                    "i16" => {
                        let num: i16 = value.parse().unwrap();
                        content.extend_from_slice(&num.to_le_bytes());
                    }

                    _ => return Err("Invalid field type".to_string()),
                }
            }
        }
        // Write data to file
        let mut data_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.path.join("data.bin"))
            .unwrap();

        data_file.write_all(&content).unwrap();

        // Update meta file to write new "next_rowid" to disk
        self.save_meta();

        Ok(())
    }

    // Rewrite rows in data.bin for use in update/delete rows. Previous rowid values are preserved.
    pub fn rewrite_rows(&mut self, rows: Vec<Vec<String>>) -> Result<(), String> {
         // Initialize empty vec to hold row data
        let mut content = Vec::new();

        // Iterate over row data in vec
        for row in rows {
            // preserve existing rowid
            let rowid: u64 = row[0].parse().unwrap();
            content.extend_from_slice(&rowid.to_le_bytes());

            // Iterate over fields and values to write, as is done in append_rows()
            for (field, value) in self.schema.fields.iter().zip(row.iter().skip(1)) {
                match field.get("type").unwrap().as_str() {
                    "str" => {
                        let bytes = value.as_bytes();
                        content.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
                        content.extend_from_slice(bytes);
                    }
                    "i16" => {
                        let num: i16 = value.parse().unwrap();
                        content.extend_from_slice(&num.to_le_bytes());
                    }

                    _ => return Err("Invalid field type".to_string()),
                }
            }
        }
        // Write data to file, overwriting existing data instead of appending rows
        let mut data_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self.path.join("data.bin"))
            .unwrap();

        data_file.write_all(&content).unwrap();

        Ok(())
    }

    // Read rows from data.bin
    pub fn read_rows(&self) -> Result<Vec<Vec<String>>, String> {
        // Initialize vec to hold rows
        let mut rows = Vec::new();
        
        // Read datafile and initialize cursor
        let bytes = std::fs::read(self.path.join("data.bin")).unwrap();
        let mut cursor = 0;

        while cursor < bytes.len() {
            // Initialize empty vec to hold cells
            let mut row = Vec::new();

            // Read rowid and advance cursor
            let rowid = u64::from_le_bytes(bytes[cursor..cursor + 8].try_into().unwrap());
            row.push(rowid.to_string());
            cursor += 8;

            // Use schema to parse row of data
            for field in &self.schema.fields {
                match field.get("type").unwrap().as_str() {
                    "str" => {
                        // Read string length to determine cursor movement
                        let len: u16 = u16::from_le_bytes(bytes[cursor..cursor + 2].try_into().unwrap());
                        cursor += 2;

                        // Read string data
                        let value = String::from_utf8(bytes[cursor..cursor + len as usize].to_vec()).unwrap();
                        cursor += len as usize;
                        row.push(value);
                    }
                    "i16" => {
                        // Read i16 data
                        let value: i16 = i16::from_le_bytes(bytes[cursor..cursor + 2].try_into().unwrap());
                        cursor += 2;
                        row.push(value.to_string());
                    }
                    _ => return Err("Invalid field type".to_string()),
                }
            }
            
            rows.push(row);
        }

        Ok(rows)
    }

    // Delete rows based on rowid
    pub fn delete_rows(&mut self, deletions: Vec<u64>) -> Result<(), String> {
        let delete_set: HashSet<u64> = deletions.into_iter().collect();

        let rows = self.read_rows().unwrap();
        let filtered_rows: Vec<Vec<String>> = rows.into_iter().filter(|row| {
            let rowid: u64 = row[0].parse().unwrap();
            !delete_set.contains(&rowid)
        }).collect();

        let _ = self.rewrite_rows(filtered_rows);

        Ok(())
    }

    // Update rows based on rowid
    pub fn update_rows(&mut self, updates: Vec<(u64, String, String)>) -> Result<(), String> {
        // We expect input where a rowid can appear in multiple update tuples.
        // However, we can group updates by rowid so that we can make all edits for a row first time we see it
        let mut updates_by_row: HashMap<u64, Vec<(String, String)>> = HashMap::new();
        
        for (rowid, field_name, value) in updates {
            updates_by_row.entry(rowid).or_insert_with(Vec::new).push((field_name, value));
        }
        
        // Now we can process each row's updates
        let rows = self.read_rows().unwrap();
        let mut updated_rows: Vec<Vec<String>> = Vec::new();
        
        // Iterate through rows. If no update (else case), keep the row as is.
        // If the rowid is present in updates, then apply the updates to the row.
        for row in rows {
            let rowid: u64 = row[0].parse().unwrap();
            if let Some(updates) = updates_by_row.get(&rowid) {
                // Apply updates to this row
                let mut updated_row = row.clone();
                for (field_name, value) in updates {
                    // Find the index of the field and update it
                    for (i, field) in self.schema.fields.iter().enumerate() {
                        if field.get("name").unwrap() == field_name {
                            updated_row[i + 1] = value.clone();
                            break;
                        }
                    }
                }
                updated_rows.push(updated_row);
            } else {
                updated_rows.push(row);
            }
        }
        
        // Overwrite data file with filtered data
        let _ = self.rewrite_rows(updated_rows);

        Ok(())
    }
    
}
