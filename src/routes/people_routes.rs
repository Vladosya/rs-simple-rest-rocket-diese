use rocket::{delete, get, post, put};
use rocket_contrib::json::Json;
use serde_json::Value;

use crate::db::Conn as DbConn;
use crate::people::models::{NewPerson, People, UpdateName};

#[get("/people", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let books: Vec<People> = People::all_people(&conn);

    Json(json!({
        "status": 200,
        "result": books
    }))
}

#[get("/people/<id>", format = "application/json")]
pub fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result: Vec<People> = People::show_person_by_id(id, &conn);
    let status: i32 = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": result.get(0)
    }))
}

#[put("/people/<id>", format = "application/json", data = "<person>")]
pub fn update(conn: DbConn, id: i32, person: Json<NewPerson>) -> Json<Value> {
    let status: i32 = if People::update_person_by_id(id, &conn, person.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null
    }))
}

#[put("/people/name/<id>", format = "application/json", data = "<need_name>")]
pub fn update_name(conn: DbConn, id: i32, need_name: Json<UpdateName>) -> Json<Value> {
    let status: i32 = if People::update_person_name_by_id(id, &conn, need_name.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null
    }))
}

#[post("/people", format = "application/json", data = "<new_person>")]
pub fn new(conn: DbConn, new_person: Json<NewPerson>) -> Json<Value> {
    Json(json!({
        "status": People::insert_person(new_person.into_inner(), &conn),
        "result": People::all_people(&conn).first()
    }))
}

#[delete("/people/<id>")]
pub fn delete(conn: DbConn, id: i32) -> Json<Value> {
    let status: i32 = if People::delete_person_by_id(id, &conn) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null
    }))
}

#[get("/people/country/<country>")]
pub fn country(conn: DbConn, country: String) -> Json<Value> {
    let result: Vec<People> = People::all_person_by_country(country, &conn);

    Json(json!({
        "status": if result.is_empty() {404} else {200},
        "result": result,
    }))
}