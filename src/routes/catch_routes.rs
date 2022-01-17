use rocket::catch;
use rocket_contrib::json::Json;
use serde_json::Value;

#[catch(404)]
pub fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "result": "Resource was not found"
    }))
}