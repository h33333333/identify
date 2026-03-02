use std::sync::Arc;

use crate::api::CachedUserInfo;

/// Information about an authenticated user.
#[derive(Debug, Clone)]
pub struct UserInformationExtension(pub Arc<CachedUserInfo>);

impl UserInformationExtension {
    pub fn new(info: Arc<CachedUserInfo>) -> Self {
        UserInformationExtension(info)
    }
}

impl From<UserInformationExtension> for Arc<CachedUserInfo> {
    fn from(value: UserInformationExtension) -> Self {
        value.0
    }
}
