use diesel::{prelude::*, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{config::db::Connection,schema::users::{self, dsl::*},};

#[derive(Identifiable, Queryable, Serialize, Selectable, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub role: String,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub role: String,
}

impl User {

    pub fn get_superadmin_user(conn: &mut Connection) -> bool {
        users
            .filter(username.eq("superadmin"))
            .select(username)
            .first::<String>(conn)
            .is_ok()
    }

    pub fn insert(new_user: UserDTO, conn: &mut Connection) -> QueryResult<usize> {
        diesel::insert_into(users)
            .values(&new_user)
            .execute(conn)
    }

    /*TODO
        UPDATE
        DELETE
     */
}