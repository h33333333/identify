use std::{pin::Pin, sync::Arc};

use axum::{
    extract::Request,
    http,
    response::{IntoResponse, Response},
};
use identify_application::{
    GetUserParams, GroupUseCaseDeps, ListGroupsFilters, ListGroupsParams,
    UserUseCaseDeps, get_user, list_groups,
};
use identify_infrastructure::storage::{
    ToShared as _, groups::GroupsRepository, users::UsersRepository,
};
use identify_macros::gen_model;
use tower::{Layer, Service};
use tracing::trace;

use crate::api::{
    ApiState, CachedUserInfo, middleware::extensions::UserInformationExtension,
};

gen_model! {
    #[derive(Clone)]
    pub struct JwtAuthenticationLayer {
        state: ApiState,
    }

    pub struct NewJwtAuthenticationLayerAttrs;
}

impl JwtAuthenticationLayer {
    pub fn new(attrs: NewJwtAuthenticationLayerAttrs) -> Self {
        JwtAuthenticationLayer { state: attrs.state }
    }
}

impl<S> Layer<S> for JwtAuthenticationLayer {
    type Service = JwtAuthenticationMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        JwtAuthenticationMiddleware {
            inner,
            state: self.state.clone(),
        }
    }
}

#[derive(Clone)]
pub struct JwtAuthenticationMiddleware<S> {
    inner: S,
    state: ApiState,
}

impl<S> Service<Request> for JwtAuthenticationMiddleware<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = Pin<
        Box<
            dyn Future<Output = Result<Self::Response, Self::Error>>
                + Send
                + 'static,
        >,
    >;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request) -> Self::Future {
        let Some(auth_header) = req
            .headers()
            .get(http::header::AUTHORIZATION)
            .and_then(|auth| auth.to_str().ok())
            .and_then(|header_value| {
                header_value.strip_prefix("Bearer").map(str::trim)
            })
        else {
            return Box::pin(async move {
                Ok(http::StatusCode::UNAUTHORIZED.into_response())
            });
        };

        let user_id = match self.state.jwt_client().decode(auth_header) {
            Ok(user_id) => user_id,
            Err(e) => {
                trace!(err = %e, "JWT token validation failed");
                return Box::pin(async move {
                    Ok(http::StatusCode::UNAUTHORIZED.into_response())
                });
            }
        };

        let auth_cache = self.state.auth_cache();
        let pool = self.state.pool();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            let user_info_result = auth_cache
                .try_get_or_insert(
                    user_id,
                    Box::pin(async move {
                        let tx = pool.begin().await?.to_shared();

                        let users_repo = UsersRepository::new(tx.clone());
                        let user = get_user(
                            UserUseCaseDeps {
                                repository: &users_repo,
                            },
                            GetUserParams { id: user_id },
                        )
                        .await?;

                        let grous_repo = GroupsRepository::new(tx);
                        let user_groups = list_groups(
                            GroupUseCaseDeps {
                                repository: &grous_repo,
                            },
                            ListGroupsParams {
                                filters: ListGroupsFilters {
                                    user_id: Some(user_id),
                                },
                            },
                        )
                        .await?;

                        Result::<_, eyre::Report>::Ok(Arc::new(
                            CachedUserInfo {
                                user,
                                is_privileged: user_groups
                                    .iter()
                                    .any(|group| group.is_privileged()),
                                groups: user_groups,
                            },
                        ))
                    }),
                )
                .await;

            let user_info = match user_info_result {
                Ok(info) => info,
                Err(e) => {
                    trace!(err = %e, "Failed to get user info");
                    return Ok(
                        http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
                    );
                }
            };

            req.extensions_mut()
                .insert(UserInformationExtension::new(user_info));

            let response = inner.call(req).await?;
            Ok(response)
        })
    }
}
