use chrono::{DateTime, Utc};

/// Implementors of this trait can be updated during their lifecycle.
pub trait Updatable {
    fn get_updated_at_mut(&mut self) -> &mut UpdatedAt;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UpdatedAt {
    Loaded(DateTime<Utc>),
    Updated(DateTime<Utc>),
}

impl UpdatedAt {
    /// Initializes self as [UpdatedAt::Loaded].
    pub fn new(time: DateTime<Utc>) -> Self {
        UpdatedAt::Loaded(time)
    }

    /// Returns `true` if this instance was updated since it was first loaded.
    pub fn is_updated(&self) -> bool {
        matches!(self, UpdatedAt::Updated(_))
    }

    /// Updates the [UpdatedAt] field in the provided updatable entity if new and old values differ.
    pub fn update_if_changed<T: PartialEq>(
        updatable: &mut impl Updatable,
        old: T,
        new: T,
    ) -> T {
        if old == new {
            return old;
        }
        *updatable.get_updated_at_mut() = UpdatedAt::Updated(Utc::now());
        new
    }
}

impl From<UpdatedAt> for DateTime<Utc> {
    fn from(value: UpdatedAt) -> Self {
        match value {
            UpdatedAt::Loaded(time) | UpdatedAt::Updated(time) => time,
        }
    }
}
