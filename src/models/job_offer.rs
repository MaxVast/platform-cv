use chrono::{NaiveDateTime, Utc};
use diesel::{prelude::*, Identifiable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::db::Connection,
    schema::job_offers::{self, dsl::*},
};

#[derive(Identifiable, Queryable, Serialize, Selectable, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct JobOffer {
    pub id: Uuid,
    pub company_id: Uuid,
    pub title: String,
    pub description: String,
    pub requirements: Option<String>,
    pub location: String,
    pub remote: Option<String>,
    pub employment_type: String,
    pub salary: f32,
    pub created_at: NaiveDateTime,
    #[diesel(sql_type = Nullable<Date>)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Queryable, Serialize, Deserialize, AsChangeset)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = job_offers)]
pub struct JobOfferDTO {
    pub company_id: Uuid,
    pub title: String,
    pub description: String,
    pub requirements: Option<String>,
    pub location: String,
    pub remote: Option<String>,
    pub employment_type: String,
    pub salary: f32,
    pub created_at: NaiveDateTime,
    #[diesel(sql_type = Nullable<Date>)]
    pub updated_at: Option<NaiveDateTime>,
}

impl JobOffer {
    pub fn find_all(conn: &mut Connection) -> QueryResult<Vec<JobOffer>> {
        job_offers.load::<JobOffer>(conn)
    }

    pub fn find_by_id(i: Uuid, conn: &mut Connection) -> QueryResult<JobOffer> {
        job_offers.find(i).get_result::<JobOffer>(conn)
    }

    pub fn find_by_company_id(
        i_company: Uuid,
        conn: &mut Connection,
    ) -> QueryResult<Vec<JobOffer>> {
        job_offers
            .filter(company_id.eq(i_company))
            .load::<JobOffer>(conn)
    }

    pub fn find_one_by_company_id(
        i: Uuid,
        i_company: Uuid,
        conn: &mut Connection,
    ) -> QueryResult<JobOffer> {
        job_offers
            .find(i)
            .filter(company_id.eq(i_company))
            .get_result::<JobOffer>(conn)
    }

    pub fn find_by_location(
        location_data: &str,
        conn: &mut Connection,
    ) -> QueryResult<Vec<JobOffer>> {
        job_offers
            .filter(location.ilike(location_data))
            .load::<JobOffer>(conn)
    }

    pub fn insert(mut new_job_offer: JobOfferDTO, conn: &mut Connection) -> QueryResult<usize> {
        let now = Utc::now();
        new_job_offer.created_at = now.naive_utc();
        diesel::insert_into(job_offers)
            .values(&new_job_offer)
            .execute(conn)
    }

    pub fn update(
        i: Uuid,
        mut updated_job_offer: JobOfferDTO,
        conn: &mut Connection,
    ) -> QueryResult<usize> {
        let now = Utc::now();
        updated_job_offer.updated_at = Option::from(now.naive_utc());
        diesel::update(job_offers.find(i))
            .set(&updated_job_offer)
            .execute(conn)
    }

    pub fn delete(i: Uuid, conn: &mut Connection) -> QueryResult<usize> {
        diesel::delete(job_offers.find(i)).execute(conn)
    }
}
