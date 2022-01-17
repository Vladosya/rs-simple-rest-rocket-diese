use diesel;
use diesel::PgConnection;
use diesel::prelude::*;

use crate::schema::people;
use crate::schema::people::dsl::people as all_people;

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct People {
    id: i32,
    name: String,
    first_name: String,
    country: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "people"]
pub struct NewPerson {
    name: String,
    first_name: String,
    country: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "people"]
pub struct UpdateName {
    name: String,
}

impl People {
    pub fn all_people(conn: &PgConnection) -> Vec<People> {
        all_people
            .order(people::id.desc())
            .load::<People>(conn)
            .expect("Error loading people")
    }
    pub fn show_person_by_id(id: i32, conn: &PgConnection) -> Vec<People> {
        all_people
            .find(id)
            .load::<People>(conn)
            .expect("Error loading person")
    }
    pub fn update_person_by_id(id: i32, conn: &PgConnection, person: NewPerson) -> bool {
        use crate::schema::people::dsl::{name as n, first_name as f, country as c};
        let NewPerson { name, first_name, country } = person;

        diesel::update(all_people.find(id))
            .set((n.eq(name), f.eq(first_name), c.eq(country)))
            .get_result::<People>(conn)
            .is_ok()
    }
    pub fn update_person_name_by_id(id: i32, conn: &PgConnection, need_name: UpdateName) -> bool {
        use crate::schema::people::dsl::{name as n};
        let UpdateName { name } = need_name;

        diesel::update(all_people.find(id))
            .set(n.eq(name))
            .get_result::<People>(conn)
            .is_ok()
    }
    pub fn insert_person(person: NewPerson, conn: &PgConnection) -> bool {
        diesel::insert_into(people::table)
            .values(&person)
            .execute(conn)
            .is_ok()
    }
    pub fn delete_person_by_id(id: i32, conn: &PgConnection) -> bool {
        if People::show_person_by_id(id, &conn).is_empty() {
            return false;
        }

        diesel::delete(all_people.find(id))
            .execute(conn)
            .is_ok()
    }
    pub fn all_person_by_country(person: String, conn: &PgConnection) -> Vec<People> {
        all_people
            .filter(people::country.eq(person))
            .load::<People>(conn)
            .expect("Error find people by country")
    }
}
















