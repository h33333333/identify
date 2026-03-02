use std::pin::Pin;

use axum::{
    extract::Request,
    http,
    response::{IntoResponse, Response},
};
use tower::{Layer, Service};
use tracing::warn;

use crate::api::middleware::extensions::UserInformationExtension;

#[derive(Clone)]
pub struct PrivilegedOnlyAuthorizationLayer;

impl<S> Layer<S> for PrivilegedOnlyAuthorizationLayer {
    type Service = PrivilegedOnlyAuthorizationMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        PrivilegedOnlyAuthorizationMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct PrivilegedOnlyAuthorizationMiddleware<S> {
    inner: S,
}

impl<S> Service<Request> for PrivilegedOnlyAuthorizationMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
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

    fn call(&mut self, req: Request) -> Self::Future {
        let Some(UserInformationExtension(user_info)) =
            req.extensions().get::<UserInformationExtension>().cloned()
        else {
            warn!(
                "Missing user information in authorization middleware -- possible middleware misconfiguration"
            );

            return Box::pin(async move {
                Ok(http::StatusCode::UNAUTHORIZED.into_response())
            });
        };

        // This middleware allows requests only from privileged users.
        if !user_info.is_privileged {
            return Box::pin(async move {
                Ok(http::StatusCode::FORBIDDEN.into_response())
            });
        }

        let future = self.inner.call(req);
        Box::pin(async move {
            let response = future.await?;
            Ok(response)
        })
    }
}

impl<S> PrivilegedOnlyAuthorizationMiddleware<S> {}
