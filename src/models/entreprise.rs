use diesel::{prelude::*, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::db::Connection,
    schema::entreprise::{self, dsl::*},
};

#[derive(Identifiable, Queryable, Serialize, Selectable, Deserialize)]
#[diesel(table_name = entreprise)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Entreprise {
    pub id: Uuid,
    pub name: String,
}

#[derive(Insertable, Queryable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = entreprise)]
pub struct EntrepriseDTO {
    pub name: String,
}

impl Entreprise {
    pub fn find_all(conn: &mut Connection) -> QueryResult<Vec<Entreprise>> {
        entreprise.load::<Entreprise>(conn)
    }

    pub fn find_by_id(i: Uuid, conn: &mut Connection) -> QueryResult<Entreprise> {
        entreprise.find(i).get_result::<Entreprise>(conn)
    }

    pub fn find_entrprise_by_name(nm: &str, conn: &mut Connection) -> QueryResult<Entreprise> {
        entreprise
            .filter(name.eq(nm))
            .get_result::<Entreprise>(conn)
    }

    pub fn insert(new_entreprise: EntrepriseDTO, conn: &mut Connection) -> QueryResult<usize> {
        diesel::insert_into(entreprise)
            .values(&new_entreprise)
            .execute(conn)
    }

    pub fn update(
        i: Uuid,
        updated_entreprise: EntrepriseDTO,
        conn: &mut Connection,
    ) -> QueryResult<usize> {
        diesel::update(entreprise.find(i))
            .set(&updated_entreprise)
            .execute(conn)
    }

    pub fn delete(i: Uuid, conn: &mut Connection) -> QueryResult<usize> {
        diesel::delete(entreprise.find(i)).execute(conn)
    }
}
