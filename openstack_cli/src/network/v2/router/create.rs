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

//! Create Router command
//!
//! Wraps invoking of the `v2.0/routers` with `POST` method

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
use crate::common::BoolString;
use openstack_sdk::api::network::v2::router::create;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::fmt;
use structable_derive::StructTable;

/// Creates a logical router.
///
/// This operation creates a logical router. The logical router does not have
/// any internal interface and it is not associated with any subnet. You can
/// optionally specify an external gateway for a router at create time. The
/// external gateway for the router must be plugged into an external network.
/// An external network has its `router:external` extended field set to `true`.
/// To specify an external gateway, the ID of the external network must be
/// passed in the `network_id` parameter of the `external_gateway_info`
/// attribute in the request body.
///
/// Normal response codes: 201
///
/// Error response codes: 400, 401
///
#[derive(Args)]
#[command(about = "Create router")]
pub struct RouterCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `router` object.
    ///
    #[command(flatten)]
    router: Router,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// ExternalGatewayInfo Body data
#[derive(Args)]
#[group(required = false, multiple = true)]
struct ExternalGatewayInfo {
    #[arg(action=clap::ArgAction::Set, long)]
    enable_snat: Option<bool>,

    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    external_fixed_ips: Option<Vec<Value>>,

    #[arg(long, required = false)]
    network_id: String,
}

