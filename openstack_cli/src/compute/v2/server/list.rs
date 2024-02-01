use async_trait::async_trait;
use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

use openstack_sdk::api::compute::v2::server::list_detailed;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use structable_derive::StructTable;

/// For each server, shows server details including config drive,
/// extended status, and server usage information.
///
/// The extended status information appears in the OS-EXT-STS:vm\_state,
/// OS-EXT-STS:power\_state, and OS-EXT-STS:task\_state attributes.
///
/// The server usage information appears in the OS-SRV-USG:launched\_at and
/// OS-SRV-USG:terminated\_at attributes.
///
/// HostId is unique per account and is not globally unique.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401),
/// forbidden(403)
#[derive(Args, Clone, Debug)]
#[command(about = "List Servers Detailed")]
pub struct ServersArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Total limit of entities count to return. Use this when there are too many entries.
    #[arg(long, default_value_t = 10000)]
    max_items: usize,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {
    #[arg(long)]
    user_id: Option<String>,

    #[arg(long)]
    project_id: Option<String>,

    #[arg(long)]
    tenant_id: Option<String>,

    #[arg(long)]
    launch_index: Option<String>,

    #[arg(long)]
    image_ref: Option<String>,

    #[arg(long)]
    image: Option<String>,

    #[arg(long)]
    kernel_id: Option<String>,

    #[arg(long)]
    ramdisk_id: Option<String>,

    #[arg(long)]
    hostname: Option<String>,

    #[arg(long)]
    key_name: Option<String>,

    #[arg(long)]
    power_state: Option<String>,

    #[arg(long)]
    vm_state: Option<String>,

    #[arg(long)]
    task_state: Option<String>,

    #[arg(long)]
    host: Option<String>,

    #[arg(long)]
    node: Option<String>,

    #[arg(long)]
    flavor: Option<String>,

    #[arg(long)]
    reservation_id: Option<String>,

    #[arg(long)]
    launched_at: Option<String>,

    #[arg(long)]
    terminated_at: Option<String>,

    #[arg(long)]
    availability_zone: Option<String>,

    #[arg(long)]
    name: Option<String>,

    #[arg(long)]
    display_name: Option<String>,

    #[arg(long)]
    description: Option<String>,

    #[arg(long)]
    display_description: Option<String>,

    #[arg(long)]
    locked_by: Option<String>,

    #[arg(long)]
    uuid: Option<String>,

    #[arg(long)]
    root_device_name: Option<String>,

    #[arg(long)]
    config_drive: Option<String>,

    #[arg(long)]
    access_ip_v4: Option<String>,

    #[arg(long)]
    access_ip_v6: Option<String>,

    #[arg(long)]
    auto_disk_config: Option<String>,

    #[arg(long)]
    progress: Option<String>,

    #[arg(long)]
    sort_key: Option<String>,

    #[arg(long)]
    sort_dir: Option<String>,

    #[arg(long)]
    all_tenants: Option<String>,

    #[arg(long)]
    soft_deleted: Option<String>,

    #[arg(long)]
    deleted: Option<String>,

    #[arg(long)]
    status: Option<String>,

    #[arg(long)]
    changes_since: Option<String>,

    #[arg(long)]
    ip: Option<String>,

    #[arg(long)]
    ip6: Option<String>,

    #[arg(long)]
    created_at: Option<String>,

    #[arg(long)]
    block_device_mapping: Option<String>,

    #[arg(long)]
    services: Option<String>,

    #[arg(long)]
    metadata: Option<String>,

    #[arg(long)]
    system_metadata: Option<String>,

    #[arg(long)]
    info_cache: Option<String>,

    #[arg(long)]
    security_groups: Option<String>,

    #[arg(long)]
    pci_devices: Option<String>,

    #[arg(long)]
    limit: Option<i32>,

    #[arg(long)]
    marker: Option<String>,

    #[arg(long)]
    tags: Option<String>,

    #[arg(long)]
    tags_any: Option<String>,

    #[arg(long)]
    not_tags: Option<String>,

    #[arg(long)]
    not_tags_any: Option<String>,

    #[arg(long)]
    changes_before: Option<String>,

    #[arg(long)]
    locked: Option<String>,
}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {}

