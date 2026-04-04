use std::fs::File;
use std::io::Write;
mod schema;

fn read(path: &str) -> bool {
    let content: Vec<u8> = std::fs::read(path).unwrap();
    for line in content.split(|&b| b == b'\n') {
        println!("{}", std::str::from_utf8(line).unwrap());
    }
    true
}
// this creates a table with a schema file and no data files
fn create_table(schema: schema::Schema) -> bool { 
    // Create dir for all table files
    std::fs::create_dir_all(schema.name).unwrap();

    // Create schema file
    schema.make_file();
    true
}

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

fn main() {
    
    let mut schema = schema::Schema::new("books", vec![]);

    let title = schema::Field::new("Title".to_string(), "string".to_string(), 100);
    let author = schema::Field::new("Author".to_string(), "string".to_string(), 100);
    let year = schema::Field::new("Year".to_string(), "i16".to_string(), 2);
    
    schema.fields.push(title);
    schema.fields.push(author);
    schema.fields.push(year);
    
    create_table(schema);
}
