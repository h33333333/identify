pub mod id;

use crate::Result;

use chrono::{DateTime, Utc};
use id::{GroupId, GroupIdAttrs};
use identify_macros::gen_model;
use uuid::Uuid;

gen_model! {
    #[derive(Debug, Clone)]
    pub struct Group {
        /// A stable deterministic ID for this group.
        #[new(skip)]
        #[hydrate(type(Uuid))]
        id: GroupId,

        /// Whether members of this group have privileged access to the system or not.
        ///
        /// Privileged access allows adding new users, groups, and configuring applications.
        #[get(copy)]
        is_privileged: bool,

        #[get(copy)]
        created_at: DateTime<Utc>,
        #[get(copy)]
        #[new(skip)]
        updated_at: DateTime<Utc>,
    }

    #[derive(Debug)]
    pub struct NewGroupAttrs {
        /// Group name.
        ///
        /// Should be unique within the system.
        name: String,
    };

    #[derive(Debug)]
    pub struct GroupAttrs {
        /// Group name.
        ///
        /// Should be unique within the system.
        name: String,
    };
}

impl Group {
    pub fn new(attrs: NewGroupAttrs) -> Self {
        Group {
            id: GroupId::new(GroupIdAttrs { name: attrs.name }),
            is_privileged: attrs.is_privileged,
            created_at: attrs.created_at,
            updated_at: attrs.created_at,
        }
    }

    pub fn load(attrs: GroupAttrs) -> Result<Self> {
        Ok(Group {
            id: GroupId::load(GroupIdAttrs { name: attrs.name }, attrs.id)?,
            is_privileged: attrs.is_privileged,
            created_at: attrs.created_at,
            updated_at: attrs.updated_at,
        })
    }

    pub fn to_attributes(&self) -> GroupAttrs {
        GroupAttrs {
            id: self.id().to_uuid(),
            name: self.id().name().to_owned(),
            is_privileged: self.is_privileged(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
