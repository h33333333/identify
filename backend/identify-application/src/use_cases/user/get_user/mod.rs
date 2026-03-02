use identify_domain::User;
use identify_ports::user_contracts;
use tracing::{instrument, trace};
use uuid::Uuid;

use crate::{Result, use_cases::user::UserUseCaseDeps};

#[derive(Debug)]
pub struct GetUserParams {
    pub id: Uuid,
}

#[instrument(skip(deps))]
pub async fn get_user<R: user_contracts::Get>(
    deps: UserUseCaseDeps<'_, R>,
    params: GetUserParams,
) -> Result<User> {
    trace!("Executing use case");

    let GetUserParams { id } = params;

    let user = deps.repository.get(id).await?;

    Ok(user)
}