/// Servers list command
pub struct ServersCmd {
    pub args: ServersArgs,
}
/// Servers response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// IPv4 address that should be used to access this server. May be
    /// automatically set by the provider.
    #[serde(rename = "accessIPv4")]
    #[structable(optional, title = "accessIPv4", wide)]
    access_ipv4: Option<String>,

    /// IPv6 address that should be used to access this server. May be
    /// automatically set by the provider.
    #[serde(rename = "accessIPv6")]
    #[structable(optional, title = "accessIPv6", wide)]
    access_ipv6: Option<String>,

    /// The addresses for the server. Servers with status `BUILD` hide their
    /// addresses information.
    #[serde()]
    #[structable(optional, wide)]
    addresses: Option<HashMapStringVecResponseAddresses>,

    /// The attached volumes, if any.
    #[serde(rename = "os-extended-volumes:volumes_attached")]
    #[structable(optional, title = "os-extended-volumes:volumes_attached", wide)]
    os_extended_volumes_volumes_attached: Option<VecHashMapStringValue>,

    /// The availability zone name.
    #[serde(rename = "OS-EXT-AZ:availability_zone")]
    #[structable(optional, title = "OS-EXT-AZ:availability_zone", wide)]
    os_ext_az_availability_zone: Option<String>,

    /// The name of the compute host on which this instance is running.
    /// Appears in the response for administrative users only.
    #[serde(rename = "OS-EXT-SRV-ATTR:host")]
    #[structable(optional, title = "OS-EXT-SRV-ATTR:host", wide)]
    os_ext_srv_attr_host: Option<String>,

    /// Indicates whether or not a config drive was used for this server.
    /// The value is `True` or an empty string. An empty string stands for
    /// `False`.
    #[serde()]
    #[structable(optional, wide)]
    config_drive: Option<String>,

    /// The date and time when the resource was created. The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm`
    /// value, if included, is the time zone as an offset from UTC. In
    /// the previous example, the offset value is `-05:00`.
    #[serde()]
    #[structable(optional, wide)]
    created: Option<String>,

    /// The description of the server.
    /// Before microversion 2.19 this was set to the server name.
    ///
    ///
    /// **New in version 2.19**
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// Disk configuration. The value is either:
    ///
    ///
    /// * `AUTO`. The API builds the server with a single partition the size of
    /// the target flavor disk. The API automatically adjusts the file system
    /// to
    /// fit the entire partition.
    /// * `MANUAL`. The API builds the server by using the partition scheme and
    /// file system that is in the source image. If the target flavor disk is
    /// larger, The API does not partition the remaining disk space.
    #[serde(rename = "OS-DCF:diskConfig")]
    #[structable(optional, title = "OS-DCF:diskConfig", wide)]
    os_dcf_disk_config: Option<String>,

    /// A fault object. Only displayed when the server status is `ERROR` or
    /// `DELETED` and a fault occurred.
    #[serde()]
    #[structable(optional, wide)]
    fault: Option<ResponseFault>,

    /// Before microversion 2.47 this contains the ID and links for the flavor
    /// used to boot the server instance. This can be an empty object in case
    /// flavor information is no longer present in the system.
    ///
    ///
    /// As of microversion 2.47 this contains a subset of the actual flavor
    /// information used to create the server instance, represented as a nested
    /// dictionary.
    #[serde()]
    #[structable(optional, wide)]
    flavor: Option<ResponseFlavor>,

    /// An ID string representing the host. This is a hashed value so will not
    /// actually look like
    /// a hostname, and is hashed with data from the project\_id, so the same
    /// physical host as seen
    /// by two different project\_ids, will be different. It is useful when
    /// within the same project you
    /// need to determine if two instances are on the same or different
    /// physical hosts for the
    /// purposes of availability or performance.
    #[serde(rename = "hostId")]
    #[structable(optional, title = "hostId", wide)]
    host_id: Option<String>,

    /// The host status. Values where next value in list can override the
    /// previous:
    ///
    ///
    /// * `UP` if nova-compute up.
    /// * `UNKNOWN` if nova-compute not reported by servicegroup driver.
    /// * `DOWN` if nova-compute forced down.
    /// * `MAINTENANCE` if nova-compute is disabled.
    /// * Empty string indicates there is no host for server.
    ///
    ///
    /// This attribute appears in the response only if the policy permits.
    /// By default, only administrators can get this parameter.
    ///
    ///
    /// **New in version 2.16**
    #[serde()]
    #[structable(optional, wide)]
    host_status: Option<String>,

    /// The hostname of the instance reported in the metadata service.
    /// This parameter only appears in responses for administrators until
    /// microversion 2.90, after which it is shown for all users.
    ///
    ///
    ///
    /// Note
    ///
    ///
    /// This information is published via the metadata service and requires
    /// application such as `cloud-init` to propogate it through to the
    /// instance.
    ///
    ///
    ///
    /// **New in version 2.3**
    #[serde(rename = "OS-EXT-SRV-ATTR:hostname")]
    #[structable(optional, title = "OS-EXT-SRV-ATTR:hostname", wide)]
    os_ext_srv_attr_hostname: Option<String>,

    /// The hypervisor host name provided by the Nova virt driver. For the
    /// Ironic driver,
    /// it is the Ironic node uuid. Appears in the response for administrative
    /// users only.
    #[serde(rename = "OS-EXT-SRV-ATTR:hypervisor_hostname")]
    #[structable(optional, title = "OS-EXT-SRV-ATTR:hypervisor_hostname", wide)]
    os_ext_srv_attr_hypervisor_hostname: Option<String>,

    /// Id of the server
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The UUID and links for the image for your server instance. The `image`
    /// object
    /// will be an empty string when you boot the server from a volume.
    #[serde()]
    #[structable(optional, wide)]
    image: Option<ResponseImage>,

    /// The instance name. The Compute API generates the instance name from the
    /// instance
    /// name template. Appears in the response for administrative users only.
    #[serde(rename = "OS-EXT-SRV-ATTR:instance_name")]
    #[structable(optional, title = "OS-EXT-SRV-ATTR:instance_name", wide)]
    os_ext_srv_attr_instance_name: Option<String>,

    /// True if the instance is locked otherwise False.
    ///
    ///
    /// **New in version 2.9**
    #[serde()]
    #[structable(optional, wide)]
    locked: Option<bool>,

    /// The UUID of the kernel image when using an AMI. Will be null if not.
    /// By default, it appears in the response for administrative users only.
    ///
    ///
    /// **New in version 2.3**
    #[serde(rename = "OS-EXT-SRV-ATTR:kernel_id")]
    #[structable(optional, title = "OS-EXT-SRV-ATTR:kernel_id", wide)]
    os_ext_srv_attr_kernel_id: Option<String>,

    /// The name of associated key pair, if any.
    #[serde()]
    #[structable(optional, wide)]
    key_name: Option<String>,

    /// When servers are launched via multiple create, this is the
    /// sequence in which the servers were launched.
    /// By default, it appears in the response for administrative users only.
    ///
    ///
    /// **New in version 2.3**
    #[serde(rename = "OS-EXT-SRV-ATTR:launch_index")]
    #[structable(optional, title = "OS-EXT-SRV-ATTR:launch_index", wide)]
    os_ext_srv_attr_launch_index: Option<i32>,

    /// The date and time when the server was launched.
    ///
    ///
    /// The date and time stamp format is [ISO
    /// 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    ///
    /// The `hh±:mm` value, if included, is the time zone as an offset from
    /// UTC.
    /// If the `deleted\_at` date and time stamp is not set, its value is
    /// `null`.
    #[serde(rename = "OS-SRV-USG:launched_at")]
    #[structable(optional, title = "OS-SRV-USG:launched_at", wide)]
    os_srv_usg_launched_at: Option<String>,

    /// Links to the resources in question. See [API Guide / Links and
    /// References](https://docs.openstack.org/api-
    /// guide/compute/links_and_references.html) for more info.
    #[serde()]
    #[structable(optional, wide)]
    links: Option<Value>,

    /// A dictionary of metadata key-and-value pairs, which is maintained for
    /// backward
    /// compatibility.
    #[serde()]
    #[structable(optional, wide)]
    metadata: Option<HashMapStringString>,

    /// The server name.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The power state of the instance. This is an enum value that is mapped
    /// as:
    ///
    ///
    ///
    /// ```text
    /// 0: NOSTATE
    /// 1: RUNNING
    /// 3: PAUSED
    /// 4: SHUTDOWN
    /// 6: CRASHED
    /// 7: SUSPENDED
    ///
    /// ```
    #[serde(rename = "OS-EXT-STS:power_state")]
    #[structable(optional, title = "OS-EXT-STS:power_state", wide)]
    os_ext_sts_power_state: Option<i32>,

    /// A percentage value of the operation progress.
    /// This parameter only appears when the server status is `ACTIVE`,
    /// `BUILD`, `REBUILD`, `RESIZE`, `VERIFY\_RESIZE` or `MIGRATING`.
    #[serde()]
    #[structable(optional, wide)]
    progress: Option<i32>,

    /// The UUID of the tenant in a multi-tenancy cloud.
    #[serde()]
    #[structable(optional, wide)]
    tenant_id: Option<String>,

    /// The UUID of the ramdisk image when using an AMI. Will be null if not.
    /// By default, it appears in the response for administrative users only.
    ///
    ///
    /// **New in version 2.3**
    #[serde(rename = "OS-EXT-SRV-ATTR:ramdisk_id")]
    #[structable(optional, title = "OS-EXT-SRV-ATTR:ramdisk_id", wide)]
    os_ext_srv_attr_ramdisk_id: Option<String>,

    /// The reservation id for the server. This is an id that can
    /// be useful in tracking groups of servers created with multiple
    /// create, that will all have the same reservation\_id.
    /// By default, it appears in the response for administrative users only.
    ///
    ///
    /// **New in version 2.3**
    #[serde(rename = "OS-EXT-SRV-ATTR:reservation_id")]
    #[structable(optional, title = "OS-EXT-SRV-ATTR:reservation_id", wide)]
    os_ext_srv_attr_reservation_id: Option<String>,

    /// The root device name for the instance
    /// By default, it appears in the response for administrative users only.
    ///
    ///
    /// **New in version 2.3**
    #[serde(rename = "OS-EXT-SRV-ATTR:root_device_name")]
    #[structable(optional, title = "OS-EXT-SRV-ATTR:root_device_name", wide)]
    os_ext_srv_attr_root_device_name: Option<String>,

    /// One or more security groups objects.
    #[serde()]
    #[structable(optional, wide)]
    security_groups: Option<VecResponseSecurityGroups>,

    /// The UUIDs of the server groups to which the server belongs. Currently
    /// this can contain at most one entry.
    #[serde()]
    #[structable(optional, wide)]
    server_groups: Option<VecString>,

    /// The server status.
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// A list of tags. The maximum count of tags in this list is 50.
    ///
    ///
    /// **New in version 2.26**
    #[serde()]
    #[structable(optional, wide)]
    tags: Option<VecString>,

    /// The task state of the instance.
    #[serde(rename = "OS-EXT-STS:task_state")]
    #[structable(optional, title = "OS-EXT-STS:task_state", wide)]
    os_ext_sts_task_state: Option<String>,

    /// The date and time when the server was deleted.
    ///
    ///
    /// The date and time stamp format is [ISO
    /// 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    /// The `±hh:mm` value, if included, is the time zone as an offset from
    /// UTC.
    /// If the `deleted\_at` date and time stamp is not set, its value is
    /// `null`.
    #[serde(rename = "OS-SRV-USG:terminated_at")]
    #[structable(optional, title = "OS-SRV-USG:terminated_at", wide)]
    os_srv_usg_terminated_at: Option<String>,

    /// A list of trusted certificate IDs, that were used during image
    /// signature
    /// verification to verify the signing certificate. The list is restricted
    /// to a maximum of 50 IDs. The value is `null` if trusted certificate IDs
    /// are not set.
    ///
    ///
    /// **New in version 2.63**
    #[serde()]
    #[structable(optional, wide)]
    trusted_image_certificates: Option<VecString>,

    /// The date and time when the resource was updated. The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm`
    /// value, if included, is the time zone as an offset from UTC. In
    /// the previous example, the offset value is `-05:00`.
    #[serde()]
    #[structable(optional, wide)]
    updated: Option<String>,

    /// The user\_data the instance was created with.
    /// By default, it appears in the response for administrative users only.
    ///
    ///
    /// **New in version 2.3**
    #[serde(rename = "OS-EXT-SRV-ATTR:user_data")]
    #[structable(optional, title = "OS-EXT-SRV-ATTR:user_data", wide)]
    os_ext_srv_attr_user_data: Option<String>,

    /// The user ID of the user who owns the server.
    #[serde()]
    #[structable(optional, wide)]
    user_id: Option<String>,

    /// The VM state.
    #[serde(rename = "OS-EXT-STS:vm_state")]
    #[structable(optional, title = "OS-EXT-STS:vm_state", wide)]
    os_ext_sts_vm_state: Option<String>,
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseAddresses {
    addr: Option<String>,
    version: Option<i32>,
}

