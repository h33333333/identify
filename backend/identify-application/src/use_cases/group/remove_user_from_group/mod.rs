use identify_ports::group_contracts;
use tracing::{instrument, trace};
use uuid::Uuid;

use crate::{Result, use_cases::group::GroupUseCaseDeps};

#[derive(Debug)]
pub struct RemoveUserFromGroupParams {
    pub user_id: Uuid,
    pub group_id: Uuid,
}

#[instrument(skip(deps))]
pub async fn remove_user_from_group<R: group_contracts::RemoveUser>(
    deps: GroupUseCaseDeps<'_, R>,
    params: RemoveUserFromGroupParams,
) -> Result<()> {
    trace!("Executing use case");

    let RemoveUserFromGroupParams { user_id, group_id } = params;

    deps.repository
        .remove_user_from_group(user_id, group_id)
        .await
}
