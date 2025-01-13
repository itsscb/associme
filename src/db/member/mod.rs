mod create;

pub use create::create;
mod delete;
pub use delete::delete_by_id;
mod get;
pub use get::get_by_id;
mod list;
pub use list::list;
mod update;
pub use update::{update, UpdateMember};

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct NewMember {
    pub email: String,
    pub phone: String,
    pub first_name: String,
    pub last_name: String,
    pub member_id: Option<i32>,
    pub birthday: DateTime<Utc>,
    pub postalcode: String,
    pub city: String,
    pub street: String,
    pub house_number: String,
    pub membership_state: crate::models::member::Membership,
}

impl NewMember {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        email: String,
        phone: String,
        first_name: String,
        last_name: String,
        member_id: Option<i32>,
        birthday: DateTime<Utc>,
        postalcode: String,
        city: String,
        street: String,
        house_number: String,
        membership_state: crate::models::member::Membership,
    ) -> Self {
        Self {
            phone,
            email,
            first_name,
            last_name,
            member_id,
            birthday,
            postalcode,
            city,
            street,
            house_number,
            membership_state,
        }
    }
}