impl fmt::Display for ResponseAddresses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "addr={}",
                self.addr
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "version={}",
                self.version
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseAddresses(Vec<ResponseAddresses>);
impl fmt::Display for VecResponseAddresses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct HashMapStringVecResponseAddresses(HashMap<String, VecResponseAddresses>);
impl fmt::Display for HashMapStringVecResponseAddresses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|v| format!("{}={}", v.0, v.1))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct HashMapStringValue(HashMap<String, Value>);
impl fmt::Display for HashMapStringValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|v| format!("{}={}", v.0, v.1))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecHashMapStringValue(Vec<HashMapStringValue>);
impl fmt::Display for VecHashMapStringValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseFault {
    code: Option<i32>,
    created: Option<String>,
    message: Option<String>,
    details: Option<String>,
}

impl fmt::Display for ResponseFault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "code={}",
                self.code.map(|v| v.to_string()).unwrap_or("".to_string())
            ),
            format!(
                "created={}",
                self.created
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "message={}",
                self.message
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "details={}",
                self.details
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseLinksStructResponseStructResponse {
    href: Option<String>,
    rel: Option<String>,
}

impl fmt::Display for ResponseLinksStructResponseStructResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "href={}",
                self.href
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "rel={}",
                self.rel
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct HashMapStringString(HashMap<String, String>);
impl fmt::Display for HashMapStringString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|v| format!("{}={}", v.0, v.1))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseFlavor {
    id: Option<String>,
    links: Option<Value>,
    vcpus: Option<i32>,
    ram: Option<i32>,
    disk: Option<i32>,
    ephemeral: Option<i32>,
    swap: Option<i32>,
    original_name: Option<String>,
    extra_specs: Option<HashMapStringString>,
}

