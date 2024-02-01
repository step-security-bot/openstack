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

use crate::common::parse_json;
use clap::ValueEnum;
use openstack_sdk::api::network::v2::subnet::create;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::fmt;
use structable_derive::StructTable;

/// Creates a subnet on a network.
///
/// OpenStack Networking does not try to derive the correct IP version
/// from the CIDR. If you do not specify the `gateway\_ip` attribute,
/// OpenStack Networking allocates an address from the CIDR for the
/// gateway for the subnet.
///
/// To specify a subnet without a gateway, set the `gateway\_ip`
/// attribute to `null` in the request body. If you do not specify
/// the `allocation\_pools` attribute, OpenStack Networking
/// automatically allocates pools for covering all IP addresses in the
/// CIDR, excluding the address reserved for the subnet gateway.
/// Otherwise, you can explicitly specify allocation pools as shown in
/// the following example.
///
/// When you specify both the `allocation\_pools` and `gateway\_ip`
/// attributes, you must ensure that the gateway IP does not overlap
/// with the allocation pools; otherwise, the call returns the
/// `Conflict (409)` response code.
///
/// A subnet can have one or more name servers and host routes. Hosts
/// in this subnet use the name servers. Devices with IP addresses from
/// this subnet, not including the local subnet route, use the host
/// routes.
///
/// Specify the `ipv6\_ra\_mode` and `ipv6\_address\_mode` attributes
/// to create subnets that support IPv6 configurations, such as
/// stateless address autoconfiguration (SLAAC), DHCPv6 stateful, and
/// DHCPv6 stateless configurations.
///
/// A subnet can optionally be associated with a network segment when
/// it is created by specifying the `segment\_id` of a valid segment
/// on the specified network. A network with subnets associated in this
/// way is called a routed network. On any given network, all of the
/// subnets must be associated with segments or none of them can be.
/// Neutron enforces this invariant. Currently, routed networks are
/// only supported for provider networks.
///
/// Normal response codes: 201
///
/// Error response codes: 400, 401, 403, 404, 409
#[derive(Args, Clone, Debug)]
#[command(about = "Create subnet")]
pub struct SubnetArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    subnet: Subnet,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Ipv6RaMode {
    Dhcpv6Stateful,
    Dhcpv6Stateless,
    Slaac,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Ipv6AddressMode {
    Dhcpv6Stateful,
    Dhcpv6Stateless,
    Slaac,
}

/// Subnet Body data
#[derive(Args, Debug, Clone)]
struct Subnet {
    /// Human-readable name of the resource. Default is an empty string.
    #[arg(long)]
    name: Option<String>,

    /// The IP protocol version. Value is `4` or `6`.
    #[arg(long)]
    ip_version: i32,

    /// The ID of the network to which the subnet belongs.
    #[arg(long)]
    network_id: String,

    /// The ID of the subnet pool associated with the subnet.
    #[arg(long)]
    subnetpool_id: Option<String>,

    /// The prefix length to use for subnet allocation from a subnet pool.
    /// If not specified, the `default\_prefixlen` value of the subnet pool
    /// will be used.
    #[arg(long)]
    prefixlen: Option<i32>,

    /// The CIDR of the subnet.
    #[arg(long)]
    cidr: Option<String>,

    /// Gateway IP of this subnet. If the value is `null` that implies no
    /// gateway is associated with the subnet. If the gateway\_ip is not
    /// specified, OpenStack Networking allocates an address from the CIDR
    /// for the gateway for the subnet by default.
    #[arg(long)]
    gateway_ip: Option<String>,

    /// Allocation pools with `start` and `end` IP addresses
    /// for this subnet. If allocation\_pools are not specified, OpenStack
    /// Networking automatically allocates pools for covering all IP addresses
    /// in the CIDR, excluding the address reserved for the subnet gateway by
    /// default.
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    allocation_pools: Option<Vec<Value>>,

    /// List of dns name servers associated with the subnet. Default is an
    /// empty list.
    #[arg(action=clap::ArgAction::Append, long)]
    dns_nameservers: Option<Vec<String>>,

    /// Additional routes for the subnet. A list of dictionaries with
    /// `destination` and `nexthop` parameters. Default value is
    /// an empty list.
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    host_routes: Option<Vec<Value>>,