/// Router Body data
#[derive(Args)]
struct Router {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`). Default is `true`.
    ///
    #[arg(action=clap::ArgAction::Set, long)]
    admin_state_up: Option<bool>,

    /// The availability zone candidates for the router. It is available when
    /// `router_availability_zone` extension is enabled.
    ///
    #[arg(action=clap::ArgAction::Append, long)]
    availability_zone_hints: Option<Vec<String>>,

    /// A human-readable description for the resource. Default is an empty
    /// string.
    ///
    #[arg(long)]
    description: Option<String>,

    /// `true` indicates a distributed router. It is available when `dvr`
    /// extension is enabled.
    ///
    #[arg(action=clap::ArgAction::Set, long)]
    distributed: Option<Option<bool>>,

    /// Enable NDP proxy attribute. Default is `false`, To persist this
    /// attribute value, set the `enable_ndp_proxy_by_default` option in the
    /// `neutron.conf` file. It is available when `router-extend-ndp-proxy`
    /// extension is enabled.
    ///
    #[arg(action=clap::ArgAction::Set, long)]
    enable_ndp_proxy: Option<Option<bool>>,

    /// The external gateway information of the router. If the router has an
    /// external gateway, this would be a dict with `network_id`,
    /// `enable_snat`, `external_fixed_ips` and `qos_policy_id`. Otherwise,
    /// this would be `null`.
    ///
    #[command(flatten)]
    external_gateway_info: Option<ExternalGatewayInfo>,

    /// The ID of the flavor associated with the router.
    ///
    #[arg(long)]
    flavor_id: Option<String>,

    /// `true` indicates a highly-available router. It is available when
    /// `l3-ha` extension is enabled.
    ///
    #[arg(action=clap::ArgAction::Set, long)]
    ha: Option<Option<bool>>,

    /// Human-readable name of the resource. Default is an empty string.
    ///
    #[arg(long)]
    name: Option<String>,

    /// The ID of the project that owns the resource. Only administrative and
    /// users with advsvc role can specify a project ID other than their own.
    /// You cannot change this value through authorization policies.
    ///
    #[arg(long)]
    tenant_id: Option<String>,
}

/// Router response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde()]
    #[structable(optional)]
    admin_state_up: Option<BoolString>,

    /// The availability zone candidates for the router. It is available when
    /// `router_availability_zone` extension is enabled.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    availability_zone_hints: Option<Value>,

    /// The availability zone(s) for the router. It is available when
    /// `router_availability_zone` extension is enabled.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    availability_zones: Option<Value>,

    /// The associated conntrack helper resources for the roter. If the router
    /// has multiple conntrack helper resources, this field has multiple
    /// entries. Each entry consists of netfilter conntrack helper (`helper`),
    /// the network protocol (`protocol`), the network port (`port`).
    ///
    #[serde()]
    #[structable(optional)]
    conntrack_helpers: Option<String>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// `true` indicates a distributed router. It is available when `dvr`
    /// extension is enabled.
    ///
    #[serde()]
    #[structable(optional)]
    distributed: Option<BoolString>,

    /// Enable NDP proxy attribute. `true` means NDP proxy is enabled for the
    /// router, the IPv6 address of internal subnets attached to the router can
    /// be published to external by create `ndp_proxy`. `false` means NDP proxy
    /// is disabled, the IPv6 address of internal subnets attached to the
    /// router can not be published to external by `ndp_proxy`. It is available
    /// when `router-extend-ndp-proxy` extension is enabled.
    ///
    #[serde()]
    #[structable(optional)]
    enable_ndp_proxy: Option<BoolString>,

    /// The external gateway information of the router. If the router has an
    /// external gateway, this would be a dict with `network_id`,
    /// `enable_snat`, `external_fixed_ips`, `qos_policy_id`,
    /// `enable_default_route_ecmp` and `enable_default_route_bfd`. Otherwise,
    /// this would be `null`.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    external_gateway_info: Option<Value>,

    /// The ID of the flavor associated with the router.
    ///
    #[serde()]
    #[structable(optional)]
    flavor_id: Option<String>,

    /// `true` indicates a highly-available router. It is available when
    /// `l3-ha` extension is enabled.
    ///
    #[serde()]
    #[structable(optional)]
    ha: Option<BoolString>,

    /// The ID of the router.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Human-readable name of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The revision number of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    revision_number: Option<i32>,

    /// The extra routes configuration for L3 router. A list of dictionaries
    /// with `destination` and `nexthop` parameters. It is available when
    /// `extraroute` extension is enabled.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    routes: Option<Value>,

    /// The router status.
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
/// `struct` response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseExternalGatewayInfo {
    enable_snat: Option<bool>,
    external_fixed_ips: Option<Value>,
    network_id: String,
}

impl fmt::Display for ResponseExternalGatewayInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "enable_snat={}",
                self.enable_snat
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "external_fixed_ips={}",
                self.external_fixed_ips
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!("network_id={}", self.network_id),
        ]);
        write!(f, "{}", data.join(";"))
    }
}

impl RouterCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Router");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.router data
        let args = &self.router;
        let mut router_builder = create::RouterBuilder::default();
        if let Some(val) = &args.name {
            router_builder.name(val);
        }

        if let Some(val) = &args.admin_state_up {
            router_builder.admin_state_up(*val);
        }

        if let Some(val) = &args.tenant_id {
            router_builder.tenant_id(val);
        }

        if let Some(val) = &args.external_gateway_info {
            let mut external_gateway_info_builder = create::ExternalGatewayInfoBuilder::default();

            external_gateway_info_builder.network_id(&val.network_id);
            if let Some(val) = &val.enable_snat {
                external_gateway_info_builder.enable_snat(*val);
            }
            if let Some(val) = &val.external_fixed_ips {
                let external_fixed_ips_builder: Vec<create::ExternalFixedIps> = val
                    .iter()
                    .flat_map(|v| serde_json::from_value::<create::ExternalFixedIps>(v.to_owned()))
                    .collect::<Vec<create::ExternalFixedIps>>();
                external_gateway_info_builder.external_fixed_ips(external_fixed_ips_builder);
            }
            router_builder.external_gateway_info(
                external_gateway_info_builder
                    .build()
                    .expect("A valid object"),
            );
        }

        if let Some(val) = &args.ha {
            router_builder.ha(*val);
        }

        if let Some(val) = &args.enable_ndp_proxy {
            router_builder.enable_ndp_proxy(*val);
        }

        if let Some(val) = &args.flavor_id {
            router_builder.flavor_id(val);
        }

        if let Some(val) = &args.availability_zone_hints {
            router_builder
                .availability_zone_hints(val.iter().map(|v| v.into()).collect::<Vec<_>>());
        }

        if let Some(val) = &args.distributed {
            router_builder.distributed(*val);
        }

        if let Some(val) = &args.description {
            router_builder.description(val);
        }

        ep_builder.router(router_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
