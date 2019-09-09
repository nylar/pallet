use std::fmt;

use crate::error::Error;
use crate::schema::owner;

use diesel::pg::expression::dsl::any;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Identifiable, Queryable, Serialize)]
#[table_name = "owner"]
pub struct Owner {
    pub id: i32,
    pub login: String,
    pub name: Option<String>,
}

impl Owner {
    pub fn by_login(conn: &PgConnection, login: &str) -> Result<Self, Error> {
        owner::table
            .filter(owner::login.eq(login))
            .first::<Owner>(conn)
            .map_err(Error::DB)
    }

    pub fn by_id(conn: &PgConnection, id: i32) -> Result<Self, Error> {
        owner::table
            .filter(owner::id.eq(id))
            .first::<Owner>(conn)
            .map_err(Error::DB)
    }

    pub fn ids_by_logins(conn: &PgConnection, logins: &[String]) -> Result<Vec<i32>, Error> {
        owner::table
            .select(owner::id)
            .filter(owner::login.eq(any(logins)))
            .load(conn)
            .map_err(Error::DB)
    }
}

impl fmt::Display for Owner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.login, self.id)
    }
}

#[derive(Insertable)]
#[table_name = "owner"]
pub struct NewOwner<'a> {
    pub login: &'a str,
    pub name: Option<&'a str>,
}

impl<'a> NewOwner<'a> {
    pub fn save(&self, conn: &PgConnection) -> Result<Owner, Error> {
        diesel::insert_into(owner::table)
            .values(self)
            .get_result(conn)
            .map_err(Error::DB)
    }
}
