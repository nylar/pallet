use crate::error::Error;
use crate::models::{krate::Krate, owner::Owner};
use crate::schema::krateowner;

use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Associations, Queryable)]
#[belongs_to(parent = "Krate")]
#[belongs_to(parent = "Owner")]
#[table_name = "krateowner"]
pub struct KrateOwner {
    pub krate_id: i32,
    pub owner_id: i32,
}

impl KrateOwner {
    pub fn crate_permission(
        conn: &PgConnection,
        krate_id: i32,
        owner_id: i32,
    ) -> Result<bool, Error> {
        let result = krateowner::table
            .filter(krateowner::krate_id.eq(krate_id))
            .filter(krateowner::owner_id.eq(owner_id))
            .first::<KrateOwner>(conn);

        match result {
            Ok(_) => Ok(true),
            Err(diesel::result::Error::NotFound) => Ok(false),
            Err(err) => Err(Error::DB(err)),
        }
    }

    pub fn remove_owner(conn: &PgConnection, krate_id: i32, owner_id: i32) -> Result<(), Error> {
        diesel::delete(
            krateowner::table
                .filter(krateowner::krate_id.eq(krate_id))
                .filter(krateowner::owner_id.eq(owner_id)),
        )
        .execute(conn)?;

        Ok(())
    }
}

#[derive(Insertable)]
#[table_name = "krateowner"]
pub struct NewKrateOwner {
    pub krate_id: i32,
    pub owner_id: i32,
}

impl NewKrateOwner {
    pub fn save(&self, conn: &PgConnection) -> Result<KrateOwner, Error> {
        diesel::insert_into(krateowner::table)
            .values(self)
            .get_result(conn)
            .map_err(Error::DB)
    }
}
