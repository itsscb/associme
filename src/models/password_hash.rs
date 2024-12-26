use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasswordHash(String);

impl<T: AsRef<str>> From<T> for PasswordHash {
    fn from(s: T) -> Self {
        Self(s.as_ref().to_string())
    }
}
// impl<'r> FromRow<'r, sqlx::postgres::PgRow> for PasswordHash {
//     fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
//         Ok(Self(row.try_get("passwordhash")?))
//     }
// }

impl Display for PasswordHash {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
