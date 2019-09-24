use std::{error, fmt, io::Write, ops::Deref, str::FromStr};

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Text;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// https://doc.rust-lang.org/grammar.html#keywords
const BLACKLIST: &[&str] = &[
    "abstract", "alignof", "as", "become", "box", "break", "const", "continue", "crate", "do",
    "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in", "let", "loop",
    "macro", "match", "mod", "move", "mut", "offsetof", "override", "priv", "proc", "pub", "pure",
    "ref", "return", "self", "sizeof", "static", "struct", "super", "test", "trait", "true",
    "type", "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

// Crates.io reserved crate names
const RESERVED: &[&str] = &[
    "alloc",
    "arena",
    "ast",
    "builtins",
    "collections",
    "compiler-builtins",
    "compiler-rt",
    "compiletest",
    "core",
    "coretest",
    "debug",
    "driver",
    "flate",
    "fmt_macros",
    "grammar",
    "graphviz",
    "macro",
    "macros",
    "proc_macro",
    "rbml",
    "rust-installer",
    "rustbook",
    "rustc",
    "rustc_back",
    "rustc_borrowck",
    "rustc_driver",
    "rustc_llvm",
    "rustc_resolve",
    "rustc_trans",
    "rustc_typeck",
    "rustdoc",
    "rustllvm",
    "rustuv",
    "serialize",
    "std",
    "syntax",
    "test",
    "unicode",
];

#[derive(Debug, PartialEq, AsExpression, FromSqlRow, Clone)]
#[sql_type = "Text"]
pub struct CrateName(String);

impl Deref for CrateName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for CrateName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl FromStr for CrateName {
    type Err = CrateNameError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        if name == "" {
            Err(CrateNameError::Empty)?;
        }

        let name = name.to_lowercase();

        if BLACKLIST.contains(&name.as_ref()) {
            Err(CrateNameError::Blacklisted)?;
        }

        if RESERVED.contains(&name.as_ref()) {
            Err(CrateNameError::Reserved)?;
        }

        if let Some(_) = name
            .chars()
            .find(|ch| !ch.is_alphanumeric() && *ch != '_' && *ch != '-')
        {
            Err(CrateNameError::NonAlphaNumeric)?;
        }

        Ok(CrateName(name))
    }
}

impl Serialize for CrateName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl ToSql<Text, Pg> for CrateName {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        ToSql::<Text, Pg>::to_sql(&self.0, out)
    }
}

impl<'de> Deserialize<'de> for CrateName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(CrateNameVisitor)
    }
}

struct CrateNameVisitor;

impl<'de> Visitor<'de> for CrateNameVisitor {
    type Value = CrateName;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a crate name")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        CrateName::from_str(value).map_err(|err| E::custom(err.to_string()))
    }
}

impl FromSql<Text, Pg> for CrateName {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        FromSql::<Text, Pg>::from_sql(bytes).map(CrateName)
    }
}

#[derive(Debug)]
pub enum CrateNameError {
    Empty,
    NonAlphaNumeric,
    Blacklisted,
    Reserved,
}

impl fmt::Display for CrateNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CrateNameError::Empty => write!(f, "crate name is empty"),
            CrateNameError::NonAlphaNumeric => write!(
                f,
                "Crate name must contain only alphanumeric characters characters or - or _"
            ),
            CrateNameError::Blacklisted => write!(f, "Crate name is blacklisted"),
            CrateNameError::Reserved => write!(f, "Crate name is reserved"),
        }
    }
}

impl error::Error for CrateNameError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crate_name_from_string() {
        let valid_cases = vec!["tokio", "serde_json"];
        let invalid_cases = vec!["", "$foo", "enum", "alloc"];

        valid_cases.iter().for_each(|case| {
            assert!(CrateName::from_str(case).is_ok());
        });

        invalid_cases.iter().for_each(|case| {
            assert!(CrateName::from_str(case).is_err());
        });
    }
}
