use identify_domain::SystemSettings;
use identify_ports::system_settings_contracts;
use tracing::{instrument, trace};

use crate::{Result, use_cases::system_settings::SystemSettingsUseCaseDeps};

// Empty struct for compatibility.
#[derive(Debug)]
pub struct GetSystemSettingsParams;

#[instrument(skip(deps))]
pub async fn get_system_settings<R: system_settings_contracts::Get>(
    deps: SystemSettingsUseCaseDeps<'_, R>,
    _params: GetSystemSettingsParams,
) -> Result<SystemSettings> {
    trace!("Executing use case");

    let settings = deps.repository.get().await?;

    Ok(settings)
}