impl fmt::Display for ResponseFlavor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "links={}",
                self.links
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "vcpus={}",
                self.vcpus.map(|v| v.to_string()).unwrap_or("".to_string())
            ),
            format!(
                "ram={}",
                self.ram.map(|v| v.to_string()).unwrap_or("".to_string())
            ),
            format!(
                "disk={}",
                self.disk.map(|v| v.to_string()).unwrap_or("".to_string())
            ),
            format!(
                "ephemeral={}",
                self.ephemeral
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "swap={}",
                self.swap.map(|v| v.to_string()).unwrap_or("".to_string())
            ),
            format!(
                "original_name={}",
                self.original_name
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "extra_specs={}",
                self.extra_specs
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseImage {
    id: Option<String>,
    links: Option<Value>,
}

impl fmt::Display for ResponseImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "links={}",
                self.links
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseSecurityGroups {
    name: Option<String>,
}

impl fmt::Display for ResponseSecurityGroups {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([format!(
            "name={}",
            self.name
                .clone()
                .map(|v| v.to_string())
                .unwrap_or("".to_string())
        )]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseSecurityGroups(Vec<ResponseSecurityGroups>);
impl fmt::Display for VecResponseSecurityGroups {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecString(Vec<String>);
impl fmt::Display for VecString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

#[async_trait]
impl OSCCommand for ServersCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Servers with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = list_detailed::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.args.query.user_id {
            ep_builder.user_id(val.clone());
        }
        if let Some(val) = &self.args.query.project_id {
            ep_builder.project_id(val.clone());
        }
        if let Some(val) = &self.args.query.tenant_id {
            ep_builder.tenant_id(val.clone());
        }
        if let Some(val) = &self.args.query.launch_index {
            ep_builder.launch_index(val.clone());
        }
        if let Some(val) = &self.args.query.image_ref {
            ep_builder.image_ref(val.clone());
        }
        if let Some(val) = &self.args.query.image {
            ep_builder.image(val.clone());
        }
        if let Some(val) = &self.args.query.kernel_id {
            ep_builder.kernel_id(val.clone());
        }
        if let Some(val) = &self.args.query.ramdisk_id {
            ep_builder.ramdisk_id(val.clone());
        }
        if let Some(val) = &self.args.query.hostname {
            ep_builder.hostname(val.clone());
        }
        if let Some(val) = &self.args.query.key_name {
            ep_builder.key_name(val.clone());
        }
        if let Some(val) = &self.args.query.power_state {
            ep_builder.power_state(val.clone());
        }
        if let Some(val) = &self.args.query.vm_state {
            ep_builder.vm_state(val.clone());
        }
        if let Some(val) = &self.args.query.task_state {
            ep_builder.task_state(val.clone());
        }
        if let Some(val) = &self.args.query.host {
            ep_builder.host(val.clone());
        }
        if let Some(val) = &self.args.query.node {
            ep_builder.node(val.clone());
        }
        if let Some(val) = &self.args.query.flavor {
            ep_builder.flavor(val.clone());
        }
        if let Some(val) = &self.args.query.reservation_id {
            ep_builder.reservation_id(val.clone());
        }
        if let Some(val) = &self.args.query.launched_at {
            ep_builder.launched_at(val.clone());
        }
        if let Some(val) = &self.args.query.terminated_at {
            ep_builder.terminated_at(val.clone());
        }
        if let Some(val) = &self.args.query.availability_zone {
            ep_builder.availability_zone(val.clone());
        }
        if let Some(val) = &self.args.query.name {
            ep_builder.name(val.clone());
        }
        if let Some(val) = &self.args.query.display_name {
            ep_builder.display_name(val.clone());
        }
        if let Some(val) = &self.args.query.description {
            ep_builder.description(val.clone());
        }
        if let Some(val) = &self.args.query.display_description {
            ep_builder.display_description(val.clone());
        }
        if let Some(val) = &self.args.query.locked_by {
            ep_builder.locked_by(val.clone());
        }
        if let Some(val) = &self.args.query.uuid {
            ep_builder.uuid(val.clone());
        }
        if let Some(val) = &self.args.query.root_device_name {
            ep_builder.root_device_name(val.clone());
        }
        if let Some(val) = &self.args.query.config_drive {
            ep_builder.config_drive(val.clone());
        }
        if let Some(val) = &self.args.query.access_ip_v4 {
            ep_builder.access_ip_v4(val.clone());
        }
        if let Some(val) = &self.args.query.access_ip_v6 {
            ep_builder.access_ip_v6(val.clone());
        }
        if let Some(val) = &self.args.query.auto_disk_config {
            ep_builder.auto_disk_config(val.clone());
        }
        if let Some(val) = &self.args.query.progress {
            ep_builder.progress(val.clone());
        }
        if let Some(val) = &self.args.query.sort_key {
            ep_builder.sort_key(val.clone());
        }
        if let Some(val) = &self.args.query.sort_dir {
            ep_builder.sort_dir(val.clone());
        }
        if let Some(val) = &self.args.query.all_tenants {
            ep_builder.all_tenants(val.clone());
        }
        if let Some(val) = &self.args.query.soft_deleted {
            ep_builder.soft_deleted(val.clone());
        }
        if let Some(val) = &self.args.query.deleted {
            ep_builder.deleted(val.clone());
        }
        if let Some(val) = &self.args.query.status {
            ep_builder.status(val.clone());
        }
        if let Some(val) = &self.args.query.changes_since {
            ep_builder.changes_since(val.clone());
        }
        if let Some(val) = &self.args.query.ip {
            ep_builder.ip(val.clone());
        }
        if let Some(val) = &self.args.query.ip6 {
            ep_builder.ip6(val.clone());
        }
        if let Some(val) = &self.args.query.created_at {
            ep_builder.created_at(val.clone());
        }
        if let Some(val) = &self.args.query.block_device_mapping {
            ep_builder.block_device_mapping(val.clone());
        }
        if let Some(val) = &self.args.query.services {
            ep_builder.services(val.clone());
        }
        if let Some(val) = &self.args.query.metadata {
            ep_builder.metadata(val.clone());
        }
        if let Some(val) = &self.args.query.system_metadata {
            ep_builder.system_metadata(val.clone());
        }
        if let Some(val) = &self.args.query.info_cache {
            ep_builder.info_cache(val.clone());
        }
        if let Some(val) = &self.args.query.security_groups {
            ep_builder.security_groups(val.clone());
        }
        if let Some(val) = &self.args.query.pci_devices {
            ep_builder.pci_devices(val.clone());
        }
        if let Some(val) = &self.args.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.args.query.marker {
            ep_builder.marker(val.clone());
        }
        if let Some(val) = &self.args.query.tags {
            ep_builder.tags(val.clone());
        }
        if let Some(val) = &self.args.query.tags_any {
            ep_builder.tags_any(val.clone());
        }
        if let Some(val) = &self.args.query.not_tags {
            ep_builder.not_tags(val.clone());
        }
        if let Some(val) = &self.args.query.not_tags_any {
            ep_builder.not_tags_any(val.clone());
        }
        if let Some(val) = &self.args.query.changes_before {
            ep_builder.changes_before(val.clone());
        }
        if let Some(val) = &self.args.query.locked {
            ep_builder.locked(val.clone());
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.args.max_items))
            .query_async(client)
            .await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
