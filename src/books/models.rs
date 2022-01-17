use diesel;
use diesel::prelude::*;
use diesel::PgConnection;

use crate::schema::books;
use crate::schema::books::dsl::books as all_books;

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Book {
    id: i32,
    title: String,
    author: String,
    published: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "books"]
pub struct NewBook {
    title: String,
    author: String,
    published: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "books"]
pub struct BookPublished {
    published: bool,
}

impl Book {
    pub fn all_books(conn: &PgConnection) -> Vec<Book> {
        all_books
            .order(books::id.desc())
            .load::<Book>(conn)
            .expect("Error loading the books")
    }
    pub fn show_book_by_id(id: i32, conn: &PgConnection) -> Vec<Book> {
        all_books
            .find(id)
            .load::<Book>(conn)
            .expect("Error loading book")
    }
    pub fn update_book_by_id(id: i32, conn: &PgConnection, book: NewBook) -> bool {
        use crate::schema::books::dsl::{author as a, published as p, title as t};
        let NewBook {
            title,
            author,
            published,
        } = book;

        diesel::update(all_books.find(id))
            .set((t.eq(title), a.eq(author), p.eq(published)))
            .get_result::<Book>(conn)
            .is_ok()
    }
    pub fn insert_book(book: NewBook, conn: &PgConnection) -> bool {
        diesel::insert_into(books::table)
            .values(&book)
            .execute(conn)
            .is_ok()
    }
    pub fn delete_book_by_id(id: i32, conn: &PgConnection) -> bool {
        if Book::show_book_by_id(id, &conn).is_empty() {
            return false;
        }

        diesel::delete(all_books.find(id)).execute(conn).is_ok()
    }
    pub fn update_book_published_by_id(
        id: i32,
        conn: &PgConnection,
        published: BookPublished,
    ) -> bool {
        use crate::schema::books::dsl::published as p;
        let BookPublished { published } = published;

        diesel::update(all_books.find(id))
            .set(p.eq(published))
            .get_result::<Book>(conn)
            .is_ok()
    }
    pub fn all_by_author(author: String, conn: &PgConnection) -> Vec<Book> {
        all_books
            .filter(books::author.eq(author))
            .load::<Book>(conn)
            .expect("Error find books by author")
    }
}