    /// The ID of the project that owns the resource.
    /// Only administrative and users with advsvc role can specify
    /// a project ID other than their own.
    /// You cannot change this value through authorization policies.
    #[arg(long)]
    tenant_id: Option<String>,

    /// Indicates whether dhcp is enabled or disabled
    /// for the subnet. Default is `true`.
    #[arg(action=clap::ArgAction::Set, long)]
    enable_dhcp: Option<bool>,

    /// The IPv6 router advertisement specifies whether the networking service
    /// should transmit ICMPv6 packets, for a subnet. Value is `slaac`,
    /// `dhcpv6-stateful`, `dhcpv6-stateless`.
    #[arg(long)]
    ipv6_ra_mode: Option<Ipv6RaMode>,

    /// The IPv6 address modes specifies mechanisms for assigning IP addresses.
    /// Value is `slaac`, `dhcpv6-stateful`, `dhcpv6-stateless`.
    #[arg(long)]
    ipv6_address_mode: Option<Ipv6AddressMode>,

    /// The service types associated with the subnet.
    #[arg(action=clap::ArgAction::Append, long)]
    service_types: Option<Vec<String>>,

    /// Whether to allocate this subnet from the default subnet pool.
    #[arg(action=clap::ArgAction::Set, long)]
    use_default_subnetpool: Option<bool>,

    /// Whether to publish DNS records for IPs from this subnet. Default
    /// is `false`.
    #[arg(action=clap::ArgAction::Set, long)]
    dns_publish_fixed_ip: Option<bool>,

    /// A human-readable description for the resource.
    /// Default is an empty string.
    #[arg(long)]
    description: Option<String>,

    /// The ID of a network segment the subnet is associated with.
    /// It is available when `segment` extension is enabled.
    #[arg(long)]
    segment_id: Option<String>,
}

/// Subnet create command
pub struct SubnetCmd {
    pub args: SubnetArgs,
}
/// Subnet response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The ID of the subnet.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Human-readable name of the resource.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The IP protocol version. Value is `4` or `6`.
    #[serde()]
    #[structable(optional)]
    ip_version: Option<i32>,

    /// The ID of the network to which the subnet belongs.
    #[serde()]
    #[structable(optional)]
    network_id: Option<String>,

    /// The ID of the subnet pool associated with the subnet.
    #[serde()]
    #[structable(optional)]
    subnetpool_id: Option<String>,

    /// The CIDR of the subnet.
    #[serde()]
    #[structable(optional)]
    cidr: Option<String>,

    /// Gateway IP of this subnet. If the value is `null` that implies no
    /// gateway is associated with the subnet.
    #[serde()]
    #[structable(optional)]
    gateway_ip: Option<String>,

    /// Allocation pools with `start` and `end` IP addresses
    /// for this subnet.
    #[serde()]
    #[structable(optional)]
    allocation_pools: Option<VecResponseAllocationPools>,

    /// List of dns name servers associated with the subnet.
    #[serde()]
    #[structable(optional)]
    dns_nameservers: Option<VecString>,

    /// Additional routes for the subnet. A list of dictionaries with
    /// `destination` and `nexthop` parameters.
    #[serde()]
    #[structable(optional)]
    host_routes: Option<VecResponseHostRoutes>,

    /// The ID of the project.
    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,

    /// Indicates whether dhcp is enabled or disabled
    /// for the subnet.
    #[serde()]
    #[structable(optional)]
    enable_dhcp: Option<bool>,

    /// The IPv6 router advertisement specifies whether the networking service
    /// should transmit ICMPv6 packets, for a subnet. Value is `slaac`,
    /// `dhcpv6-stateful`, `dhcpv6-stateless` or `null`.
    #[serde()]
    #[structable(optional)]
    ipv6_ra_mode: Option<String>,

    /// The IPv6 address modes specifies mechanisms for assigning IP addresses.
    /// Value is `slaac`, `dhcpv6-stateful`, `dhcpv6-stateless` or `null`.
    #[serde()]
    #[structable(optional)]
    ipv6_address_mode: Option<String>,

    /// The revision number of the resource.
    #[serde()]
    #[structable(optional)]
    revision_number: Option<i32>,

    /// The service types associated with the subnet.
    #[serde()]
    #[structable(optional)]
    service_types: Option<VecString>,

    /// The list of tags on the resource.
    #[serde()]
    #[structable(optional)]
    tags: Option<VecString>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// Whether to publish DNS records for IPs from this subnet.
    #[serde()]
    #[structable(optional)]
    dns_publish_fixed_ip: Option<bool>,

