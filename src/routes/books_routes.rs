use rocket::{delete, get, post, put};
use rocket_contrib::json::Json;
use serde_json::Value;

use crate::books::models::{Book, BookPublished, NewBook};
use crate::db::Conn as DbConn;

#[get("/books", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let books: Vec<Book> = Book::all_books(&conn);

    Json(json!({
        "status": if books.is_empty() {404} else {200},
        "result": books,
    }))
}

#[get("/books/<id>", format = "application/json")]
pub fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result: Vec<Book> = Book::show_book_by_id(id, &conn);
    let status: i32 = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": result.get(0)
    }))
}

#[put("/books/<id>", format = "application/json", data = "<book>")]
pub fn update(conn: DbConn, id: i32, book: Json<NewBook>) -> Json<Value> {
    let status: i32 = if Book::update_book_by_id(id, &conn, book.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null
    }))
}

#[put("/books/pub/<id>", format = "application/json", data = "<published>")]
pub fn update_published(conn: DbConn, id: i32, published: Json<BookPublished>) -> Json<Value> {
    let status: i32 = if Book::update_book_published_by_id(id, &conn, published.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null
    }))
}

#[post("/books", format = "application/json", data = "<new_book>")]
pub fn new(conn: DbConn, new_book: Json<NewBook>) -> Json<Value> {
    Json(json!({
        "status": Book::insert_book(new_book.into_inner(), &conn),
        "result": Book::all_books(&conn).first()
    }))
}

#[delete("/books/<id>")]
pub fn delete(conn: DbConn, id: i32) -> Json<Value> {
    let status: i32 = if Book::delete_book_by_id(id, &conn) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null
    }))
}

#[get("/books/author/<author>", format = "application/json")]
pub fn author(conn: DbConn, author: String) -> Json<Value> {
    let result: Vec<Book> = Book::all_by_author(author, &conn);

    Json(json!({
        "status": if result.is_empty() {404} else {200},
        "result": result
    }))
}
