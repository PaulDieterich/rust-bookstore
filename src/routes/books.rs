
use crate::models::book::Book;
use rocket::serde::json::Json;


#[get("/books")]
pub fn get_books() -> Json<Vec<Book>> {
    let book_one = Book{
        id: 1,

        title: String::from("The Great Gatsby"),
        author_name: String::from("F. Scott Fitzgerald"),
        site_count: 218,
        is_published: true,
    };
    let book_tow = Book{
        id: 2,
        title: String::from("The Great Gatsby"),
        author_name: String::from("F. Scott Fitzgerald"),
        site_count: 218,
        is_published: true,
    };
    let list = vec![book_one, book_tow];
    Json(list)
    
}

#[get("/books/<_id>")]
pub fn get_book_by_id(_id: usize) -> Json<Book> {
    let b = Book{
        id: 1,
        title: String::from("The Great Gatsby"),
        author_name: String::from("F. Scott Fitzgerald"),
        site_count: 218,
        is_published: true,
    };
    Json(b)
}

#[post("/books", format = "application/json", data = "<book>")]
pub fn create_book(book: Json<Book>) {
    let b = Book{
        id: book.id,
        title: book.title.clone(),
        author_name: book.author_name.clone(),
        site_count: book.site_count,
        is_published: book.is_published,
    };
    println!("book: {:?}", b);
}
