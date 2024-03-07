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

//! List Hypervisors command
//!
//! Wraps invoking of the `v2.1/os-hypervisors/detail` with `GET` method

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

use crate::common::IntString;
use openstack_sdk::api::compute::v2::hypervisor::list_detailed;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;
use std::fmt;
use structable_derive::StructTable;

/// Lists hypervisors details.
///
/// Policy defaults enable only users with the administrative role to perform
/// this operation. Cloud providers can change these permissions through the
/// `policy.json` file.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403)
///
#[derive(Args)]
#[command(about = "List Hypervisors Details")]
pub struct HypervisorsCommand {
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
#[derive(Args)]
struct QueryParameters {
    #[arg(long)]
    hypervisor_hostname_pattern: Option<String>,

    #[arg(long)]
    limit: Option<i32>,

    #[arg(long)]
    marker: Option<String>,

    #[arg(action=clap::ArgAction::Set, long)]
    with_servers: Option<bool>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Hypervisors response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// A dictionary that contains cpu information like `arch`, `model`,
    /// `vendor`, `features` and `topology`. The content of this field is
    /// hypervisor specific.
    ///
    /// Note
    ///
    /// Since version 2.28 `cpu_info` field is returned as a dictionary instead
    /// of string.
    ///
    /// **Available until version 2.87**
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    cpu_info: Option<Value>,

    /// The current_workload is the number of tasks the hypervisor is
    /// responsible for. This will be equal or greater than the number of
    /// active VMs on the system (it can be greater when VMs are being deleted
    /// and the hypervisor is still cleaning up).
    ///
    /// **Available until version 2.87**
    ///
    #[serde()]
    #[structable(optional, wide)]
    current_workload: Option<i32>,

    /// The actual free disk on this hypervisor(in GiB). If allocation ratios
    /// used for overcommit are configured, this may be negative. This is
    /// intentional as it provides insight into the amount by which the disk is
    /// overcommitted.
    ///
    /// **Available until version 2.87**
    ///
    #[serde()]
    #[structable(optional, wide)]
    disk_available_least: Option<i32>,

    /// The free disk remaining on this hypervisor(in GiB). This does not take
    /// allocation ratios used for overcommit into account so this value may be
    /// negative.
    ///
    /// **Available until version 2.87**
    ///
    #[serde()]
    #[structable(optional, wide)]
    free_disk_gb: Option<i32>,

    /// The free RAM in this hypervisor(in MiB). This does not take allocation
    /// ratios used for overcommit into account so this value may be negative.
    ///
    /// **Available until version 2.87**
    ///
    #[serde()]
    #[structable(optional, wide)]
    free_ram_mb: Option<i32>,

    /// The IP address of the hypervisor’s host.
    ///
    #[serde()]
    #[structable(optional, wide)]
    host_ip: Option<String>,

    /// The hypervisor host name provided by the Nova virt driver. For the
    /// Ironic driver, it is the Ironic node uuid.
    ///
    #[serde()]
    #[structable(optional, wide)]
    hypervisor_hostname: Option<String>,

    /// The hypervisor type.
    ///
    #[serde()]
    #[structable(optional, wide)]
    hypervisor_type: Option<String>,

    /// The hypervisor version.
    ///
    #[serde()]
    #[structable(optional, wide)]
    hypervisor_version: Option<i32>,

    /// The id of the hypervisor. From version 2.53 it is a string as UUID
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The disk in this hypervisor (in GiB). This does not take allocation
    /// ratios used for overcommit into account so there may be disparity
    /// between this and the used count.
    ///
    /// **Available until version 2.87**
    ///
    #[serde()]
    #[structable(optional, wide)]
    local_gb: Option<i32>,

    /// The disk used in this hypervisor (in GiB).
    ///
    /// **Available until version 2.87**
    ///
    #[serde()]
    #[structable(optional, wide)]
    local_gb_used: Option<i32>,

    /// The memory of this hypervisor (in MiB). This does not take allocation
    /// ratios used for overcommit into account so there may be disparity
    /// between this and the used count.
    ///
    /// **Available until version 2.87**
    ///
    #[serde()]
    #[structable(optional, wide)]
    memory_mb: Option<i32>,

    /// The memory used in this hypervisor (in MiB).
    ///
    /// **Available until version 2.87**
    ///
    #[serde()]
    #[structable(optional, wide)]
    memory_mb_used: Option<i32>,

    /// The number of running VMs on this hypervisor.
    ///
    /// **Available until version 2.87**
    ///
    #[serde()]
    #[structable(optional, wide)]
    running_vms: Option<i32>,

    /// A list of `server` objects. This field has become mandatory in
    /// microversion 2.75. If no servers is on hypervisor then empty list is
    /// returned.
    ///
    /// **New in version 2.53**
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    servers: Option<Value>,

    /// The hypervisor service object.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    service: Option<Value>,

    /// The state of the hypervisor. One of `up` or `down`.
    ///
    #[serde()]
    #[structable(optional)]
    state: Option<String>,

    /// The status of the hypervisor. One of `enabled` or `disabled`.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The total uptime of the hypervisor and information about average load.
    /// Only reported for active hosts where the virt driver supports this
    /// feature.
    ///
    /// **New in version 2.88**
    ///
    #[serde()]
    #[structable(optional, wide)]
    uptime: Option<String>,

    /// The number of vCPU in this hypervisor. This does not take allocation
    /// ratios used for overcommit into account so there may be disparity
    /// between this and the used count.
    ///
    /// **Available until version 2.87**
    ///
    #[serde()]
    #[structable(optional, wide)]
    vcpus: Option<i32>,

    /// The number of vCPU used in this hypervisor.
    ///
    /// **Available until version 2.87**
    ///
    #[serde()]
    #[structable(optional, wide)]
    vcpus_used: Option<i32>,
}
/// `struct` response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseService {
    disabled_reason: Option<String>,
    host: Option<String>,
    id: Option<IntString>,
}

impl fmt::Display for ResponseService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "disabled_reason={}",
                self.disabled_reason
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "host={}",
                self.host
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}

impl HypervisorsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Hypervisors");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list_detailed::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.query.marker {
            ep_builder.marker(val);
        }
        if let Some(val) = &self.query.hypervisor_hostname_pattern {
            ep_builder.hypervisor_hostname_pattern(val);
        }
        if let Some(val) = &self.query.with_servers {
            ep_builder.with_servers(*val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.max_items))
            .query_async(client)
            .await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
