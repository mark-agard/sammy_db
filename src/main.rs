


mod table;
use crate::table::Table;
use crate::table::Schema;

use std::collections::HashMap;
use std::path::Path;

fn main() {
    
    let fields = vec![
        HashMap::from([
            (String::from("name"), String::from("Title")), 
            (String::from("type"), String::from("str"))]),

        HashMap::from([
            (String::from("name"), String::from("Author")), 
            (String::from("type"), String::from("str"))]),
            
        HashMap::from([
            (String::from("name"), String::from("Year")), 
            (String::from("type"), String::from("i16"))]),
    ];
    
    let schema = Schema::new(fields);
    let path = Path::new("test");

    Table::new(path, schema);
}
