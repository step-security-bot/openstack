//! Creates or replaces a metadata item, by key, for a server.
//!
//! Creates a metadata item that does not already exist in the server. Replaces
//! existing metadata items that match keys with the metadata item in the
//! request.
//!
//! Policy defaults enable only users with the administrative role or the owner
//! of the server to perform this operation. Cloud providers can change these
//! permissions through the `policy.json` file.
//!
//! Normal response codes: 200
//!
//! Error response codes: badRequest(400), unauthorized(401), forbidden(403),
//! itemNotFound(404), conflict(409)
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde::Deserialize;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    #[builder(private, setter(name = "_meta"))]
    pub(crate) meta: BTreeMap<Cow<'a, str>, Cow<'a, str>>,

    /// server_id parameter for /v2.1/servers/{server_id}/topology API
    #[builder(default, setter(into))]
    server_id: Cow<'a, str>,

    /// id parameter for /v2.1/servers/{server_id}/metadata/{id} API
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
    pub fn meta<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.meta
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }

    /// Add a single header to the Metadata.
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
        http::Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "v2.1/servers/{server_id}/metadata/{id}",
            server_id = self.server_id.as_ref(),
            id = self.id.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("meta", serde_json::to_value(&self.meta)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Compute
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        None
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
                .meta(BTreeMap::<String, String>::new().into_iter())
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Compute
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Request::builder()
            .meta(BTreeMap::<String, String>::new().into_iter())
            .build()
            .unwrap()
            .response_key()
            .is_none())
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PUT).path(format!(
                "/v2.1/servers/{server_id}/metadata/{id}",
                server_id = "server_id",
                id = "id",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .server_id("server_id")
            .id("id")
            .meta(BTreeMap::<String, String>::new().into_iter())
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PUT)
                .path(format!(
                    "/v2.1/servers/{server_id}/metadata/{id}",
                    server_id = "server_id",
                    id = "id",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .server_id("server_id")
            .id("id")
            .meta(BTreeMap::<String, String>::new().into_iter())
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
