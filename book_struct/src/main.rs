use std::fs::File;
use std::io::{Write, BufReader, BufRead};

// --- Struct Definition ---
// The fields: title (String), author (String), year (u16).
struct Book {
    title: String,
    author: String,
    year: u16,
}

// --- File I/O Functions ---

/// Saves a vector of Book structs to a file, with each book on a separate line
/// and fields separated by commas (title,author,year).
fn save_books(books: &Vec<Book>, filename: &str) {
    // 1. Create or overwrite the file. Use match for error handling.
    let mut file = match File::create(filename) {
        Ok(f) => f,
        Err(e) => {
            // Print error and exit if file creation fails
            println!("Error creating file '{}': {}", filename, e);
            return;
        }
    };

    // 2. Iterate over books and write them line by line
    for book in books {
        let line = format!("{},{},{}\n", book.title, book.author, book.year);
        
        if let Err(e) = file.write_all(line.as_bytes()) {
            // Print error if writing fails and stop processing
            println!("Error writing book '{}' to file: {}", book.title, e);
            return;
        }
    }
}

/// Loads books from a file, expecting each line to be in the format: title,author,year.
/// Returns an empty vector if the file cannot be opened.
fn load_books(filename: &str) -> Vec<Book> {
    // 1. Open the file. Return an empty vector on failure.
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            println!("Error opening file '{}': {}", filename, e);
            return Vec::new();
        }
    };

    let reader = BufReader::new(file);
    let mut books = Vec::new();

    // 2. Iterate over the lines of the file
    for (line_num, line_result) in reader.lines().enumerate() {
        let line = match line_result {
            Ok(l) => l,
            Err(e) => {
                println!("Error reading line {}: {}", line_num + 1, e);
                continue; // Skip corrupted line
            }
        };

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue; // Skip empty lines
        }

        // Split the line by the comma delimiter
        let parts: Vec<&str> = trimmed_line.split(',').collect();

        // Ensure we have exactly three components
        if parts.len() != 3 {
            println!("Skipping line {} due to incorrect format (expected 3 parts, got {}): {}", 
                      line_num + 1, parts.len(), trimmed_line);
            continue;
        }

        // Try to parse the year (the third part)
        let year: u16 = match parts[2].trim().parse() {
            Ok(y) => y,
            Err(e) => {
                println!("Skipping line {} due to year parsing error for '{}': {}", 
                          line_num + 1, parts[2].trim(), e);
                continue; // Skip if year is not a valid u16
            }
        };

        // If all checks pass, create and store the Book
        books.push(Book { 
            title: parts[0].trim().to_string(), // .trim() handles potential spaces around commas
            author: parts[1].trim().to_string(),
            year,
        });
    }

    books
}

fn main() {
    // Create the initial catalog
    let catalog = vec![
        Book { 
            title: "1984".to_string(), 
            author: "George Orwell".to_string(), 
            year: 1949 
        },
        Book { 
            title: "To Kill a Mockingbird".to_string(), 
            author: "Harper Lee".to_string(), 
            year: 1960 
        },
        Book { 
            title: "The Great Gatsby".to_string(), 
            author: "F. Scott Fitzgerald".to_string(), 
            year: 1925 
        },
    ];

    const FILENAME: &str = "books.txt";

    // 1. Save the books
    save_books(&catalog, FILENAME);
    println!("Successfully saved {} books to '{}'.", catalog.len(), FILENAME);

    // 2. Load the books
    let loaded_books = load_books(FILENAME);
    
    // 3. Print the loaded books
    if loaded_books.is_empty() {
        println!("\nCould not load any books. Check for file opening or parsing errors.");
    } else {
        println!("\nSuccessfully loaded {} books. Catalog details:", loaded_books.len());
        for book in loaded_books {
            println!("- Title: '{}', Author: {}, Year: {}", book.title, book.author, book.year);
        }
    }
}