    /// A human-readable description for the resource.
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of a network segment the subnet is associated with.
    /// It is available when `segment` extension is enabled.
    #[serde()]
    #[structable(optional)]
    segment_id: Option<String>,
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseAllocationPools {
    start: Option<String>,
    end: Option<String>,
}

impl fmt::Display for ResponseAllocationPools {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "start={}",
                self.start
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "end={}",
                self.end
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseAllocationPools(Vec<ResponseAllocationPools>);
impl fmt::Display for VecResponseAllocationPools {
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
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseHostRoutes {
    destination: Option<String>,
    nexthop: Option<String>,
}

impl fmt::Display for ResponseHostRoutes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "destination={}",
                self.destination
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "nexthop={}",
                self.nexthop
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseHostRoutes(Vec<ResponseHostRoutes>);
impl fmt::Display for VecResponseHostRoutes {
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
impl OSCCommand for SubnetCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Subnet with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.subnet data
        let args = &self.args.subnet;
        let mut subnet_builder = create::SubnetBuilder::default();
        if let Some(val) = &args.name {
            subnet_builder.name(val.clone());
        }

        subnet_builder.ip_version(args.ip_version);

        subnet_builder.network_id(args.network_id.clone());

        if let Some(val) = &args.subnetpool_id {
            subnet_builder.subnetpool_id(Some(val.into()));
        }

        if let Some(val) = &args.prefixlen {
            subnet_builder.prefixlen(*val);
        }

        if let Some(val) = &args.cidr {
            subnet_builder.cidr(Some(val.into()));
        }

        if let Some(val) = &args.gateway_ip {
            subnet_builder.gateway_ip(val.clone());
        }

        if let Some(val) = &args.allocation_pools {
            let allocation_pools_builder: Vec<create::AllocationPools> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<create::AllocationPools>(v.clone()))
                .collect::<Vec<create::AllocationPools>>();
            subnet_builder.allocation_pools(allocation_pools_builder);
        }

        if let Some(val) = &args.dns_nameservers {
            subnet_builder.dns_nameservers(val.iter().map(|v| v.into()).collect::<Vec<_>>());
        }

        if let Some(val) = &args.host_routes {
            let host_routes_builder: Vec<create::HostRoutes> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<create::HostRoutes>(v.clone()))
                .collect::<Vec<create::HostRoutes>>();
            subnet_builder.host_routes(host_routes_builder);
        }

        if let Some(val) = &args.tenant_id {
            subnet_builder.tenant_id(val.clone());
        }

        if let Some(val) = &args.enable_dhcp {
            subnet_builder.enable_dhcp(*val);
        }

        if let Some(val) = &args.ipv6_ra_mode {
            let tmp = match val {
                Ipv6RaMode::Dhcpv6Stateful => create::Ipv6RaMode::Dhcpv6Stateful,
                Ipv6RaMode::Dhcpv6Stateless => create::Ipv6RaMode::Dhcpv6Stateless,
                Ipv6RaMode::Slaac => create::Ipv6RaMode::Slaac,
            };
            subnet_builder.ipv6_ra_mode(tmp);
        }

        if let Some(val) = &args.ipv6_address_mode {
            let tmp = match val {
                Ipv6AddressMode::Dhcpv6Stateful => create::Ipv6AddressMode::Dhcpv6Stateful,
                Ipv6AddressMode::Dhcpv6Stateless => create::Ipv6AddressMode::Dhcpv6Stateless,
                Ipv6AddressMode::Slaac => create::Ipv6AddressMode::Slaac,
            };
            subnet_builder.ipv6_address_mode(tmp);
        }

        if let Some(val) = &args.service_types {
            subnet_builder.service_types(val.iter().map(|v| v.into()).collect::<Vec<_>>());
        }

        if let Some(val) = &args.use_default_subnetpool {
            subnet_builder.use_default_subnetpool(*val);
        }

        if let Some(val) = &args.dns_publish_fixed_ip {
            subnet_builder.dns_publish_fixed_ip(*val);
        }

        if let Some(val) = &args.description {
            subnet_builder.description(val.clone());
        }

        if let Some(val) = &args.segment_id {
            subnet_builder.segment_id(Some(val.into()));
        }

        ep_builder.subnet(subnet_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
