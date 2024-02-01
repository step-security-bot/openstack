//! Lists IDs, names, and links for servers.
//!
//! By default the servers are filtered using the project ID associated
//! with the authenticated request.
//!
//! Servers contain a status attribute that indicates the current server
//! state. You can filter on the server status when you complete a list
//! servers request. The server status is returned in the response
//! body. The possible server status values are:
//!
//! There is whitelist for valid filter keys. Any filter key other than from
//! whitelist will be silently ignored.
//!
//! Normal response codes: 200
//!
//! Error response codes: badRequest(400), unauthorized(401),
//! forbidden(403)
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use std::borrow::Cow;

use crate::api::Pageable;
#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    #[builder(default, setter(into))]
    user_id: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    project_id: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    tenant_id: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    launch_index: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    image_ref: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    image: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    kernel_id: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    ramdisk_id: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    hostname: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    key_name: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    power_state: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    vm_state: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    task_state: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    host: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    node: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    flavor: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    reservation_id: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    launched_at: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    terminated_at: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    availability_zone: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    display_name: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    display_description: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    locked_by: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    uuid: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    root_device_name: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    config_drive: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    access_ip_v4: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    access_ip_v6: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    auto_disk_config: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    progress: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    sort_key: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    sort_dir: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    all_tenants: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    soft_deleted: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    deleted: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    status: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    changes_since: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    ip: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    ip6: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    created_at: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    block_device_mapping: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    services: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    metadata: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    system_metadata: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    info_cache: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    security_groups: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    pci_devices: Option<Cow<'a, str>>,

    #[builder(default)]
    limit: Option<i32>,

    #[builder(default, setter(into))]
    marker: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    tags: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    tags_any: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    not_tags: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    not_tags_any: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    changes_before: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    locked: Option<Cow<'a, str>>,

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
        http::Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v2.1/servers".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("user_id", self.user_id.as_ref());
        params.push_opt("project_id", self.project_id.as_ref());
        params.push_opt("tenant_id", self.tenant_id.as_ref());
        params.push_opt("launch_index", self.launch_index.as_ref());
        params.push_opt("image_ref", self.image_ref.as_ref());
        params.push_opt("image", self.image.as_ref());
        params.push_opt("kernel_id", self.kernel_id.as_ref());
        params.push_opt("ramdisk_id", self.ramdisk_id.as_ref());
        params.push_opt("hostname", self.hostname.as_ref());
        params.push_opt("key_name", self.key_name.as_ref());
        params.push_opt("power_state", self.power_state.as_ref());
        params.push_opt("vm_state", self.vm_state.as_ref());
        params.push_opt("task_state", self.task_state.as_ref());
        params.push_opt("host", self.host.as_ref());
        params.push_opt("node", self.node.as_ref());
        params.push_opt("flavor", self.flavor.as_ref());
        params.push_opt("reservation_id", self.reservation_id.as_ref());
        params.push_opt("launched_at", self.launched_at.as_ref());
        params.push_opt("terminated_at", self.terminated_at.as_ref());
        params.push_opt("availability_zone", self.availability_zone.as_ref());
        params.push_opt("name", self.name.as_ref());
        params.push_opt("display_name", self.display_name.as_ref());
        params.push_opt("description", self.description.as_ref());
        params.push_opt("display_description", self.display_description.as_ref());
        params.push_opt("locked_by", self.locked_by.as_ref());
        params.push_opt("uuid", self.uuid.as_ref());
        params.push_opt("root_device_name", self.root_device_name.as_ref());
        params.push_opt("config_drive", self.config_drive.as_ref());
        params.push_opt("access_ip_v4", self.access_ip_v4.as_ref());
        params.push_opt("access_ip_v6", self.access_ip_v6.as_ref());
        params.push_opt("auto_disk_config", self.auto_disk_config.as_ref());
        params.push_opt("progress", self.progress.as_ref());
        params.push_opt("sort_key", self.sort_key.as_ref());
        params.push_opt("sort_dir", self.sort_dir.as_ref());
        params.push_opt("all_tenants", self.all_tenants.as_ref());
        params.push_opt("soft_deleted", self.soft_deleted.as_ref());
        params.push_opt("deleted", self.deleted.as_ref());
        params.push_opt("status", self.status.as_ref());
        params.push_opt("changes-since", self.changes_since.as_ref());
        params.push_opt("ip", self.ip.as_ref());
        params.push_opt("ip6", self.ip6.as_ref());
        params.push_opt("created_at", self.created_at.as_ref());
        params.push_opt("block_device_mapping", self.block_device_mapping.as_ref());
        params.push_opt("services", self.services.as_ref());
        params.push_opt("metadata", self.metadata.as_ref());
        params.push_opt("system_metadata", self.system_metadata.as_ref());
        params.push_opt("info_cache", self.info_cache.as_ref());
        params.push_opt("security_groups", self.security_groups.as_ref());
        params.push_opt("pci_devices", self.pci_devices.as_ref());
        params.push_opt("limit", self.limit);
        params.push_opt("marker", self.marker.as_ref());
        params.push_opt("tags", self.tags.as_ref());
        params.push_opt("tags-any", self.tags_any.as_ref());
        params.push_opt("not-tags", self.not_tags.as_ref());
        params.push_opt("not-tags-any", self.not_tags_any.as_ref());
        params.push_opt("changes-before", self.changes_before.as_ref());
        params.push_opt("locked", self.locked.as_ref());

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Compute
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("servers".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }
}
impl<'a> Pageable for Request<'a> {}

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
            Request::builder().build().unwrap().service_type(),
            ServiceType::Compute
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder().build().unwrap().response_key().unwrap(),
            "servers"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.1/servers".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "servers": {} }));
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
                .path("/v2.1/servers".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "servers": {} }));
        });

        let endpoint = Request::builder()
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
