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
//
// WARNING: This file is automatically generated from OpenAPI schema using
// `openstack-codegenerator`.
//! Creates a floating IP, and, if you specify port information, associates the
//! floating IP with an internal port.
//!
//! To associate the floating IP with an internal port, specify the
//! port ID attribute in the request body. If you do not specify a
//! port ID in the request, you can issue a PUT request instead of a
//! POST request.
//!
//! Default policy settings enable only administrative users to set
//! floating IP addresses and some non-administrative users might
//! require a floating IP address. If you do not specify a floating IP
//! address in the request, the operation automatically allocates one.
//!
//! By default, this operation associates the floating IP address with
//! a single fixed IP address that is configured on an OpenStack
//! Networking port. If a port has multiple IP addresses, you must
//! specify the `fixed\_ip\_address` attribute in the request body to
//! associate a fixed IP address with the floating IP address.
//!
//! You can create floating IPs on only external networks. When you
//! create a floating IP, you must specify the ID of the network on
//! which you want to create the floating IP. Alternatively, you can
//! create a floating IP on a subnet in the external network, based on
//! the costs and quality of that subnet.
//!
//! You must configure an IP address with the internal OpenStack
//! Networking port that is associated with the floating IP address.
//!
//! The operation returns the `Bad Request (400)` response code for one of
//! reasons:
//!
//! If the port ID is not valid, this operation returns `404` response code.
//!
//! The operation returns the `Conflict (409)` response code for one of
//! reasons:
//!
//! Normal response codes: 201
//!
//! Error response codes: 400, 401, 404, 409
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde::Deserialize;
use std::borrow::Cow;

/// A `floatingip` object. When you associate a
/// floating IP address with a VM, the instance has the same public IP
/// address each time that it boots, basically to maintain a
/// consistent IP address for maintaining DNS assignment.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Floatingip<'a> {
    /// The floating IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) floating_ip_address: Option<Cow<'a, str>>,

    /// The subnet ID on which you want to create the floating IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) subnet_id: Option<Option<Cow<'a, str>>>,

    /// The ID of the network associated with the
    /// floating IP.
    #[serde()]
    #[builder(setter(into))]
    pub(crate) floating_network_id: Cow<'a, str>,

    /// The ID of a port associated with the floating IP.
    /// To associate the floating IP with a fixed IP at creation time,
    /// you must specify the identifier of the internal port.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) port_id: Option<Option<Cow<'a, str>>>,

    /// The fixed IP address that is associated with the floating IP.
    /// If an internal port has multiple associated IP addresses,
    /// the service chooses the first IP address unless you explicitly
    /// define a fixed IP address in the `fixed\_ip\_address` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) fixed_ip_address: Option<Cow<'a, str>>,

    /// The ID of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tenant_id: Option<Cow<'a, str>>,

    /// The ID of the QoS policy associated with the floating IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) qos_policy_id: Option<Option<Cow<'a, str>>>,

    /// A valid DNS name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) dns_name: Option<Cow<'a, str>>,

    /// A valid DNS domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) dns_domain: Option<Cow<'a, str>>,

    /// A human-readable description for the resource.
    /// Default is an empty string.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) description: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// A `floatingip` object. When you associate a
    /// floating IP address with a VM, the instance has the same public IP
    /// address each time that it boots, basically to maintain a
    /// consistent IP address for maintaining DNS assignment.
    #[builder(setter(into))]
    pub(crate) floatingip: Floatingip<'a>,

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
    /// Add a single header to the Floatingip.
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
        "v2.0/floatingips".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("floatingip", serde_json::to_value(&self.floatingip)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Network
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("floatingip".into())
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
                .floatingip(
                    FloatingipBuilder::default()
                        .floating_network_id("foo")
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Network
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder()
                .floatingip(
                    FloatingipBuilder::default()
                        .floating_network_id("foo")
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "floatingip"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/v2.0/floatingips".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "floatingip": {} }));
        });

        let endpoint = Request::builder()
            .floatingip(
                FloatingipBuilder::default()
                    .floating_network_id("foo")
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
                .path("/v2.0/floatingips".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "floatingip": {} }));
        });

        let endpoint = Request::builder()
            .floatingip(
                FloatingipBuilder::default()
                    .floating_network_id("foo")
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
