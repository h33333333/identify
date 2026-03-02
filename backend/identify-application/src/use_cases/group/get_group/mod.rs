use identify_domain::Group;
use identify_ports::group_contracts;
use tracing::{instrument, trace};
use uuid::Uuid;

use crate::{Result, use_cases::group::GroupUseCaseDeps};

#[derive(Debug)]
pub struct GetGroupParams {
    pub id: Uuid,
}

#[instrument(skip(deps))]
pub async fn get_group<R: group_contracts::Get>(
    deps: GroupUseCaseDeps<'_, R>,
    params: GetGroupParams,
) -> Result<Group> {
    trace!("Executing use case");

    let GetGroupParams { id } = params;

    let group = deps.repository.get(id).await?;

    Ok(group)
}
