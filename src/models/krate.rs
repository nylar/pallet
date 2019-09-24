use crate::error::Error;
use crate::models::owner::Owner;
use crate::schema::krate;
use crate::types::CrateName;

use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Debug, Identifiable, Queryable)]
#[table_name = "krate"]
pub struct Krate {
    pub id: i32,
    pub name: CrateName,
    pub description: Option<String>,
}

impl Krate {
    pub fn by_name(conn: &PgConnection, name: &str) -> Result<Option<Self>, Error> {
        let result = krate::table
            .filter(krate::name.eq(name))
            .first::<Krate>(conn);

        match result {
            Ok(k) => Ok(Some(k)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(err) => Err(Error::DB(err)),
        }
    }

    pub fn owners(&self, conn: &PgConnection) -> Result<Vec<Owner>, Error> {
        use crate::schema::{krateowner, owner};

        owner::table
            .inner_join(krateowner::table.on(krateowner::owner_id.eq(owner::id)))
            .filter(krateowner::krate_id.eq(self.id))
            .select((owner::id, owner::login, owner::name))
            .load::<Owner>(conn)
            .map_err(Error::DB)
    }
}

#[derive(Insertable)]
#[table_name = "krate"]
pub struct NewKrate<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
}

impl<'a> NewKrate<'a> {
    pub fn save(&self, conn: &PgConnection) -> Result<Krate, Error> {
        diesel::insert_into(krate::table)
            .values(self)
            .get_result(conn)
            .map_err(Error::DB)
    }
}
