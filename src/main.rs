


mod table;
use crate::table::Table;
use crate::table::Schema;

use std::collections::HashMap;
use std::path::Path;

fn main() {
<<<<<<< HEAD
    // set path (used in all scopes)
    let path = Path::new("test");
     
    // Scope 1: Make a table
    {
=======
    
>>>>>>> 96ca6c2154e8869eecf1cd309fdd7fb39bf05737
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
<<<<<<< HEAD
    Table::new("test", path, schema);
    }

    // Scope 2: Test load functionality, append rows to table
    {
        let rows = vec![
            vec![String::from("Test Book 1"), String::from("Author 1"), String::from("2020")],
            vec![String::from("Test Book 2"), String::from("Author 2"), String::from("2021")],
            vec![String::from("Test Book 3"), String::from("Author 3"), String::from("2022")],
        ];

        let mut table = Table::load(path).unwrap();
        let _ = table.append_rows(rows);
    }

    // Scope 3: test read functionality
    {
        let table = Table::load(path).unwrap();
        let rows = table.read_rows().unwrap();
        println!("--- After append ---");
        println!("| {:<15} | {:<15} | {:<6} |", "Title", "Author", "Year");
        println!("{}", "-".repeat(44));
        for row in &rows {
            println!("| {:<15} | {:<15} | {:<6} |", row[1], row[2], row[3]);
        }
        println!();
    }

    // Scope 4: test update functionality
    {
        let mut table = Table::load(path).unwrap();
        let _ = table.update_rows(vec![(1, String::from("Title"), String::from("Updated Title"))]);
    
    }

    // Scope 4b. Confirm updates written to disk.
    {
        let table = Table::load(path).unwrap();
        let rows = table.read_rows().unwrap();
        println!("--- After update ---");
        println!("| {:<15} | {:<15} | {:<6} |", "Title", "Author", "Year");
        println!("{}", "-".repeat(44));
        for row in &rows {
            println!("| {:<15} | {:<15} | {:<6} |", row[1], row[2], row[3]);
        }
        println!();
    }

    // Scope 5: Test delete functionality
    {
        let mut table = Table::load(path).unwrap();
        let _ = table.delete_rows(vec![1]);
    }
    
    // Scope 5b. Confirm delete written to disk
    {
        let table = Table::load(path).unwrap();
        let rows = table.read_rows().unwrap();
        println!("--- After delete ---");
        println!("| {:<15} | {:<15} | {:<6} |", "Title", "Author", "Year");
        println!("{}", "-".repeat(44));
        for row in &rows {
            println!("| {:<15} | {:<15} | {:<6} |", row[1], row[2], row[3]);
        }
        println!();
    }
=======
    let path = Path::new("test");

    Table::new(path, schema);
>>>>>>> 96ca6c2154e8869eecf1cd309fdd7fb39bf05737
}
