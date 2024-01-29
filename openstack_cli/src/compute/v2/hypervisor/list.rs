//! Lists hypervisors details.
//!
//! Policy defaults enable only users with the administrative role to perform
//! this operation. Cloud providers can change these permissions through
//! the `policy.json` file.
//!
//! Normal response codes: 200
//!
//! Error response codes: badRequest(400), unauthorized(401), forbidden(403)
//!
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
use structable_derive::StructTable;

use openstack_sdk::AsyncOpenStack;

use crate::common::IntString;
use openstack_sdk::api::compute::v2::hypervisor::list_detailed;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct HypervisorsArgs {
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
    limit: Option<i32>,

    #[arg(long)]
    marker: Option<String>,

    #[arg(long)]
    hypervisor_hostname_pattern: Option<String>,

    #[arg(long)]
    with_servers: Option<bool>,
}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {}

/// Hypervisors list command
pub struct HypervisorsCmd {
    pub args: HypervisorsArgs,
}
/// Hypervisors response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// A dictionary that contains cpu information like `arch`, `model`,
    /// `vendor`, `features` and `topology`. The content of this field is
    /// hypervisor specific.
    ///
    ///
    ///
    /// Note
    ///
    ///
    /// Since version 2.28 `cpu\_info` field is returned as a dictionary
    /// instead of string.
    ///
    ///
    ///
    /// **Available until version 2.87**
    #[serde()]
    #[structable(optional, wide)]
    cpu_info: Option<HashMapStringValue>,

    /// The current\_workload is the number of tasks the hypervisor is
    /// responsible
    /// for. This will be equal or greater than the number of active VMs on the
    /// system (it can be greater when VMs are being deleted and the hypervisor
    /// is
    /// still cleaning up).
    ///
    ///
    /// **Available until version 2.87**
    #[serde()]
    #[structable(optional, wide)]
    current_workload: Option<i32>,

    /// The actual free disk on this hypervisor(in GiB). If allocation ratios
    /// used
    /// for overcommit are configured, this may be negative. This is
    /// intentional as
    /// it provides insight into the amount by which the disk is overcommitted.
    ///
    ///
    /// **Available until version 2.87**
    #[serde()]
    #[structable(optional, wide)]
    disk_available_least: Option<i32>,

    /// The IP address of the hypervisor’s host.
    #[serde()]
    #[structable(optional, wide)]
    host_ip: Option<String>,

    /// The free disk remaining on this hypervisor(in GiB). This does not take
    /// allocation ratios used for overcommit into account so this value may be
    /// negative.
    ///
    ///
    /// **Available until version 2.87**
    #[serde()]
    #[structable(optional, wide)]
    free_disk_gb: Option<i32>,

    /// The free RAM in this hypervisor(in MiB). This does not take allocation
    /// ratios used for overcommit into account so this value may be negative.
    ///
    ///
    /// **Available until version 2.87**
    #[serde()]
    #[structable(optional, wide)]
    free_ram_mb: Option<i32>,

    /// The hypervisor host name provided by the Nova virt driver. For the
    /// Ironic
    /// driver, it is the Ironic node uuid.
    #[serde()]
    #[structable(optional, wide)]
    hypervisor_hostname: Option<String>,

    /// The hypervisor type.
    #[serde()]
    #[structable(optional, wide)]
    hypervisor_type: Option<String>,

    /// The hypervisor version.
    #[serde()]
    #[structable(optional, wide)]
    hypervisor_version: Option<i32>,

    /// The disk in this hypervisor (in GiB). This does not take allocation
    /// ratios used for overcommit into account so there may be disparity
    /// between
    /// this and the used count.
    ///
    ///
    /// **Available until version 2.87**
    #[serde()]
    #[structable(optional, wide)]
    local_gb: Option<i32>,

    /// The disk used in this hypervisor (in GiB).
    ///
    ///
    /// **Available until version 2.87**
    #[serde()]
    #[structable(optional, wide)]
    local_gb_used: Option<i32>,

    /// The memory of this hypervisor (in MiB). This does not take allocation
    /// ratios used for overcommit into account so there may be disparity
    /// between
    /// this and the used count.
    ///
    ///
    /// **Available until version 2.87**
    #[serde()]
    #[structable(optional, wide)]
    memory_mb: Option<i32>,

    /// The memory used in this hypervisor (in MiB).
    ///
    ///
    /// **Available until version 2.87**
    #[serde()]
    #[structable(optional, wide)]
    memory_mb_used: Option<i32>,

    /// The number of running VMs on this hypervisor.
    ///
    ///
    /// **Available until version 2.87**
    #[serde()]
    #[structable(optional, wide)]
    running_vms: Option<i32>,

    /// The hypervisor service object.
    #[serde()]
    #[structable(optional, wide)]
    service: Option<ResponseService>,

    /// The total uptime of the hypervisor and information about average load.
    /// Only
    /// reported for active hosts where the virt driver supports this feature.
    ///
    ///
    /// **New in version 2.88**
    #[serde()]
    #[structable(optional, wide)]
    uptime: Option<String>,

    /// The number of vCPU in this hypervisor. This does not take allocation
    /// ratios used for overcommit into account so there may be disparity
    /// between
    /// this and the used count.
    ///
    ///
    /// **Available until version 2.87**
    #[serde()]
    #[structable(optional, wide)]
    vcpus: Option<i32>,

    /// The number of vCPU used in this hypervisor.
    ///
    ///
    /// **Available until version 2.87**
    #[serde()]
    #[structable(optional, wide)]
    vcpus_used: Option<i32>,

    /// The id of the hypervisor. From version 2.53 it is a string as UUID
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The state of the hypervisor. One of `up` or `down`.
    #[serde()]
    #[structable(optional)]
    state: Option<String>,

    /// The status of the hypervisor. One of `enabled` or `disabled`.
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// A list of `server` objects.
    /// This field has become mandatory in microversion 2.75. If no servers is
    /// on hypervisor
    /// then empty list is returned.
    ///
    ///
    /// **New in version 2.53**
    #[serde()]
    #[structable(optional, wide)]
    servers: Option<VecResponseServers>,
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
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseService {
    host: Option<String>,
    id: Option<IntString>,
    disabled_reason: Option<String>,
}

impl fmt::Display for ResponseService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
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
            format!(
                "disabled_reason={}",
                self.disabled_reason
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseServers {
    uuid: Option<String>,
    name: Option<String>,
}

impl fmt::Display for ResponseServers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "uuid={}",
                self.uuid
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "name={}",
                self.name
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseServers(Vec<ResponseServers>);
impl fmt::Display for VecResponseServers {
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
impl OSCCommand for HypervisorsCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Hypervisors with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = list_detailed::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.args.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.args.query.marker {
            ep_builder.marker(val.clone());
        }
        if let Some(val) = &self.args.query.hypervisor_hostname_pattern {
            ep_builder.hypervisor_hostname_pattern(val.clone());
        }
        if let Some(val) = &self.args.query.with_servers {
            ep_builder.with_servers(*val);
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
