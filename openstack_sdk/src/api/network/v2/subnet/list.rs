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

//! Lists subnets that the project has access to.
//!
//! Default policy settings return only subnets owned by the project of the
//! user submitting the request, unless the user has administrative role. You
//! can control which attributes are returned by using the fields query
//! parameter. You can filter results by using query string parameters.
//!
//! Use the `fields` query parameter to control which fields are returned in
//! the response body. Additionally, you can filter results by using query
//! string parameters. For information, see
//! [Filtering and Column Selection](https://wiki.openstack.org/wiki/Neutron/APIv2-specification#Filtering_and_Column_Selection).
//!
//! Normal response codes: 200
//!
//! Error response codes: 401
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use crate::api::common::CommaSeparatedList;
use std::borrow::Cow;

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// cidr query parameter for /v2.0/subnets API
    ///
    #[builder(default, setter(into))]
    cidr: Option<Cow<'a, str>>,

    /// description query parameter for /v2.0/subnets API
    ///
    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,

    /// enable_dhcp query parameter for /v2.0/subnets API
    ///
    #[builder(default)]
    enable_dhcp: Option<bool>,

    /// gateway_ip query parameter for /v2.0/subnets API
    ///
    #[builder(default, setter(into))]
    gateway_ip: Option<Cow<'a, str>>,

    /// id query parameter for /v2.0/subnets API
    ///
    #[builder(default, setter(into))]
    id: Option<Cow<'a, str>>,

    /// ip_version query parameter for /v2.0/subnets API
    ///
    #[builder(default)]
    ip_version: Option<i32>,

    /// ipv6_address_mode query parameter for /v2.0/subnets API
    ///
    #[builder(default, setter(into))]
    ipv6_address_mode: Option<Cow<'a, str>>,

    /// ipv6_ra_mode query parameter for /v2.0/subnets API
    ///
    #[builder(default, setter(into))]
    ipv6_ra_mode: Option<Cow<'a, str>>,

    /// name query parameter for /v2.0/subnets API
    ///
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// network_id query parameter for /v2.0/subnets API
    ///
    #[builder(default, setter(into))]
    network_id: Option<Cow<'a, str>>,

    /// not-tags query parameter for /v2.0/subnets API
    ///
    #[builder(default, private, setter(name = "_not_tags"))]
    not_tags: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// not-tags-any query parameter for /v2.0/subnets API
    ///
    #[builder(default, private, setter(name = "_not_tags_any"))]
    not_tags_any: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// revision_number query parameter for /v2.0/subnets API
    ///
    #[builder(default, setter(into))]
    revision_number: Option<Cow<'a, str>>,

    /// segment_id query parameter for /v2.0/subnets API
    ///
    #[builder(default, setter(into))]
    segment_id: Option<Cow<'a, str>>,

    /// shared query parameter for /v2.0/subnets API
    ///
    #[builder(default)]
    shared: Option<bool>,

    /// subnetpool_id query parameter for /v2.0/subnets API
    ///
    #[builder(default, setter(into))]
    subnetpool_id: Option<Cow<'a, str>>,

    /// tags query parameter for /v2.0/subnets API
    ///
    #[builder(default, private, setter(name = "_tags"))]
    tags: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// tags-any query parameter for /v2.0/subnets API
    ///
    #[builder(default, private, setter(name = "_tags_any"))]
    tags_any: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// tenant_id query parameter for /v2.0/subnets API
    ///
    #[builder(default, setter(into))]
    tenant_id: Option<Cow<'a, str>>,

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
    /// tags query parameter for /v2.0/subnets API
    ///
    pub fn tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.tags
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// tags-any query parameter for /v2.0/subnets API
    ///
    pub fn tags_any<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.tags_any
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// not-tags query parameter for /v2.0/subnets API
    ///
    pub fn not_tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.not_tags
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// not-tags-any query parameter for /v2.0/subnets API
    ///
    pub fn not_tags_any<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.not_tags_any
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// Add a single header to the Subnet.
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
        http::Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v2.0/subnets".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("id", self.id.as_ref());
        params.push_opt("name", self.name.as_ref());
        params.push_opt("ip_version", self.ip_version);
        params.push_opt("network_id", self.network_id.as_ref());
        params.push_opt("subnetpool_id", self.subnetpool_id.as_ref());
        params.push_opt("cidr", self.cidr.as_ref());
        params.push_opt("gateway_ip", self.gateway_ip.as_ref());
        params.push_opt("tenant_id", self.tenant_id.as_ref());
        params.push_opt("enable_dhcp", self.enable_dhcp);
        params.push_opt("ipv6_ra_mode", self.ipv6_ra_mode.as_ref());
        params.push_opt("ipv6_address_mode", self.ipv6_address_mode.as_ref());
        params.push_opt("shared", self.shared);
        params.push_opt("revision_number", self.revision_number.as_ref());
        params.push_opt("tags", self.tags.as_ref());
        params.push_opt("tags-any", self.tags_any.as_ref());
        params.push_opt("not-tags", self.not_tags.as_ref());
        params.push_opt("not-tags-any", self.not_tags_any.as_ref());
        params.push_opt("description", self.description.as_ref());
        params.push_opt("segment_id", self.segment_id.as_ref());

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Network
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("subnets".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use crate::api::Query;
    use crate::test::client::MockServerClient;
    use crate::types::ServiceType;
    use http::{HeaderName, HeaderValue};
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Request::builder().build().unwrap().service_type(),
            ServiceType::Network
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder().build().unwrap().response_key().unwrap(),
            "subnets"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.0/subnets".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "subnets": {} }));
        });

        let endpoint = Request::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.0/subnets".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "subnets": {} }));
        });

        let endpoint = Request::builder()
            .headers(
                [(
                    Some(HeaderName::from_static("foo")),
                    HeaderValue::from_static("bar"),
                )]
                .into_iter(),
            )
            .header("not_foo", "not_bar")
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }
}
