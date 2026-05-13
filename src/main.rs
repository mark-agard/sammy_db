mod field;
use crate::field::Field;

mod schema;
use crate::schema::Schema;

mod table;
use crate::table::create_table;




fn main() {
    
    let mut schema = Schema::new("books", vec![]);

    let title = Field::new("Title".to_string(), "string".to_string(), 100);
    let author = Field::new("Author".to_string(), "string".to_string(), 100);
    let year = Field::new("Year".to_string(), "i16".to_string(), 2);
    
    schema.fields.push(title);
    schema.fields.push(author);
    schema.fields.push(year);
    
    create_table(schema);
}
