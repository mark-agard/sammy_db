use crate::schema;

// implement as class with attached methods

// this method needs to read based on schema
pub fn read(path: &str) -> bool {
    let content: Vec<u8> = std::fs::read(path).unwrap();
    for line in content.split(|&b| b == b'\n') {
        println!("{}", std::str::from_utf8(line).unwrap());
    }
    true
}

// this creates a table with a schema file and no data files
pub fn create_table(schema: schema::Schema) -> bool { 
    // Create dir for all table files
    std::fs::create_dir_all(schema.name).unwrap();

    // Create schema file
    schema.make_file();
    true
}


// i'm keeping this block as a reference but
// it should be written to append according to schema
/*
fn write(data: &[Book]) -> bool {
    let mut binary = Vec::new();
    for (index, book) in data.iter().enumerate() {
        for field in book.fields() {
            binary.extend_from_slice(field.as_bytes());
            binary.push(b' '); // null terminator
        } 
        if index < data.len() - 1 {
            binary.push(b'\n'); // newline terminator
        }
    }
    let mut file = File::create(format!("books.bin")).unwrap();
    file.write_all(&binary).unwrap();
    
    true
}
*/