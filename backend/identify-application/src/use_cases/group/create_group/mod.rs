use identify_domain::{Group, NewGroupAttrs};
use identify_ports::group_contracts;
use tracing::{instrument, trace};

use crate::{Result, use_cases::group::GroupUseCaseDeps};

#[derive(Debug)]
pub struct CreateGroupParams {
    pub group_attrs: NewGroupAttrs,
}

#[instrument(skip(deps))]
pub async fn create_group<R: group_contracts::Insert>(
    deps: GroupUseCaseDeps<'_, R>,
    params: CreateGroupParams,
) -> Result<Group> {
    trace!("Executing use case");

    let CreateGroupParams { group_attrs } = params;

    let group = Group::new(group_attrs);
    deps.repository.insert(&group).await?;

    Ok(group)
}
