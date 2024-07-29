use diesel::{prelude::*, Identifiable, Insertable, Queryable, AsExpression, FromSqlRow, serialize::{self, IsNull, Output, ToSql}, deserialize::{self, FromSql}, pg::{Pg, PgValue}, sql_types::Varchar};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::{io::Write, str::FromStr, fmt};

use crate::{
    config::db::Connection,
    schema::users::{self, dsl::*},
};

#[derive(Identifiable, Queryable, Serialize, Selectable, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub entreprise_id: Option<Uuid>,
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub role: RoleType,
}

#[derive(Insertable, Queryable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserDTO {
    pub username: String,
    pub entreprise_id: Option<Uuid>,
    pub email: String,
    pub password: Option<String>,
    pub role: RoleType,
}

impl User {
    pub fn find_all(conn: &mut Connection) -> QueryResult<Vec<User>> {
        users.load::<User>(conn)
    }

    pub fn find_by_id(i: Uuid, conn: &mut Connection) -> QueryResult<User> {
        users.find(i).get_result::<User>(conn)
    }

    pub fn find_user_by_username(un: &str, conn: &mut Connection) -> QueryResult<User> {
        users.filter(username.eq(un)).get_result::<User>(conn)
    }

    pub fn get_superadmin_user(conn: &mut Connection) -> bool {
        users
            .filter(username.eq(RoleType::SuperAdmin))
            .select(username)
            .first::<String>(conn)
            .is_ok()
    }

    pub fn insert(new_user: UserDTO, conn: &mut Connection) -> QueryResult<usize> {
        diesel::insert_into(users)
            .values(&new_user)
            .execute(conn)
    }

    pub fn update(i: Uuid, updated_user: UserDTO, conn: &mut Connection) -> QueryResult<usize> {
        diesel::update(users.find(i))
            .set(&updated_user)
            .execute(conn)
    }

    pub fn delete(i: Uuid, conn: &mut Connection) -> QueryResult<usize> {
        diesel::delete(users.find(i)).execute(conn)
    }

    fn validate_role(role_data: &RoleType) -> Result<(), diesel::result::Error> {
        if RoleType::from_str(&role_data.to_string()).is_err() {
            return Err(diesel::result::Error::RollbackTransaction);
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, AsExpression, FromSqlRow)]
#[serde(rename_all = "lowercase")]
#[diesel(sql_type = Varchar)]
pub enum RoleType {
    SuperAdmin,
    Admin,
    User,
}

impl FromStr for RoleType {
    type Err = String;

    fn from_str(s: &str) -> Result<RoleType, Self::Err> {
        match s.to_lowercase().as_str() {
            "superadmin" => Ok(RoleType::SuperAdmin),
            "admin" => Ok(RoleType::Admin),
            "user" => Ok(RoleType::User),
            _ => Err(format!("'{}' is not a valid role", s)),
        }
    }
}

impl fmt::Display for RoleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            RoleType::SuperAdmin => "superadmin",
            RoleType::Admin => "admin",
            RoleType::User => "user",
        })
    }
}

impl ToSql<Varchar, Pg> for RoleType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            RoleType::SuperAdmin => out.write_all(b"superadmin")?,
            RoleType::Admin => out.write_all(b"admin")?,
            RoleType::User => out.write_all(b"user")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Varchar, Pg> for RoleType {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"superadmin" => Ok(RoleType::SuperAdmin),
            b"admin" => Ok(RoleType::Admin),
            b"user" => Ok(RoleType::User),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
