use identify_domain::{NewUserAttrs, User};
use tracing::{instrument, trace};

use crate::{Result, use_cases::user::UserUseCaseDeps, user_contracts};

#[derive(Debug)]
pub struct CreateUserParams {
    pub user_attrs: NewUserAttrs,
}

#[instrument(skip(deps))]
pub async fn create_user<R: user_contracts::Insert>(
    deps: UserUseCaseDeps<'_, R>,
    params: CreateUserParams,
) -> Result<User> {
    trace!("Executing use case");

    let CreateUserParams { user_attrs } = params;

    let user = User::new(user_attrs);
    deps.repository.insert(&user).await?;

    Ok(user)
}
