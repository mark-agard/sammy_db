use rh::table::{Table, Schema};
use rh::database::Database;

use std::collections::HashMap;
use std::path::Path;

fn main() -> Result<(), String>{
    // set path (used in all scopes)
    let db: Database = Database::new(Path::new("mydb"))?;
     
    // Scope 1: Make a table
    {
    let fields: Vec<HashMap<String, String>> = vec![
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
    
    let schema: Schema = Schema::new(fields)?;
    let _table: Table = db.create_table("books", schema).unwrap();
    }

    // Scope 2: Test load functionality, append rows to table
    {
        let rows = vec![
            vec![String::from("Test Book 1"), String::from("Author 1"), String::from("2020")],
            vec![String::from("Test Book 2"), String::from("Author 2"), String::from("2021")],
            vec![String::from("Test Book 3"), String::from("Author 3"), String::from("2022")],
        ];

        let mut table: Table = db.load_table("books")?;
        let _ = table.append_rows(rows);
    }

    // Scope 3: test read functionality
    {
        let table = db.load_table("books")?;
        let rows = table.read_rows()?;
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
        let mut table = db.load_table("books")?;
        let _ = table.update_rows(vec![(1, String::from("Title"), String::from("Updated Title"))]);
    
    }

    // Scope 4b. Confirm updates written to disk.
    {
        let table = db.load_table("books")?;
        let rows = table.read_rows()?;
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
        let mut table = db.load_table("books")?;
        let _ = table.delete_rows(vec![1]);
    }
    
    // Scope 5b. Confirm delete written to disk
    {
        let table = db.load_table("books")?;
        let rows = table.read_rows()?;
        println!("--- After delete ---");
        println!("| {:<15} | {:<15} | {:<6} |", "Title", "Author", "Year");
        println!("{}", "-".repeat(44));
        for row in &rows {
            println!("| {:<15} | {:<15} | {:<6} |", row[1], row[2], row[3]);
        }
        println!();
    }
    Ok(())
}
