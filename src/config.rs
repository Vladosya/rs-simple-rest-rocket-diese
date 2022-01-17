use std::collections::HashMap;
use std::env;

use dotenv::dotenv;
use rocket::Config;
use rocket::config::{Environment, Value};

pub fn from_env() -> Config {
    dotenv().ok();
    let environment: Environment = Environment::active().expect("No environment found");

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment variable should parse to an integer");

    let mut database_config: HashMap<&str, Value> = HashMap::new();
    let mut databases: HashMap<&str, Value> = HashMap::new();
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL environment variable found");
    database_config.insert("url", Value::from(database_url));
    databases.insert("diesel_postgres_pool", Value::from(database_config));

    Config::build(environment)
        .environment(environment)
        .port(port)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}