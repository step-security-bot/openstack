//! Updates a user.
//!
//! If the back-end driver does not support this functionality, this
//! call might return the HTTP `Not Implemented (501)` response code.
//!
//! Relationship: `https://docs.openstack.org/api/openstack-
//! identity/3/rel/user`
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde::Deserialize;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Protocols<'a> {
    #[serde()]
    #[builder(setter(into))]
    pub(crate) protocol_id: Cow<'a, str>,

    #[serde()]
    #[builder(setter(into))]
    pub(crate) unique_id: Cow<'a, str>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Federated<'a> {
    #[serde()]
    #[builder(setter(into))]
    pub(crate) idp_id: Cow<'a, str>,

    #[serde()]
    #[builder(setter(into))]
    pub(crate) protocols: Vec<Protocols<'a>>,
}

/// The resource options for the user. Available resource options are
/// `ignore\_change\_password\_upon\_first\_use`, `ignore\_password\_expiry`,
/// `ignore\_lockout\_failure\_attempts`, `lock\_password`,
/// `multi\_factor\_auth\_enabled`, and `multi\_factor\_auth\_rules`
/// `ignore\_user\_inactivity`.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Options<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) ignore_change_password_upon_first_use: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) ignore_password_expiry: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) ignore_lockout_failure_attempts: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) lock_password: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) ignore_user_inactivity: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) multi_factor_auth_rules: Option<Vec<Vec<Cow<'a, str>>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) multi_factor_auth_enabled: Option<bool>,
}

/// A `user` object
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct User<'a> {
    /// The ID of the default project for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) default_project_id: Option<Option<Cow<'a, str>>>,

    /// The new description of the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) description: Option<Option<Cow<'a, str>>>,

    /// The ID of the domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) domain_id: Option<Cow<'a, str>>,

    /// If the user is enabled, this value is `true`.
    /// If the user is disabled, this value is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) enabled: Option<bool>,

    /// List of federated objects associated with a user. Each object in the
    /// list
    /// contains the `idp\_id` and `protocols`. `protocols` is a list of
    /// objects, each of which contains `protocol\_id` and `unique\_id` of
    /// the protocol and user respectively. For example:
    ///
    ///
    ///
    /// ```text
    /// "federated": [
    ///   {
    ///     "idp\_id": "efbab5a6acad4d108fec6c63d9609d83",
    ///     "protocols": [
    ///       {"protocol\_id": "mapped", "unique\_id": "test@example.com"}
    ///     ]
    ///   }
    /// ]
    ///
    /// ```
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) federated: Option<Vec<Federated<'a>>>,

    /// The user name. Must be unique within the owning domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    /// The new password for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) password: Option<Option<Cow<'a, str>>>,

    /// The resource options for the user. Available resource options are
    /// `ignore\_change\_password\_upon\_first\_use`,
    /// `ignore\_password\_expiry`,
    /// `ignore\_lockout\_failure\_attempts`, `lock\_password`,
    /// `multi\_factor\_auth\_enabled`, and `multi\_factor\_auth\_rules`
    /// `ignore\_user\_inactivity`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) options: Option<Options<'a>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// A `user` object
    #[builder(setter(into))]
    pub(crate) user: User<'a>,

    /// user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id}
    /// API
    #[builder(default, setter(into))]
    id: Cow<'a, str>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}
impl<'a> Request<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RequestBuilder<'a> {
        RequestBuilder::default()
    }
}

impl<'a> RequestBuilder<'a> {
    /// Add a single header to the User.
    pub fn header(&mut self, header_name: &'static str, header_value: &'static str) -> &mut Self
where {
        self._headers
            .get_or_insert(None)
            .get_or_insert_with(HeaderMap::new)
            .insert(header_name, HeaderValue::from_static(header_value));
        self
    }

    /// Add multiple headers.
    pub fn headers<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<(Option<HeaderName>, HeaderValue)>,
    {
        self._headers
            .get_or_insert(None)
            .get_or_insert_with(HeaderMap::new)
            .extend(iter.map(Into::into));
        self
    }
}

impl<'a> RestEndpoint for Request<'a> {
    fn method(&self) -> http::Method {
        http::Method::PATCH
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("v3/users/{id}", id = self.id.as_ref(),).into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("user", serde_json::to_value(&self.user)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Identity
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("user".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{self, Query, RawQuery};
    use crate::test::client::MockServerClient;
    use crate::types::ServiceType;
    use http::{HeaderName, HeaderValue};
    use serde::Deserialize;
    use serde::Serialize;
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Request::builder()
                .user(UserBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Identity
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder()
                .user(UserBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "user"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PATCH)
                .path(format!("/v3/users/{id}", id = "id",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "user": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .user(UserBuilder::default().build().unwrap())
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PATCH)
                .path(format!("/v3/users/{id}", id = "id",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "user": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .user(UserBuilder::default().build().unwrap())
            .headers(
                [(
                    Some(HeaderName::from_static("foo")),
                    HeaderValue::from_static("bar"),
                )]
                .iter()
                .cloned(),
            )
            .header("not_foo", "not_bar")
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }
}
