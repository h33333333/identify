use identify_domain::SystemSettings;
use identify_ports::system_settings_contracts;
use tracing::{instrument, trace};

use crate::{Result, SystemSettingsUseCaseDeps};

#[derive(Debug)]
pub struct UpsertSystemSettingsParams<'a> {
    pub settings: &'a SystemSettings,
}

#[instrument(skip(deps))]
pub async fn update_system_settings<R: system_settings_contracts::Update>(
    deps: SystemSettingsUseCaseDeps<'_, R>,
    params: UpsertSystemSettingsParams<'_>,
) -> Result<()> {
    trace!("Executing use case");

    let UpsertSystemSettingsParams { settings } = params;

    if !settings.updated_at().is_updated() {
        trace!("Entity is already up to date, nothing to update.");
        return Ok(());
    }

    deps.repository.update(settings).await?;

    Ok(())
}
