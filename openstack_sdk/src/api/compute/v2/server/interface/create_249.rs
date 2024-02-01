//! Creates a port interface and uses it to attach a port to a server.
//!
//! Normal response codes: 200
//!
//! Error response codes: badRequest(400), unauthorized(401), forbidden(403),
//! itemNotFound(404), conflict(409), computeFault(500), NotImplemented(501)
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde::Deserialize;
use std::borrow::Cow;

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct FixedIps<'a> {
    /// The IP address. It is required when `fixed\_ips` is specified.
    #[serde()]
    #[builder(setter(into))]
    pub(crate) ip_address: Cow<'a, str>,
}

/// Specify the `interfaceAttachment` action in the request body.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct InterfaceAttachment<'a> {
    /// The ID of the network for which you want to create a port interface.
    /// The `net\_id`
    /// and `port\_id` parameters are mutually exclusive. If you do not specify
    /// the
    /// `net\_id` parameter, the OpenStack Networking API v2.0 uses the network
    /// information
    /// cache that is associated with the instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) net_id: Option<Cow<'a, str>>,

    /// The ID of the port for which you want to create an interface. The
    /// `net\_id`
    /// and `port\_id` parameters are mutually exclusive. If you do not specify
    /// the
    /// `port\_id` parameter, the OpenStack Networking API v2.0 allocates a
    /// port and
    /// creates an interface for it on the network.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) port_id: Option<Cow<'a, str>>,

    /// Fixed IP addresses. If you request a specific fixed IP address without
    /// a `net\_id`, the request returns a `Bad Request (400)` response code.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) fixed_ips: Option<Vec<FixedIps<'a>>>,

    /// A device role tag that can be applied to a network interface when
    /// attaching
    /// it to the VM. The guest OS of a server that has devices tagged in this
    /// manner can access hardware metadata about the tagged devices from the
    /// metadata API and on the config
    /// drive, if enabled.
    ///
    ///
    /// **New in version 2.49**
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tag: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// Specify the `interfaceAttachment` action in the request body.
    #[builder(setter(into))]
    pub(crate) interface_attachment: InterfaceAttachment<'a>,

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
    /// Add a single header to the Interface.
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
            "v2.1/servers/{server_id}/os-interface",
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
            "interfaceAttachment",
            serde_json::to_value(&self.interface_attachment)?,
        );

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Compute
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("interfaceAttachment".into())
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
                .interface_attachment(InterfaceAttachmentBuilder::default().build().unwrap())
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
                .interface_attachment(InterfaceAttachmentBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "interfaceAttachment"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST).path(format!(
                "/v2.1/servers/{server_id}/os-interface",
                server_id = "server_id",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "interfaceAttachment": {} }));
        });

        let endpoint = Request::builder()
            .server_id("server_id")
            .interface_attachment(InterfaceAttachmentBuilder::default().build().unwrap())
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
                    "/v2.1/servers/{server_id}/os-interface",
                    server_id = "server_id",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "interfaceAttachment": {} }));
        });

        let endpoint = Request::builder()
            .server_id("server_id")
            .interface_attachment(InterfaceAttachmentBuilder::default().build().unwrap())
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
