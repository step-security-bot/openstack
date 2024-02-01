//! The API provides a unified request for creating a remote console. The user
//! can
//! get a URL to connect the console from this API. The URL includes the token
//! which is used to get permission to access the console. Servers may support
//! different console protocols. To return a remote console using a specific
//! protocol, such as RDP, set the `protocol` parameter to `rdp`.
//!
//! Normal response codes: 200
//!
//! Error response codes: badRequest(400), unauthorized(401), forbidden(403),
//! itemNotFound(404),
//! conflict(409), notImplemented(501)
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Protocol {
    #[serde(rename = "rdp")]
    Rdp,
    #[serde(rename = "serial")]
    Serial,
    #[serde(rename = "spice")]
    Spice,
    #[serde(rename = "vnc")]
    Vnc,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Type {
    #[serde(rename = "novnc")]
    Novnc,
    #[serde(rename = "rdp-html5")]
    RdpHtml5,
    #[serde(rename = "serial")]
    Serial,
    #[serde(rename = "spice-html5")]
    SpiceHtml5,
    #[serde(rename = "xvpvnc")]
    Xvpvnc,
}

/// The remote console object.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct RemoteConsole {
    /// The protocol of remote console. The valid values are `vnc`, `spice`,
    /// `rdp`, `serial` and `mks`. The protocol `mks` is added since
    /// Microversion `2.8`.
    #[serde()]
    #[builder()]
    pub(crate) protocol: Protocol,

    /// The type of remote console. The valid values are `novnc`,
    /// `rdp-html5`, `spice-html5`, `serial`, and `webmks`. The type
    /// `webmks` is added since Microversion `2.8`.
    #[serde(rename = "type")]
    #[builder()]
    pub(crate) _type: Type,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// The remote console object.
    #[builder(setter(into))]
    pub(crate) remote_console: RemoteConsole,

    /// server_id parameter for /v2.1/servers/{server_id}/topology API
    #[builder(default, setter(into))]
    server_id: Cow<'a, str>,

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
    /// Add a single header to the Remote_Console.
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
        http::Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "v2.1/servers/{server_id}/remote-consoles",
            server_id = self.server_id.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push(
            "remote_console",
            serde_json::to_value(&self.remote_console)?,
        );

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Compute
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("remote_console".into())
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
                .remote_console(
                    RemoteConsoleBuilder::default()
                        .protocol(Protocol::Vnc)
                        ._type(Type::Novnc)
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Compute
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder()
                .remote_console(
                    RemoteConsoleBuilder::default()
                        .protocol(Protocol::Vnc)
                        ._type(Type::Novnc)
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "remote_console"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST).path(format!(
                "/v2.1/servers/{server_id}/remote-consoles",
                server_id = "server_id",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "remote_console": {} }));
        });

        let endpoint = Request::builder()
            .server_id("server_id")
            .remote_console(
                RemoteConsoleBuilder::default()
                    .protocol(Protocol::Vnc)
                    ._type(Type::Novnc)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!(
                    "/v2.1/servers/{server_id}/remote-consoles",
                    server_id = "server_id",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "remote_console": {} }));
        });

        let endpoint = Request::builder()
            .server_id("server_id")
            .remote_console(
                RemoteConsoleBuilder::default()
                    .protocol(Protocol::Vnc)
                    ._type(Type::Novnc)
                    .build()
                    .unwrap(),
            )
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
