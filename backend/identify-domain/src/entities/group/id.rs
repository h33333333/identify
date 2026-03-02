use identify_macros::gen_id;
use uuid::Uuid;

use crate::entities::UUID_NAMESPACE;
use crate::{DomainError, Result};

gen_id! {
    UUID_NAMESPACE,
    /// A stable and deterministic ID that uniquely identifies a [Group](super::Group) within the system.
    #[derive(Debug, Clone)]
    pub struct GroupId {
        /// Group name.
        name: String,
    }

    #[derive(Debug)]
    pub struct GroupIdAttrs;
}

impl GroupId {
    pub fn new(attrs: GroupIdAttrs) -> Self {
        GroupId { name: attrs.name }
    }

    pub fn load(attrs: GroupIdAttrs, expected: Uuid) -> Result<Self> {
        let id = GroupId { name: attrs.name };

        let generated = id.to_uuid();

        if generated != expected {
            return Err(DomainError::id_mismatch(
                "GroupId",
                format!("expected {}, got {}", expected, generated),
            ));
        }

        Ok(id)
    }
}
