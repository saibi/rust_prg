#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn find_rust(books: &Vec<Book>) -> Vec<&Book> {
    let mut found_books: Vec<&Book> = Vec::new();

    for b in books {
        if b.title.contains("Rust") {
            found_books.push(b);
        }
    }
    found_books
}

fn main() {
    let rust_book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 2018,
    };

    let rust_in_action = Book {
        title: String::from("Rust in Action"),
        author: String::from("Tim McNamara"),
        published: 2021,
    };

    let another = Book {
        title: String::from("book"),
        author: String::from("author"),
        published: 2020,
    };

    let mut library = Vec::new();
    library.push(rust_book);
    library.push(rust_in_action);
    library.push(another);

    let rust_books = find_rust(&library);

    if rust_books.is_empty() {
        println!("No books found");
    } else {
        let mut only_titles = Vec::new();

        for b in rust_books {
            let mut title = b.title.clone();
            title.push('\n');
            only_titles.push(title);
        }

        let collect = only_titles.into_iter().collect::<String>();
        println!("{}", collect);
    }
}
