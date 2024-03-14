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

//! Update a federated mapping.
//!
//! Relationship:
//! `https://docs.openstack.org/api/openstack-identity/3/ext/OS-FEDERATION/1.0/rel/mapping`
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Domain<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) id: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Type {
    #[serde(rename = "ephemeral")]
    Ephemeral,
    #[serde(rename = "local")]
    Local,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct User<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) domain: Option<Domain<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) email: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) id: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) _type: Option<Type>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Roles<'a> {
    #[serde()]
    #[builder(setter(into))]
    pub(crate) name: Cow<'a, str>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Projects<'a> {
    #[serde()]
    #[builder(setter(into))]
    pub(crate) name: Cow<'a, str>,

    #[serde()]
    #[builder(setter(into))]
    pub(crate) roles: Vec<Roles<'a>>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Group<'a> {
    #[serde()]
    #[builder(setter(into))]
    pub(crate) id: Cow<'a, str>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct GroupStruct<'a> {
    #[serde()]
    #[builder(setter(into))]
    pub(crate) domain: Domain<'a>,

    #[serde()]
    #[builder(setter(into))]
    pub(crate) name: Cow<'a, str>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum GroupEnum<'a> {
    F1(Group<'a>),
    F2(GroupStruct<'a>),
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Local<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) domain: Option<Domain<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) group: Option<GroupEnum<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) group_ids: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) groups: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) projects: Option<Vec<Projects<'a>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) user: Option<User<'a>>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct RemoteType<'a> {
    #[serde(rename = "type")]
    #[builder(setter(into))]
    pub(crate) _type: Cow<'a, str>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct RemoteTypeAnyOneOfRegex<'a> {
    #[serde()]
    #[builder(setter(into))]
    pub(crate) any_one_of: Vec<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) regex: Option<bool>,

    #[serde(rename = "type")]
    #[builder(setter(into))]
    pub(crate) _type: Cow<'a, str>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct RemoteTypeNotAnyOfRegex<'a> {
    #[serde()]
    #[builder(setter(into))]
    pub(crate) not_any_of: Vec<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) regex: Option<bool>,

    #[serde(rename = "type")]
    #[builder(setter(into))]
    pub(crate) _type: Cow<'a, str>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct RemoteTypeBlacklistRegex<'a> {
    #[serde()]
    #[builder(setter(into))]
    pub(crate) blacklist: Vec<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) regex: Option<bool>,

    #[serde(rename = "type")]
    #[builder(setter(into))]
    pub(crate) _type: Cow<'a, str>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct RemoteTypeWhitelistRegex<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) regex: Option<bool>,

    #[serde(rename = "type")]
    #[builder(setter(into))]
    pub(crate) _type: Cow<'a, str>,

    #[serde()]
    #[builder(setter(into))]
    pub(crate) whitelist: Vec<Cow<'a, str>>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum RemoteEnum<'a> {
    F1(RemoteType<'a>),
    F2(RemoteTypeAnyOneOfRegex<'a>),
    F3(RemoteTypeNotAnyOfRegex<'a>),
    F4(RemoteTypeBlacklistRegex<'a>),
    F5(RemoteTypeWhitelistRegex<'a>),
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Rules<'a> {
    #[serde()]
    #[builder(setter(into))]
    pub(crate) local: Vec<Local<'a>>,

    #[serde()]
    #[builder(setter(into))]
    pub(crate) remote: Vec<RemoteEnum<'a>>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Mapping<'a> {
    /// The list of rules used to map remote users into local users
    ///
    #[serde()]
    #[builder(setter(into))]
    pub(crate) rules: Vec<Rules<'a>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    #[builder(setter(into))]
    pub(crate) mapping: Mapping<'a>,

    /// mapping_id parameter for /v3/OS-FEDERATION/mappings/{mapping_id} API
    ///
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
    /// Add a single header to the Mapping.
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
        format!("v3/OS-FEDERATION/mappings/{id}", id = self.id.as_ref(),).into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("mapping", serde_json::to_value(&self.mapping)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Identity
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("mapping".into())
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
            Request::builder()
                .mapping(
                    MappingBuilder::default()
                        .rules(Vec::from([RulesBuilder::default()
                            .local(Vec::from([LocalBuilder::default().build().unwrap()]))
                            .remote(Vec::from([RemoteEnum::F1(
                                RemoteTypeBuilder::default()._type("foo").build().unwrap()
                            )]))
                            .build()
                            .unwrap()]))
                        .build()
                        .unwrap()
                )
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
                .mapping(
                    MappingBuilder::default()
                        .rules(Vec::from([RulesBuilder::default()
                            .local(Vec::from([LocalBuilder::default().build().unwrap()]))
                            .remote(Vec::from([RemoteEnum::F1(
                                RemoteTypeBuilder::default()._type("foo").build().unwrap()
                            )]))
                            .build()
                            .unwrap()]))
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "mapping"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PATCH)
                .path(format!("/v3/OS-FEDERATION/mappings/{id}", id = "id",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "mapping": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .mapping(
                MappingBuilder::default()
                    .rules(Vec::from([RulesBuilder::default()
                        .local(Vec::from([LocalBuilder::default().build().unwrap()]))
                        .remote(Vec::from([RemoteEnum::F1(
                            RemoteTypeBuilder::default()._type("foo").build().unwrap(),
                        )]))
                        .build()
                        .unwrap()]))
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
            when.method(httpmock::Method::PATCH)
                .path(format!("/v3/OS-FEDERATION/mappings/{id}", id = "id",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "mapping": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .mapping(
                MappingBuilder::default()
                    .rules(Vec::from([RulesBuilder::default()
                        .local(Vec::from([LocalBuilder::default().build().unwrap()]))
                        .remote(Vec::from([RemoteEnum::F1(
                            RemoteTypeBuilder::default()._type("foo").build().unwrap(),
                        )]))
                        .build()
                        .unwrap()]))
                    .build()
                    .unwrap(),
            )
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
