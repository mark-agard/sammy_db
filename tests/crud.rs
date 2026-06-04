use rh::database::Database;
use rh::table::Schema;
use std::collections::HashMap;

fn books_schema() -> Result<Schema, String> {
    let fields = vec![
        HashMap::from([("name".into(), "Title".into()), ("type".into(), "str".into())]),
        HashMap::from([("name".into(), "Author".into()), ("type".into(), "str".into())]),
        HashMap::from([("name".into(), "Year".into()), ("type".into(), "i16".into())]),
    ];
    Schema::new(fields)
}
 
#[test]
fn test_full_lifecycle() -> Result<(), String> {
    let tmp = std::env::temp_dir().join("redhare_test_lifecycle");
    let _ = std::fs::remove_dir_all(&tmp); // clean start
    
    // 1. Create
    let db = Database::new(&tmp)?;
    let mut table = db.create_table("books", books_schema()?)?;
    
    // 2. Append
    table.append_rows(vec![vec!["Book 1".into(), "Author A".into(), "2020".into()]])?;
    drop(table); // drop memory handle to force reload from disk
    
    // 3. Read after reload
    let table = db.load_table("books")?;
    let rows = table.read_rows()?;
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0][1], "Book 1");
    
    // 4. Update
    let mut table = db.load_table("books")?;
    table.update_rows(vec![(0, "Title".into(), "Updated".into())])?;
    drop(table);
    
    // 5. Read to confirm update persisted
    let table = db.load_table("books")?;
    let rows = table.read_rows()?;
    assert_eq!(rows[0][1], "Updated");
    
    // 6. Delete
    let mut table = db.load_table("books")?;
    table.delete_rows(vec![0])?;
    drop(table);
    
    // 7. Confirm deletion persisted
    let table = db.load_table("books")?;
    let rows = table.read_rows()?;
    assert_eq!(rows.len(), 0);
    
    Ok(())
}