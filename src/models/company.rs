use diesel::{prelude::*, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::db::Connection,
    schema::company::{self, dsl::*},
};

#[derive(Identifiable, Queryable, Serialize, Selectable, Deserialize)]
#[diesel(table_name = company)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Company {
    pub id: Uuid,
    pub name: String,
}

#[derive(Insertable, Queryable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = company)]
pub struct CompanyDTO {
    pub name: String,
}

impl Company {
    pub fn find_all(conn: &mut Connection) -> QueryResult<Vec<Company>> {
        company.load::<Company>(conn)
    }

    pub fn find_by_id(i: Uuid, conn: &mut Connection) -> QueryResult<Company> {
        company.find(i).get_result::<Company>(conn)
    }

    pub fn find_entrprise_by_name(nm: &str, conn: &mut Connection) -> QueryResult<Company> {
        company.filter(name.eq(nm)).get_result::<Company>(conn)
    }

    pub fn insert(new_company: CompanyDTO, conn: &mut Connection) -> QueryResult<usize> {
        diesel::insert_into(company)
            .values(&new_company)
            .execute(conn)
    }

    pub fn update(
        i: Uuid,
        updated_company: CompanyDTO,
        conn: &mut Connection,
    ) -> QueryResult<usize> {
        diesel::update(company.find(i))
            .set(&updated_company)
            .execute(conn)
    }

    pub fn delete(i: Uuid, conn: &mut Connection) -> QueryResult<usize> {
        diesel::delete(company.find(i)).execute(conn)
    }
}
