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

//! Set Port command
//!
//! Wraps invoking of the `v2.0/ports/{port_id}` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::parse_json;
use crate::common::parse_key_val;
use crate::common::BoolString;
use clap::ValueEnum;
use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::port::find;
use openstack_sdk::api::network::v2::port::set;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Updates a port.
///
/// You can update information for a port, such as its symbolic name and
/// associated IPs. When you update IPs for a port, any previously associated
/// IPs are removed, returned to the respective subnet allocation pools, and
/// replaced by the IPs in the request body. Therefore, this operation replaces
/// the `fixed_ip` attribute when you specify it in the request body. If the
/// updated IP addresses are not valid or are already in use, the operation
/// fails and the existing IP addresses are not removed from the port.
///
/// When you update security groups for a port and the operation succeeds, any
/// associated security groups are removed and replaced by the security groups
/// in the request body. Therefore, this operation replaces the
/// `security_groups` attribute when you specify it in the request body. If the
/// security groups are not valid, the operation fails and the existing
/// security groups are not removed from the port.
///
/// When you update `binding:profile` of a port with null it is treated as {}
/// in the response.
///
/// Only admins and users with a specific role can update the data plane status
/// (default role: `data_plane_integrator`).
///
/// Normal response codes: 200
///
/// Error response codes: 400, 401, 403, 404, 409, 412
///
#[derive(Args)]
#[command(about = "Update port")]
pub struct PortCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    port: Port,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// port_id parameter for /v2.0/ports/{port_id}/add_allowed_address_pairs
    /// API
    ///
    #[arg(id = "path_param_id", value_name = "ID")]
    id: String,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum DataPlaneStatus {
    Active,
    Down,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum NumaAffinityPolicy {
    Legacy,
    Preferred,
    Required,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum BindingVnicType {
    AcceleratorDirect,
    AcceleratorDirectPhysical,
    Baremetal,
    Direct,
    DirectPhysical,
    Macvtap,
    Normal,
    RemoteManaged,
    SmartNic,
    Vdpa,
    VirtioForwarder,
}

/// Port Body data
#[derive(Args)]
struct Port {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`). Default is `true`.
    ///
    #[arg(action=clap::ArgAction::Set, long)]
    admin_state_up: Option<bool>,

    /// A set of zero or more allowed address pair objects each where address
    /// pair object contains an `ip_address` and `mac_address`. While the
    /// `ip_address` is required, the `mac_address` will be taken from the port
    /// if not specified. The value of `ip_address` can be an IP Address or a
    /// CIDR (if supported by the underlying extension plugin). A server
    /// connected to the port can send a packet with source address which
    /// matches one of the specified allowed address pairs.
    ///
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    allowed_address_pairs: Option<Vec<Value>>,

    /// The ID of the host where the port resides. The default is an empty
    /// string.
    ///
    #[arg(long)]
    binding_host_id: Option<String>,

    /// A dictionary that enables the application running on the specific host
    /// to pass and receive vif port information specific to the networking
    /// back-end. This field is only meant for machine-machine communication
    /// for compute services like Nova, Ironic or Zun to pass information to a
    /// Neutron back-end. It should not be used by multiple services
    /// concurrently or by cloud end users. The existing counterexamples
    /// (`capabilities: [switchdev]` for Open vSwitch hardware offload and
    /// `trusted=true` for Trusted Virtual Functions) are due to be cleaned up.
    /// The networking API does not define a specific format of this field. The
    /// default is an empty dictionary. If you update it with null then it is
    /// treated like {} in the response. Since the port-mac-address-override
    /// extension the `device_mac_address` field of the binding:profile can be
    /// used to provide the MAC address of the physical device a
    /// direct-physical port is being bound to. If provided, then the
    /// `mac_address` field of the port resource will be updated to the MAC
    /// from the active binding.
    ///
    #[arg(long, value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    binding_profile: Option<Vec<(String, Value)>>,

    /// The type of vNIC which this port should be attached to. This is used to
    /// determine which mechanism driver(s) to be used to bind the port. The
    /// valid values are `normal`, `macvtap`, `direct`, `baremetal`,
    /// `direct-physical`, `virtio-forwarder`, `smart-nic` and
    /// `remote-managed`. What type of vNIC is actually available depends on
    /// deployments. The default is `normal`.
    ///
    #[arg(long)]
    binding_vnic_type: Option<BindingVnicType>,

    /// Status of the underlying data plane of a port.
    ///
    #[arg(long)]
    data_plane_status: Option<DataPlaneStatus>,

    /// A human-readable description for the resource. Default is an empty
    /// string.
    ///
    #[arg(long)]
    description: Option<String>,

    /// The ID of the device that uses this port. For example, a server
    /// instance or a logical router.
    ///
    #[arg(long)]
    device_id: Option<String>,

    /// The entity type that uses this port. For example, `compute:nova`
    /// (server instance), `network:dhcp` (DHCP agent) or
    /// `network:router_interface` (router interface).
    ///
    #[arg(long)]
    device_owner: Option<String>,

    /// A valid DNS domain.
    ///
    #[arg(long)]
    dns_domain: Option<String>,

    /// A valid DNS name.
    ///
    #[arg(long)]
    dns_name: Option<String>,

    /// A set of zero or more extra DHCP option pairs. An option pair consists
    /// of an option value and name.
    ///
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    extra_dhcp_opts: Option<Vec<Value>>,

    /// The IP addresses for the port. If you would like to assign multiple IP
    /// addresses for the port, specify multiple entries in this field. Each
    /// entry consists of IP address (`ip_address`) and the subnet ID from
    /// which the IP address is assigned (`subnet_id`).
    ///
    /// - If you specify both a subnet ID and an IP address, OpenStack
    ///   Networking tries to allocate the IP address on that subnet to the
    ///   port.
    /// - If you specify only a subnet ID, OpenStack Networking allocates an
    ///   available IP from that subnet to the port.
    /// - If you specify only an IP address, OpenStack Networking tries to
    ///   allocate the IP address if the address is a valid IP for any of the
    ///   subnets on the specified network.
    ///
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    fixed_ips: Option<Vec<Value>>,

    /// Admin-only. A dict, at the top level keyed by mechanism driver aliases
    /// (as defined in setup.cfg). To following values can be used to control
    /// Open vSwitch’s Userspace Tx packet steering feature:
    ///
    /// - `{"openvswitch": {"other_config": {"tx-steering": "hash"}}}`
    /// - `{"openvswitch": {"other_config": {"tx-steering": "thread"}}}`
    ///
    /// If omitted the default is defined by Open vSwitch. The field cannot be
    /// longer than 4095 characters.
    ///
    #[arg(long, value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    hints: Option<Vec<(String, Value)>>,

    /// The MAC address of the port. By default, only administrative users and
    /// users with advsvc role can change this value.
    ///
    #[arg(long)]
    mac_address: Option<String>,

    /// Human-readable name of the resource. Default is an empty string.
    ///
    #[arg(long)]
    name: Option<String>,

    /// The port NUMA affinity policy requested during the virtual machine
    /// scheduling. Values: `None`, `required`, `preferred` or `legacy`.
    ///
    #[arg(long)]
    numa_affinity_policy: Option<NumaAffinityPolicy>,

    /// The port security status. A valid value is enabled (`true`) or disabled
    /// (`false`). If port security is enabled for the port, security group
    /// rules and anti-spoofing rules are applied to the traffic on the port.
    /// If disabled, no such rules are applied.
    ///
    #[arg(action=clap::ArgAction::Set, long)]
    port_security_enabled: Option<bool>,

    /// QoS policy associated with the port.
    ///
    #[arg(long)]
    qos_policy_id: Option<String>,

    /// The IDs of security groups applied to the port.
    ///
    #[arg(action=clap::ArgAction::Append, long)]
    security_groups: Option<Vec<String>>,
}

/// Port response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde()]
    #[structable(optional)]
    admin_state_up: Option<BoolString>,

    /// A set of zero or more allowed address pair objects each where address
    /// pair object contains an `ip_address` and `mac_address`. While the
    /// `ip_address` is required, the `mac_address` will be taken from the port
    /// if not specified. The value of `ip_address` can be an IP Address or a
    /// CIDR (if supported by the underlying extension plugin). A server
    /// connected to the port can send a packet with source address which
    /// matches one of the specified allowed address pairs.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    allowed_address_pairs: Option<Value>,

    /// The ID of the host where the port resides.
    ///
    #[serde(rename = "binding:host_id")]
    #[structable(optional, title = "binding:host_id")]
    binding_host_id: Option<String>,

    /// A dictionary that enables the application running on the specific host
    /// to pass and receive vif port information specific to the networking
    /// back-end. The networking API does not define a specific format of this
    /// field. If the update request is null this response field will be {}.
    ///
    #[serde(rename = "binding:profile")]
    #[structable(optional, pretty, title = "binding:profile")]
    binding_profile: Option<Value>,

    /// A dictionary which contains additional information on the port.
    /// Currently the following fields are defined: `port_filter` and
    /// `ovs_hybrid_plug`. `port_filter` is a boolean indicating the networking
    /// service provides port filtering features such as security group and/or
    /// anti MAC/IP spoofing. `ovs_hybrid_plug` is a boolean used to inform an
    /// API consumer like nova that the hybrid plugging strategy for OVS should
    /// be used.
    ///
    #[serde(rename = "binding:vif_details")]
    #[structable(optional, pretty, title = "binding:vif_details")]
    binding_vif_details: Option<Value>,

    /// The type of which mechanism is used for the port. An API consumer like
    /// nova can use this to determine an appropriate way to attach a device
    /// (for example an interface of a virtual server) to the port. Available
    /// values currently defined includes `ovs`, `bridge`, `macvtap`, `hw_veb`,
    /// `hostdev_physical`, `vhostuser`, `distributed` and `other`. There are
    /// also special values: `unbound` and `binding_failed`. `unbound` means
    /// the port is not bound to a networking back-end. `binding_failed` means
    /// an error that the port failed to be bound to a networking back-end.
    ///
    #[serde(rename = "binding:vif_type")]
    #[structable(optional, title = "binding:vif_type")]
    binding_vif_type: Option<String>,

    /// The type of vNIC which this port should be attached to. This is used to
    /// determine which mechanism driver(s) to be used to bind the port. The
    /// valid values are `normal`, `macvtap`, `direct`, `baremetal`,
    /// `direct-physical`, `virtio-forwarder`, `smart-nic` and
    /// `remote-managed`. What type of vNIC is actually available depends on
    /// deployments.
    ///
    #[serde(rename = "binding:vnic_type")]
    #[structable(optional, title = "binding:vnic_type")]
    binding_vnic_type: Option<String>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// Status of the underlying data plane of a port.
    ///
    #[serde()]
    #[structable(optional)]
    data_plane_status: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of the device that uses this port. For example, a server
    /// instance or a logical router.
    ///
    #[serde()]
    #[structable(optional)]
    device_id: Option<String>,

    /// The entity type that uses this port. For example, `compute:nova`
    /// (server instance), `network:dhcp` (DHCP agent) or
    /// `network:router_interface` (router interface).
    ///
    #[serde()]
    #[structable(optional)]
    device_owner: Option<String>,

    #[serde()]
    #[structable(optional)]
    device_profile: Option<String>,

    /// Data assigned to a port by the Networking internal DNS including the
    /// `hostname`, `ip_address` and `fqdn`.
    ///
    #[serde()]
    #[structable(optional)]
    dns_assignment: Option<String>,

    /// A valid DNS domain.
    ///
    #[serde()]
    #[structable(optional)]
    dns_domain: Option<String>,

    /// A valid DNS name.
    ///
    #[serde()]
    #[structable(optional)]
    dns_name: Option<String>,

    /// A set of zero or more extra DHCP option pairs. An option pair consists
    /// of an option value and name.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    extra_dhcp_opts: Option<Value>,

    /// The IP addresses for the port. If the port has multiple IP addresses,
    /// this field has multiple entries. Each entry consists of IP address
    /// (`ip_address`) and the subnet ID from which the IP address is assigned
    /// (`subnet_id`).
    ///
    #[serde()]
    #[structable(optional, pretty)]
    fixed_ips: Option<Value>,

    /// Admin-only. The following values control Open vSwitch’s Userspace Tx
    /// packet steering feature:
    ///
    /// - `{"openvswitch": {"other_config": {"tx-steering": "hash|thread"}}}`
    ///
    #[serde()]
    #[structable(optional, pretty)]
    hints: Option<Value>,

    /// The ID of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Indicates when ports use either `deferred`, `immediate` or no IP
    /// allocation (`none`).
    ///
    #[serde()]
    #[structable(optional)]
    ip_allocation: Option<String>,

    /// The MAC address of the port. If the port uses the `direct-physical`
    /// `vnic_type` then the value of this field is overwritten with the MAC
    /// address provided in the active binding:profile if any.
    ///
    #[serde()]
    #[structable(optional)]
    mac_address: Option<String>,

    /// Human-readable name of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The ID of the attached network.
    ///
    #[serde()]
    #[structable(optional)]
    network_id: Option<String>,

    /// The port NUMA affinity policy requested during the virtual machine
    /// scheduling. Values: `None`, `required`, `preferred` or `legacy`.
    ///
    #[serde()]
    #[structable(optional)]
    numa_affinity_policy: Option<String>,

    /// The port security status. A valid value is enabled (`true`) or disabled
    /// (`false`). If port security is enabled for the port, security group
    /// rules and anti-spoofing rules are applied to the traffic on the port.
    /// If disabled, no such rules are applied.
    ///
    #[serde()]
    #[structable(optional)]
    port_security_enabled: Option<BoolString>,

    /// The uplink status propagation of the port. Valid values are enabled
    /// (`true`) and disabled (`false`).
    ///
    #[serde()]
    #[structable(optional)]
    propagate_uplink_status: Option<BoolString>,

    /// The ID of the QoS policy of the network where this port is plugged.
    ///
    #[serde()]
    #[structable(optional)]
    qos_network_policy_id: Option<String>,

    /// The ID of the QoS policy associated with the port.
    ///
    #[serde()]
    #[structable(optional)]
    qos_policy_id: Option<String>,

    /// Expose Placement resources (i.e.: `minimum-bandwidth`) and traits
    /// (i.e.: `vnic-type`, `physnet`) requested by a port to Nova and
    /// Placement. A `resource_request` object contains `request_groups` and
    /// `same_subtree` keys. `request_groups` is a list of dicts, where each
    /// dict represents one group of resources and traits that needs to be
    /// fulfilled from a single resource provider. Every dict in the list must
    /// contain `id`, `required` and `resources` keys. The `id` field is a
    /// string which represents a unique UUID that is generated for each group
    /// by combining the `port_id` and UUIDs of the QoS rules contributing to
    /// the group via the UUID5 method. `required` key contains the traits
    /// (generated from the `vnic_type` and the `physnet`) required by the
    /// port, and a `resources` key contains a mapping of requested resource
    /// class name and requested amount from the QoS policy. `same_subtree` key
    /// contains a list of `id` values from every resource group.
    ///
    #[serde()]
    #[structable(optional)]
    resource_request: Option<String>,

    /// The revision number of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    revision_number: Option<i32>,

    /// The IDs of security groups applied to the port.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    security_groups: Option<Value>,

    /// The port status. Values are `ACTIVE`, `DOWN`, `BUILD` and `ERROR`.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The list of tags on the resource.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    tags: Option<Value>,

    /// The ID of the project.
    ///
    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl PortCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Port");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;
        let mut ep_builder = set::Request::builder();

        // Set path parameters
        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();
        ep_builder.id(resource_id.clone());
        // Set query parameters
        // Set body parameters
        // Set Request.port data
        let args = &self.port;
        let mut port_builder = set::PortBuilder::default();
        if let Some(val) = &args.name {
            port_builder.name(val);
        }

        if let Some(val) = &args.admin_state_up {
            port_builder.admin_state_up(*val);
        }

        if let Some(val) = &args.mac_address {
            port_builder.mac_address(val);
        }

        if let Some(val) = &args.fixed_ips {
            let fixed_ips_builder: Vec<set::FixedIps> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<set::FixedIps>(v.to_owned()))
                .collect::<Vec<set::FixedIps>>();
            port_builder.fixed_ips(fixed_ips_builder);
        }

        if let Some(val) = &args.device_id {
            port_builder.device_id(val);
        }

        if let Some(val) = &args.device_owner {
            port_builder.device_owner(val);
        }

        if let Some(val) = &args.allowed_address_pairs {
            let allowed_address_pairs_builder: Vec<set::AllowedAddressPairs> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<set::AllowedAddressPairs>(v.to_owned()))
                .collect::<Vec<set::AllowedAddressPairs>>();
            port_builder.allowed_address_pairs(allowed_address_pairs_builder);
        }

        if let Some(val) = &args.data_plane_status {
            let tmp = match val {
                DataPlaneStatus::Active => set::DataPlaneStatus::Active,
                DataPlaneStatus::Down => set::DataPlaneStatus::Down,
            };
            port_builder.data_plane_status(tmp);
        }

        if let Some(val) = &args.extra_dhcp_opts {
            use std::collections::BTreeMap;
            port_builder.extra_dhcp_opts(
                val.iter()
                    .map(|v| {
                        v.as_object()
                            .expect("Is a valid Json object")
                            .iter()
                            .map(|(k, v)| (k.clone().into(), v.clone().into()))
                            .collect::<BTreeMap<_, Value>>()
                    })
                    .collect::<Vec<_>>(),
            );
        }

        if let Some(val) = &args.hints {
            port_builder.hints(val.iter().cloned());
        }

        if let Some(val) = &args.numa_affinity_policy {
            let tmp = match val {
                NumaAffinityPolicy::Legacy => set::NumaAffinityPolicy::Legacy,
                NumaAffinityPolicy::Preferred => set::NumaAffinityPolicy::Preferred,
                NumaAffinityPolicy::Required => set::NumaAffinityPolicy::Required,
            };
            port_builder.numa_affinity_policy(tmp);
        }

        if let Some(val) = &args.binding_vnic_type {
            let tmp = match val {
                BindingVnicType::AcceleratorDirect => set::BindingVnicType::AcceleratorDirect,
                BindingVnicType::AcceleratorDirectPhysical => {
                    set::BindingVnicType::AcceleratorDirectPhysical
                }
                BindingVnicType::Baremetal => set::BindingVnicType::Baremetal,
                BindingVnicType::Direct => set::BindingVnicType::Direct,
                BindingVnicType::DirectPhysical => set::BindingVnicType::DirectPhysical,
                BindingVnicType::Macvtap => set::BindingVnicType::Macvtap,
                BindingVnicType::Normal => set::BindingVnicType::Normal,
                BindingVnicType::RemoteManaged => set::BindingVnicType::RemoteManaged,
                BindingVnicType::SmartNic => set::BindingVnicType::SmartNic,
                BindingVnicType::Vdpa => set::BindingVnicType::Vdpa,
                BindingVnicType::VirtioForwarder => set::BindingVnicType::VirtioForwarder,
            };
            port_builder.binding_vnic_type(tmp);
        }

        if let Some(val) = &args.binding_host_id {
            port_builder.binding_host_id(val);
        }

        if let Some(val) = &args.binding_profile {
            port_builder.binding_profile(val.iter().cloned());
        }

        if let Some(val) = &args.port_security_enabled {
            port_builder.port_security_enabled(*val);
        }

        if let Some(val) = &args.qos_policy_id {
            port_builder.qos_policy_id(Some(val.into()));
        }

        if let Some(val) = &args.dns_name {
            port_builder.dns_name(val);
        }

        if let Some(val) = &args.dns_domain {
            port_builder.dns_domain(val);
        }

        if let Some(val) = &args.description {
            port_builder.description(val);
        }

        if let Some(val) = &args.security_groups {
            port_builder.security_groups(val.iter().map(|v| v.into()).collect::<Vec<_>>());
        }

        ep_builder.port(port_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
