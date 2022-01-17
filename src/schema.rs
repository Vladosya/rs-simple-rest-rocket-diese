table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        published -> Bool,
    }
}

table! {
    people (id) {
        id -> Int4,
        name -> Varchar,
        first_name -> Varchar,
        country -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    books,
    people,
);
