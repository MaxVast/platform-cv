use diesel::{prelude::*, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::db::Connection,
    schema::candidate::{self, dsl::*},
};

#[derive(Identifiable, Queryable, Serialize, Selectable, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = candidate)]
pub struct Candidate {
    pub id: Uuid,
    pub company_id: Uuid,
    pub lastname: String,
    pub firstname: String,
    pub file_name: String,
    pub phone: String,
    pub email: String,
    pub motivation: String,
}

#[derive(Insertable, Queryable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = candidate)]
pub struct CandidateDTO {
    pub company_id: Uuid,
    pub lastname: String,
    pub firstname: String,
    pub file_name: String,
    pub phone: String,
    pub email: String,
    pub motivation: String,
}

impl Candidate {
    pub fn find_all(conn: &mut Connection) -> QueryResult<Vec<Candidate>> {
        candidate.load::<Candidate>(conn)
    }

    pub fn find_by_id(i: Uuid, conn: &mut Connection) -> QueryResult<Candidate> {
        candidate.find(i).get_result::<Candidate>(conn)
    }

    pub fn find_by_company_id(
        i_company: Uuid,
        conn: &mut Connection,
    ) -> QueryResult<Vec<Candidate>> {
        candidate
            .filter(company_id.eq(i_company))
            .load::<Candidate>(conn)
    }

    pub fn insert(new_candidate: CandidateDTO, conn: &mut Connection) -> QueryResult<usize> {
        diesel::insert_into(candidate)
            .values(&new_candidate)
            .execute(conn)
    }

    pub fn delete(i: Uuid, conn: &mut Connection) -> QueryResult<usize> {
        diesel::delete(candidate.find(i)).execute(conn)
    }
}
