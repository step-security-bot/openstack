use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum OsDcfDiskConfig {
    #[serde(rename = "AUTO")]
    Auto,
    #[serde(rename = "MANUAL")]
    Manual,
}

/// The action to resize a server.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Resize<'a> {
    /// The flavor ID for resizing the server. The size of the disk in the
    /// flavor
    /// being resized to must be greater than or equal to the size of the disk
    /// in
    /// the current flavor.
    ///
    ///
    /// If a specified flavor ID is the same as the current one of the server,
    /// the request returns a `Bad Request (400)` response code.
    #[serde(rename = "flavorRef")]
    #[builder(setter(into))]
    pub(crate) flavor_ref: Cow<'a, str>,

    /// Controls how the API partitions the disk when you create, rebuild, or
    /// resize servers.
    /// A server inherits the `OS-DCF:diskConfig` value from the image from
    /// which it
    /// was created, and an image inherits the `OS-DCF:diskConfig` value from
    /// the server
    /// from which it was created. To override the inherited setting, you can
    /// include
    /// this attribute in the request body of a server create, rebuild, or
    /// resize request. If
    /// the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot
    /// create
    /// a server from that image and set its `OS-DCF:diskConfig` value to
    /// `AUTO`.
    /// A valid value is:
    ///
    ///
    /// * `AUTO`. The API builds the server with a single partition the size of
    /// the
    /// target flavor disk. The API automatically adjusts the file system to
    /// fit the
    /// entire partition.
    /// * `MANUAL`. The API builds the server by using whatever partition
    /// scheme and
    /// file system is in the source image. If the target flavor disk is
    /// larger, the API
    /// does not partition the remaining disk space.
    #[serde(rename = "OS-DCF:diskConfig", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) os_dcf_disk_config: Option<OsDcfDiskConfig>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// The action to resize a server.
    #[builder(setter(into))]
    pub(crate) resize: Resize<'a>,

    /// id parameter for /v2.1/servers/{id}/action API
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
    /// Add a single header to the Server.
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
        format!("v2.1/servers/{id}/action", id = self.id.as_ref(),).into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("resize", serde_json::to_value(&self.resize)?);

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
                .resize(ResizeBuilder::default().flavor_ref("foo").build().unwrap())
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Compute
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Request::builder()
            .resize(ResizeBuilder::default().flavor_ref("foo").build().unwrap())
            .build()
            .unwrap()
            .response_key()
            .is_none())
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!("/v2.1/servers/{id}/action", id = "id",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .resize(ResizeBuilder::default().flavor_ref("foo").build().unwrap())
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
                .path(format!("/v2.1/servers/{id}/action", id = "id",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .resize(ResizeBuilder::default().flavor_ref("foo").build().unwrap())
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
