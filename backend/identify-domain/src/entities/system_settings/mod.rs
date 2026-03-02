use crate::Result;

use chrono::{DateTime, Utc};
use identify_macros::gen_model;
use identify_util::{Updatable, UpdatedAt};

gen_model! {
    /// System-wide settings.
    ///
    /// At most one instance of system settings can exist at the same time.
    #[derive(Debug, Clone)]
    pub struct SystemSettings {
        /// Is this instance initialized.
        #[get(copy)]
        is_initialized: bool,

        #[get(copy)]
        created_at: DateTime<Utc>,
        #[get(copy)]
        #[new(skip)]
        #[hydrate(type(DateTime<Utc>))]
        updated_at: UpdatedAt,
    }

    #[derive(Debug)]
    pub struct NewSystemSettingsAttrs;

    #[derive(Debug)]
    pub struct SystemSettingsAttrs;
}

impl SystemSettings {
    pub fn new(attrs: NewSystemSettingsAttrs) -> Self {
        SystemSettings {
            is_initialized: attrs.is_initialized,
            created_at: attrs.created_at,
            updated_at: UpdatedAt::new(attrs.created_at),
        }
    }

    pub fn load(attrs: SystemSettingsAttrs) -> Result<Self> {
        Ok(SystemSettings {
            is_initialized: attrs.is_initialized,
            created_at: attrs.created_at,
            updated_at: UpdatedAt::Loaded(attrs.updated_at),
        })
    }

    pub fn to_attributes(&self) -> SystemSettingsAttrs {
        SystemSettingsAttrs {
            is_initialized: self.is_initialized,
            created_at: self.created_at,
            updated_at: self.updated_at.into(),
        }
    }

    pub fn update_is_initialized(&mut self, is_initialized: bool) -> &mut Self {
        self.is_initialized = UpdatedAt::update_if_changed(
            self,
            self.is_initialized,
            is_initialized,
        );
        self
    }
}

impl Updatable for SystemSettings {
    fn get_updated_at_mut(&mut self) -> &mut UpdatedAt {
        &mut self.updated_at
    }
}
