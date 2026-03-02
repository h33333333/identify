use identify_domain::Group;
pub use identify_ports::group_contracts::ListGroupsFilters;
use identify_ports::group_contracts::{self};
use tracing::{instrument, trace};

use crate::{Result, use_cases::group::GroupUseCaseDeps};

#[derive(Debug)]
pub struct ListGroupsParams {
    pub filters: ListGroupsFilters,
}

#[instrument(skip(deps))]
pub async fn list_groups<R: group_contracts::List>(
    deps: GroupUseCaseDeps<'_, R>,
    params: ListGroupsParams,
) -> Result<Vec<Group>> {
    trace!("Executing use case");

    let ListGroupsParams { filters } = params;

    deps.repository.list(filters).await
}
