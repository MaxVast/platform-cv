use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::{
    deserialize::{self, FromSql},
    pg::{Pg, PgValue},
    prelude::*,
    result::Error as DieselError,
    serialize::{self, IsNull, Output, ToSql},
    sql_types::Varchar,
    AsExpression, FromSqlRow, Identifiable, Insertable, Queryable,
};
use serde::{Deserialize, Serialize};
use std::{fmt, io::Write, str::FromStr};
use uuid::Uuid;

use crate::{
    config::db::Connection,
    constants,
    models::{login_history::LoginHistory, user_token::UserToken},
    schema::users::{self, dsl::*},
};

#[derive(Identifiable, Queryable, Serialize, Selectable, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub company_id: Option<Uuid>,
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub role: RoleType,
    pub login_session: Option<String>,
}

#[derive(Insertable, Queryable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserDTO {
    pub username: String,
    pub company_id: Option<Uuid>,
    pub email: String,
    pub password: Option<String>,
    pub role: RoleType,
    pub login_session: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct LoginInfoDTO {
    pub username: String,
    pub login_session: String,
}

impl User {
    pub fn signup(new_user: UserDTO, conn: &mut Connection) -> Result<String, String> {
        match Self::find_user_by_username(&new_user.username, conn) {
            Ok(_) => Err(format!(
                "User '{}' is already registered",
                &new_user.username
            )),
            Err(_) => {
                if let Some(password_clone) = new_user.password.clone() {
                    match hash(password_clone, DEFAULT_COST) {
                        Ok(password_hash) => {
                            let new_user = UserDTO {
                                password: Some(password_hash),
                                ..new_user
                            };
                            match Self::insert(new_user, conn) {
                                Ok(_) => Ok(constants::MESSAGE_SIGNUP_SUCCESS.to_string()),
                                Err(e) => Err(format!("Failed to insert user: {}", e)),
                            }
                        }
                        Err(e) => Err(format!("Failed to hash password: {}", e)),
                    }
                } else {
                    Err("Password is required".to_string())
                }
            }
        }
    }

    pub fn login(login: LoginDTO, conn: &mut Connection) -> Option<LoginInfoDTO> {
        if let Ok(user_to_verify) = users
            .filter(username.eq(&login.username_or_email))
            .or_filter(email.eq(&login.username_or_email))
            .get_result::<User>(conn)
        {
            if !user_to_verify.password.clone()?.is_empty()
                && verify(&login.password, &user_to_verify.password?.to_string()).unwrap()
            {
                if let Some(login_history) = LoginHistory::create(&user_to_verify.username, conn) {
                    if LoginHistory::save_login_history(login_history, conn).is_err() {
                        return None;
                    }
                    let login_session_str = User::generate_login_session();
                    if User::update_login_session_to_db(
                        &user_to_verify.username,
                        &login_session_str,
                        conn,
                    ) {
                        return Some(LoginInfoDTO {
                            username: user_to_verify.username,
                            login_session: login_session_str,
                        });
                    }
                }
            } else {
                return Some(LoginInfoDTO {
                    username: user_to_verify.username,
                    login_session: String::new(),
                });
            }
        }

        None
    }

    pub fn logout(user_id: Uuid, conn: &mut Connection) {
        if let Ok(user) = users.find(user_id).get_result::<User>(conn) {
            Self::update_login_session_to_db(&user.username, "", conn);
        }
    }

    pub fn update_login_session_to_db(
        un: &str,
        login_session_str: &str,
        conn: &mut Connection,
    ) -> bool {
        if let Ok(user) = User::find_user_by_username(un, conn) {
            diesel::update(users.find(user.id))
                .set(login_session.eq(login_session_str))
                .execute(conn)
                .is_ok()
        } else {
            false
        }
    }

    pub fn is_valid_login_session(user_token: &UserToken, conn: &mut Connection) -> bool {
        users
            .filter(username.eq(&user_token.user))
            .filter(login_session.eq(&user_token.login_session))
            .get_result::<User>(conn)
            .is_ok()
    }

    pub fn find_login_info_by_token(
        user_token: &UserToken,
        conn: &mut Connection,
    ) -> Result<LoginInfoDTO, String> {
        let user_result = users
            .filter(username.eq(&user_token.user))
            .filter(login_session.eq(&user_token.login_session))
            .first::<User>(conn);

        match user_result {
            Ok(user) => {
                let login_session_data = user.login_session.ok_or("Login session is missing")?;
                Ok(LoginInfoDTO {
                    username: user.username,
                    login_session: login_session_data,
                })
            }
            Err(DieselError::NotFound) => Err("User not found".to_string()),
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_string()
    }

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
        diesel::insert_into(users).values(&new_user).execute(conn)
    }

    pub fn update(i: Uuid, updated_user: UserDTO, conn: &mut Connection) -> QueryResult<usize> {
        diesel::update(users.find(i))
            .set(&updated_user)
            .execute(conn)
    }

    pub fn delete(i: Uuid, conn: &mut Connection) -> QueryResult<usize> {
        diesel::delete(users.find(i)).execute(conn)
    }

    #[allow(dead_code)]
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
        write!(
            f,
            "{}",
            match self {
                RoleType::SuperAdmin => "superadmin",
                RoleType::Admin => "admin",
                RoleType::User => "user",
            }
        )
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
