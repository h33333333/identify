use identify_ports::group_contracts;
use tracing::{instrument, trace};
use uuid::Uuid;

use crate::{Result, use_cases::group::GroupUseCaseDeps};

#[derive(Debug)]
pub struct AddUserToGroupParams {
    pub user_id: Uuid,
    pub group_id: Uuid,
}

#[instrument(skip(deps))]
pub async fn add_user_to_group<R: group_contracts::AddUser>(
    deps: GroupUseCaseDeps<'_, R>,
    params: AddUserToGroupParams,
) -> Result<()> {
    trace!("Executing use case");

    let AddUserToGroupParams { user_id, group_id } = params;

    deps.repository.add_user_to_group(user_id, group_id).await
}
