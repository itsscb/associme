use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Role {
    Admin,
    User,
    Treasurer,
    Secretary,
    President,
    VicePresident,
}

impl Display for Role {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let out = match self {
            Self::Admin => "admin",
            Self::User => "user",
            Self::Treasurer => "treasurer",
            Self::Secretary => "secretary",
            Self::President => "president",
            Self::VicePresident => "vice_president",
        };
        write!(f, "{out:?}")
    }
}

impl Default for Role {
    fn default() -> Self {
        Self::User
    }
}

impl<T: AsRef<str>> From<T> for Role {
    fn from(s: T) -> Self {
        match s.as_ref().to_lowercase().as_str() {
            "admin" => Self::Admin,
            "treasurer" => Self::Treasurer,
            "secretary" => Self::Secretary,
            "president" => Self::President,
            "vice_president" => Self::VicePresident,
            _ => Self::User,
        }
    }
}
