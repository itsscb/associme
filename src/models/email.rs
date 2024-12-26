use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(String);

impl<T: AsRef<str>> From<T> for Email {
    fn from(s: T) -> Self {
        Self(s.as_ref().to_string())
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// impl<'r> FromRow<'r, sqlx::postgres::PgRow> for Email {
//     fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
//         Ok(Self(row.try_get("email")?))
//     }
// }
