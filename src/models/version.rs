use crate::error::Error;
use crate::models::krate::Krate;
use crate::schema::version;

use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(AsChangeset, Associations, Debug, Identifiable, Queryable)]
#[belongs_to(parent = "Krate")]
#[table_name = "version"]
pub struct Version {
    pub id: i32,
    pub krate_id: i32,
    pub vers: String,
    pub yanked: bool,
}

impl Version {
    pub fn by_crate_id_and_version(
        conn: &PgConnection,
        krate_id: i32,
        vers: &str,
    ) -> Result<Option<Self>, Error> {
        let result = version::table
            .filter(version::krate_id.eq(krate_id))
            .filter(version::vers.eq(vers))
            .first::<Version>(conn);

        match result {
            Ok(v) => Ok(Some(v)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(err) => Err(Error::DB(err)),
        }
    }

    pub fn set_yanked(&self, conn: &PgConnection, yanked: bool) -> Result<(), Error> {
        let yanked_version = YankedVersion {
            id: self.id,
            yanked,
        };

        yanked_version
            .save_changes::<Version>(conn)
            .map_err(Error::DB)?;
        Ok(())
    }
}

#[derive(Insertable)]
#[table_name = "version"]
pub struct NewVersion<'a> {
    pub krate_id: i32,
    pub vers: &'a str,
    pub yanked: bool,
}

impl<'a> NewVersion<'a> {
    pub fn save(&self, conn: &PgConnection) -> Result<Version, Error> {
        diesel::insert_into(version::table)
            .values(self)
            .get_result(conn)
            .map_err(Error::DB)
    }
}

#[derive(AsChangeset, Identifiable)]
#[table_name = "version"]
pub struct YankedVersion {
    pub id: i32,
    pub yanked: bool,
}
