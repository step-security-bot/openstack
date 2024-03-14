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

//! Creates a property definition in a namespace.
//!
//! The schema is a subset of the JSON property definition schema.
//!
//! Normal response codes: 201
//!
//! Error response codes: 400, 404
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::borrow::Cow;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Type {
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "string")]
    String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum TypeStringEnum {
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "string")]
    String,
}

/// Schema for the items in an array.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Items<'a> {
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) _enum: Option<Vec<Cow<'a, str>>>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) _type: Option<TypeStringEnum>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// Describes extra items, if you use tuple typing. If the value of `items`
    /// is an array (tuple typing) and the instance is longer than the list of
    /// schemas in `items`, the additional items are described by the schema in
    /// this property. If this value is `false`, the instance cannot be longer
    /// than the list of schemas in `items`. If this value is `true`, that is
    /// equivalent to the empty schema (anything goes).
    ///
    #[builder(default)]
    pub(crate) additional_items: Option<bool>,

    #[builder(default, setter(into))]
    pub(crate) _default: Option<Value>,

    /// Detailed description of the property.
    ///
    #[builder(default, setter(into))]
    pub(crate) description: Option<Cow<'a, str>>,

    /// Enumerated list of property values.
    ///
    #[builder(default, setter(into))]
    pub(crate) _enum: Option<Vec<Cow<'a, str>>>,

    /// Schema for the items in an array.
    ///
    #[builder(default, setter(into))]
    pub(crate) items: Option<Items<'a>>,

    /// Maximum allowed numerical value.
    ///
    #[builder(default)]
    pub(crate) maximum: Option<f32>,

    /// Maximum allowed string length.
    ///
    #[builder(default)]
    pub(crate) max_items: Option<i32>,

    /// Maximum allowed string length.
    ///
    #[builder(default)]
    pub(crate) max_length: Option<i32>,

    /// Minimum allowed numerical value.
    ///
    #[builder(default)]
    pub(crate) minimum: Option<f32>,

    /// Minimum allowed string length.
    ///
    #[builder(default)]
    pub(crate) min_items: Option<i32>,

    /// Minimum allowed string length.
    ///
    #[builder(default)]
    pub(crate) min_length: Option<i32>,

    #[builder(setter(into))]
    pub(crate) name: Cow<'a, str>,

    /// Operators property description.
    ///
    #[builder(default, setter(into))]
    pub(crate) operators: Option<Vec<Cow<'a, str>>>,

    /// A regular expression (
    /// [ECMA 262](http://www.ecma-international.org/publications/standards/Ecma-262.htm)
    /// ) that a string value must match.
    ///
    #[builder(default, setter(into))]
    pub(crate) pattern: Option<Cow<'a, str>>,

    /// Indicates whether this is a read-only property.
    ///
    #[builder(default)]
    pub(crate) readonly: Option<bool>,

    #[builder(default, setter(into))]
    pub(crate) required: Option<Vec<Cow<'a, str>>>,

    /// The title of the property.
    ///
    #[builder(setter(into))]
    pub(crate) title: Cow<'a, str>,

    /// The property type.
    ///
    #[builder()]
    pub(crate) _type: Type,

    /// Indicates whether all values in the array must be distinct.
    ///
    #[builder(default)]
    pub(crate) unique_items: Option<bool>,

    /// namespace_name parameter for
    /// /v2/metadefs/namespaces/{namespace_name}/properties/{property_name} API
    ///
    #[builder(default, setter(into))]
    namespace_name: Cow<'a, str>,

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
    /// Add a single header to the Property.
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
            "v2/metadefs/namespaces/{namespace_name}/properties",
            namespace_name = self.namespace_name.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("name", serde_json::to_value(&self.name)?);
        params.push("title", serde_json::to_value(&self.title)?);
        if let Some(val) = &self.description {
            params.push("description", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.operators {
            params.push("operators", serde_json::to_value(val)?);
        }
        params.push("type", serde_json::to_value(&self._type)?);
        if let Some(val) = &self.required {
            params.push("required", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.minimum {
            params.push("minimum", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.maximum {
            params.push("maximum", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.max_length {
            params.push("maxLength", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.min_length {
            params.push("minLength", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.pattern {
            params.push("pattern", serde_json::to_value(val)?);
        }
        if let Some(val) = &self._enum {
            params.push("enum", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.readonly {
            params.push("readonly", serde_json::to_value(val)?);
        }
        if let Some(val) = &self._default {
            params.push("default", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.items {
            params.push("items", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.max_items {
            params.push("maxItems", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.min_items {
            params.push("minItems", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.unique_items {
            params.push("uniqueItems", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.additional_items {
            params.push("additionalItems", serde_json::to_value(val)?);
        }

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Image
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
                .name("foo")
                .title("foo")
                ._type(Type::Array)
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Image
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Request::builder()
            .name("foo")
            .title("foo")
            ._type(Type::Array)
            .build()
            .unwrap()
            .response_key()
            .is_none())
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST).path(format!(
                "/v2/metadefs/namespaces/{namespace_name}/properties",
                namespace_name = "namespace_name",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .namespace_name("namespace_name")
            .name("foo")
            .title("foo")
            ._type(Type::Array)
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
                    "/v2/metadefs/namespaces/{namespace_name}/properties",
                    namespace_name = "namespace_name",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .namespace_name("namespace_name")
            .name("foo")
            .title("foo")
            ._type(Type::Array)
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
