pub mod id;

use crate::{Result, entities::user::id::UserIdAttrs};
use chrono::{DateTime, Utc};
use id::UserId;
use identify_macros::gen_model;
use uuid::Uuid;

gen_model! {
    #[derive(Debug, Clone)]
    pub struct User {
        /// A stable deterministic ID for this user.
        #[new(skip)]
        #[hydrate(type(Uuid))]
        id: UserId,
        /// User's first name.
        first_name: String,
        /// User's last name.
        last_name: Option<String>,
        /// Password hash.
        ///
        /// Having a password is optional in the system, hence the [Option].
        password_hash: Option<String>,
        #[get(copy)]
        created_at: DateTime<Utc>,
        #[get(copy)]
        #[new(skip)]
        updated_at: DateTime<Utc>,
    }

    #[derive(Debug)]
    pub struct NewUserAttrs {
        /// Email of the user that uniquely identifies them within the system.
        email: String,
    };

    #[derive(Debug)]
    pub struct UserAttrs {
        /// Email of the user that uniquely identifies them within the system.
        email: String,
    };
}

impl User {
    pub fn new(attrs: NewUserAttrs) -> Self {
        User {
            id: UserId::new(UserIdAttrs { email: attrs.email }),
            first_name: attrs.first_name,
            last_name: attrs.last_name,
            password_hash: attrs.password_hash,
            created_at: attrs.created_at,
            updated_at: attrs.created_at,
        }
    }

    pub fn load(attrs: UserAttrs) -> Result<Self> {
        Ok(User {
            id: UserId::load(UserIdAttrs { email: attrs.email }, attrs.id)?,
            first_name: attrs.first_name,
            last_name: attrs.last_name,
            password_hash: attrs.password_hash,
            created_at: attrs.created_at,
            updated_at: attrs.updated_at,
        })
    }

    pub fn to_attributes(&self) -> UserAttrs {
        UserAttrs {
            id: self.id().to_uuid(),
            email: self.id.email().to_owned(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            password_hash: self.password_hash.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
