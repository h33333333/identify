pub mod id;

use crate::{Result, entities::user::id::UserIdAttrs};
use chrono::{DateTime, Utc};
use id::UserId;
use identify_macros::gen_model;
use uuid::Uuid;

gen_model! {
    #[derive(Debug)]
    pub struct User {
        /// A stable deterministic ID for this user.
        #[get(ref_into(Uuid))]
        #[new(skip)]
        #[hydrate(type(Uuid))]
        id: UserId,
        /// User's first name.
        first_name: String,
        /// User's last name.
        last_name: Option<String>,
        #[new(skip)]
        created_at: DateTime<Utc>,
        #[new(skip)]
        updated_at: DateTime<Utc>,
    }

    #[derive(Debug)]
    pub struct NewUserAttrs {
        /// Email of the user that uniquely identifies them within the system.
        email: String,
    }

    #[derive(Debug)]
    pub struct UserAttrs {
        /// Email of the user that uniquely identifies them within the system.
        email: String,
    }
}

impl User {
    pub fn new(attrs: NewUserAttrs) -> Self {
        let now = Utc::now();
        User {
            id: UserId::new(UserIdAttrs { email: attrs.email }),
            first_name: attrs.first_name,
            last_name: attrs.last_name,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn load(attrs: UserAttrs) -> Result<Self> {
        Ok(User {
            id: UserId::load(UserIdAttrs { email: attrs.email }, attrs.id)?,
            first_name: attrs.first_name,
            last_name: attrs.last_name,
            created_at: attrs.created_at,
            updated_at: attrs.updated_at,
        })
    }

    pub fn to_attributes(&self) -> UserAttrs {
        UserAttrs {
            id: self.id(),
            email: self.id.email().to_owned(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
