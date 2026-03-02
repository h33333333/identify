use std::borrow::Cow;

use identify_macros::gen_model;
use serde::{Deserialize, Serialize};

pub const AUDIENCE: &str = "identify";

gen_model! {
    /// JWT token claims that are expected within this domain.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Claims<'a> {
        /// Expires at.
        exp: i64,
        /// Issued at.
        iat: i64,
        /// Audience.
        ///
        /// Should always be equal to [AUDIENCE].
        #[new(skip)]
        aud: Cow<'a, str>,
        /// Subject.
        ///
        /// A UUID of the user this token was issued to.
        #[get(consume)]
        sub: Cow<'a, str>,
        /// Additional information about the associated entity.
        #[fw(serde(flatten))]
        meta: ClaimsMetadata<'a>,
    }

    pub struct NewClaimsAttrs<'a>;
}

impl<'a> Claims<'a> {
    pub fn new(attrs: NewClaimsAttrs<'a>) -> Self {
        Claims {
            exp: attrs.exp,
            iat: attrs.iat,
            aud: AUDIENCE.into(),
            sub: attrs.sub,
            meta: attrs.meta,
        }
    }
}

gen_model! {
    /// Additional information about the entity this token was issued to.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ClaimsMetadata<'a> {
        /// First name of the user this token was issued to.
        first_name: Cow<'a, str>,
        /// Last name of the user this token was issued to.
        last_name: Option<Cow<'a, str>>,
    }

    pub struct NewClaimsMetadataAttrs<'a>;
}

impl<'a> ClaimsMetadata<'a> {
    pub fn new(attrs: NewClaimsMetadataAttrs<'a>) -> Self {
        ClaimsMetadata {
            first_name: attrs.first_name,
            last_name: attrs.last_name,
        }
    }
}
