// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

//! OpenStack API authentication
//!
//! Currently there are only 2 types of auth supported:
//!
//! - AuthToken (X-Auth-Token header)
//! - None (unauthenticated)

use http::{HeaderMap, HeaderValue};

use std::fmt::Debug;
use tracing::error;

use thiserror::Error;

pub mod authtoken;
mod authtoken_utils;
pub mod v3password;
pub mod v3token;
pub mod v3totp;
pub mod v3websso;

use authtoken::{AuthToken, AuthTokenError};

/// Authentication error
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthError {
    #[error("header value error: {}", source)]
    HeaderValue {
        #[from]
        /// Error source
        source: http::header::InvalidHeaderValue,
    },

    #[error("Invalid auth_url: {}", source)]
    InvalidAuthUrl {
        #[from]
        /// Error source
        source: url::ParseError,
    },

    #[error("AuthToken error: {}", source)]
    AuthToken {
        #[from]
        /// Error source
        source: AuthTokenError,
    },

    #[error("Cannot construct auth from config: {}", msg)]
    Config {
        /// Error source
        msg: String,
    },

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Authentication state enum
#[derive(Debug, Eq, PartialEq)]
pub enum AuthState {
    /// Auth is valid
    Valid,
    /// Expired
    Expired,
    /// Authentication is missing
    Unset,
}

/// An OpenStack Authentication type
#[derive(Clone)]
#[non_exhaustive]
pub enum Auth {
    /// An X-Auth-Token
    AuthToken(Box<AuthToken>),
    /// Unauthenticated access
    None,
}

impl Auth {
    /// Adds X-Auth-Token header to a request headers.
    ///
    /// Returns an error if the token string cannot be parsed as a header value.
    pub fn set_header<'a>(
        &self,
        headers: &'a mut HeaderMap<HeaderValue>,
    ) -> Result<&'a mut HeaderMap<HeaderValue>, AuthError> {
        if let Auth::AuthToken(token) = self {
            let _ = token.set_header(headers);
        }

        Ok(headers)
    }
}
