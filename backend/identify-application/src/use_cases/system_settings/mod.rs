pub mod get_system_settings;
pub mod update_system_settings;

pub struct SystemSettingsUseCaseDeps<'a, R> {
    pub repository: &'a R,
}
