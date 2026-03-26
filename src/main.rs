#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    year: i16,
}

fn add_book(title: &str, author: &str, year: i16) -> Book {
    Book {
        title: title.to_string(),
        author: author.to_string(),
        year: year,
    }
}

fn book_to_binary(book: &Book) -> Vec<u8> {
    let mut binary = Vec::new();
    binary.extend_from_slice(book.title.as_bytes());
    binary.push(0); // null terminator
    binary.extend_from_slice(book.author.as_bytes());
    binary.push(0); // null terminator
    binary.extend_from_slice(&book.year.to_le_bytes());
    binary
}

fn binary_to_book(binary: &[u8]) -> bool {
    let binary_vec = binary.to_vec();
    let split = binary_vec.split(|&x| x == 0).collect::<Vec<_>>();
    
    let title = String::from_utf8(split[0].to_vec()).unwrap();
    let author = String::from_utf8(split[1].to_vec()).unwrap();
    let year = i16::from_le_bytes([split[2][0], split[2][1]]);
    
    println!("title: {}", title);
    println!("author: {}", author);
    println!("year: {}", year);

    true
}
fn main() {
    let b1 = add_book("Crime and Punishment", "Fyodor Dostoevsky", 1866);
    let b2 = add_book("The Plague", "Albert Camus", 1947);

    let books = vec![b1, b2];
    
    for book in books {
        println!("{} by {} ({})", book.title, book.author, book.year);
        let book2binary = book_to_binary(&book);
        println!("Book to Binary: {:?}", book2binary);
        let binary2book = binary_to_book(&book2binary);
        println!("Binary to Book: {:?}", binary2book);
    }
    
}
