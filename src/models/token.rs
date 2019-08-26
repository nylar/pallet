use crate::error::Error;
use crate::models::owner::Owner;
use crate::schema::token;

use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Associations, Debug, Identifiable, Queryable)]
#[belongs_to(parent = "Owner")]
#[table_name = "token"]
pub struct Token {
    pub id: i32,
    pub owner_id: i32,
    pub name: String,
    pub api_token: String,
    pub created_at: NaiveDateTime,
}

impl Token {
    pub fn by_token(conn: &PgConnection, token: &str) -> Result<Option<Token>, Error> {
        let result = token::table
            .filter(token::api_token.eq(token))
            .first::<Token>(conn);

        match result {
            Ok(t) => Ok(Some(t)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(err) => Err(Error::DB(err)),
        }
    }

    pub fn owner(&self, conn: &PgConnection) -> Result<Option<Owner>, Error> {
        use crate::schema::owner;

        let result = owner::table
            .inner_join(token::table.on(token::owner_id.eq(owner::id)))
            .filter(token::owner_id.eq(self.id))
            .select((owner::id, owner::login, owner::name))
            .first::<Owner>(conn);

        match result {
            Ok(o) => Ok(Some(o)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(err) => Err(Error::DB(err)),
        }
    }
}

#[derive(Insertable)]
#[table_name = "token"]
pub struct NewToken<'a> {
    pub owner_id: i32,
    pub name: &'a str,
    pub api_token: &'a str,
    pub created_at: NaiveDateTime,
}

impl<'a> NewToken<'a> {
    pub fn save(&self, conn: &PgConnection) -> Result<Token, Error> {
        diesel::insert_into(token::table)
            .values(self)
            .get_result(conn)
            .map_err(Error::DB)
    }
}
