use std::fs::File;
use std::io::Write;

struct Book {
    title: String,
    author: String,
    year: String, //making this a string- I need to figure out how to handle non-string data
}

impl Book{
    fn fields(&self) -> Vec<&str> {
        vec![&self.title, &self.author, &self.year]
    }
    fn new(title: &str, author: &str, year: &str) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            year: year.to_string(),
        }
    }
}


fn read(path: &str) -> bool {
    let content: Vec<u8> = std::fs::read(path).unwrap();
    for line in content.split(|&b| b == b'\n') {
        println!("{}", std::str::from_utf8(line).unwrap());
    }
    true
}

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
fn main() {
    loop {
        println!("Welcome to SammyDB\n1. Add Books\n2. List Books\n3. Exit");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "1" => {
                println!("How many books do you want to add?");
                let mut count = String::new();
                std::io::stdin().read_line(&mut count).unwrap();
                let count = count.trim().parse::<usize>().unwrap();
                let mut books = Vec::new();
                
                for _ in 0..count {
                    println!("Enter title:");
                    let mut title = String::new();
                    std::io::stdin().read_line(&mut title).unwrap();
                    println!("Enter author:");
                    let mut author = String::new();
                    std::io::stdin().read_line(&mut author).unwrap();
                    println!("Enter year:");
                    let mut year = String::new();
                    std::io::stdin().read_line(&mut year).unwrap();
                    books.push(Book::new(title.trim(), author.trim(), year.trim()));
                }
                write(&books);
            }
            "2" => {
                read("books.bin");
            }
            "3" => {
                break;
            }
            _ => {
                println!("Invalid input");
            }
        }
    }
}
