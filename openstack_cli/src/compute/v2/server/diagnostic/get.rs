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

use openstack_sdk::api::compute::v2::server::diagnostic::get;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use structable_derive::StructTable;

/// Shows basic usage data for a server.
///
/// Policy defaults enable only users with the administrative role. Cloud
/// providers can change these permissions through the `policy.json`
/// file.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403), notfound(404),
/// conflict(409), notimplemented(501)
#[derive(Args, Clone, Debug)]
#[command(about = "Show Server Diagnostics")]
pub struct DiagnosticArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// server_id parameter for /v2.1/servers/{server_id}/topology API
    #[arg(value_name = "SERVER_ID", id = "path_param_server_id")]
    server_id: String,
}

/// Diagnostic get command
pub struct DiagnosticCmd {
    pub args: DiagnosticArgs,
}
/// Diagnostic response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The list of dictionaries with detailed information about VM CPUs.
    /// Following fields are presented in each dictionary:
    ///
    ///
    /// * `id` - the ID of CPU (Integer)
    /// * `time` - CPU Time in nano seconds (Integer)
    /// * `utilisation` - CPU utilisation in percents (Integer)
    ///
    ///
    /// **New in version 2.48**
    #[serde()]
    #[structable(optional)]
    cpu_details: Option<VecHashMapStringValue>,

    /// The list of dictionaries with detailed information about VM disks.
    /// Following fields are presented in each dictionary:
    ///
    ///
    /// * `read\_bytes` - Disk reads in bytes (Integer)
    /// * `read\_requests` - Read requests (Integer)
    /// * `write\_bytes` - Disk writes in bytes (Integer)
    /// * `write\_requests` - Write requests (Integer)
    /// * `errors\_count` - Disk errors (Integer)
    ///
    ///
    /// **New in version 2.48**
    #[serde()]
    #[structable(optional)]
    disk_details: Option<VecHashMapStringValue>,

    /// The driver on which the VM is running. Possible values are:
    ///
    ///
    /// * `libvirt`
    /// * `xenapi`
    /// * `hyperv`
    /// * `vmwareapi`
    /// * `ironic`
    ///
    ///
    /// **New in version 2.48**
    #[serde()]
    #[structable(optional)]
    driver: Option<String>,

    /// Indicates whether or not a config drive was used for this server.
    ///
    ///
    /// **New in version 2.48**
    #[serde()]
    #[structable(optional)]
    config_drive: Option<bool>,

    /// The hypervisor on which the VM is running. Examples for libvirt driver
    /// may be: `qemu`, `kvm` or `xen`.
    ///
    ///
    /// **New in version 2.48**
    #[serde()]
    #[structable(optional)]
    hypervisor: Option<String>,

    /// The hypervisor OS.
    ///
    ///
    /// **New in version 2.48**
    #[serde()]
    #[structable(optional)]
    hypervisor_os: Option<String>,

    /// Id of the resource
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The dictionary with information about VM memory usage.
    /// Following fields are presented in the dictionary:
    ///
    ///
    /// * `maximum` - Amount of memory provisioned for the VM in MiB (Integer)
    /// * `used` - Amount of memory that is currently used by the guest
    /// operating
    /// system and its applications in MiB (Integer)
    ///
    ///
    /// **New in version 2.48**
    #[serde()]
    #[structable(optional)]
    memory_details: Option<VecHashMapStringValue>,

    /// Name
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The list of dictionaries with detailed information about VM NICs.
    /// Following fields are presented in each dictionary:
    ///
    ///
    /// * `mac\_address` - Mac address of the interface (String)
    /// * `rx\_octets` - Received octets (Integer)
    /// * `rx\_errors` - Received errors (Integer)
    /// * `rx\_drop` - Received packets dropped (Integer)
    /// * `rx\_packets` - Received packets (Integer)
    /// * `rx\_rate` - Receive rate in bytes (Integer)
    /// * `tx\_octets` - Transmitted Octets (Integer)
    /// * `tx\_errors` - Transmit errors (Integer)
    /// * `tx\_drop` - Transmit dropped packets (Integer)
    /// * `tx\_packets` - Transmit packets (Integer)
    /// * `tx\_rate` - Transmit rate in bytes (Integer)
    ///
    ///
    /// **New in version 2.48**
    #[serde()]
    #[structable(optional)]
    nic_details: Option<VecResponseNicDetails>,

    /// The number of vCPUs.
    ///
    ///
    /// **New in version 2.48**
    #[serde()]
    #[structable(optional)]
    num_cpus: Option<i32>,

    /// The number of disks.
    ///
    ///
    /// **New in version 2.48**
    #[serde()]
    #[structable(optional)]
    num_disks: Option<i32>,

    /// The number of vNICs.
    ///
    ///
    /// **New in version 2.48**
    #[serde()]
    #[structable(optional)]
    num_nics: Option<i32>,

    /// A string enum denoting the current state of the VM. Possible values
    /// are:
    ///
    ///
    /// * `pending`
    /// * `running`
    /// * `paused`
    /// * `shutdown`
    /// * `crashed`
    /// * `suspended`
    ///
    ///
    /// **New in version 2.48**
    #[serde()]
    #[structable(optional)]
    state: Option<String>,

    /// The amount of time in seconds that the VM has been running.
    ///
    ///
    /// **New in version 2.48**
    #[serde()]
    #[structable(optional)]
    uptime: Option<i32>,
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
struct ResponseNicDetails {
    mac_address: Option<String>,
    rx_octets: Option<i32>,
    rx_errors: Option<i32>,
    rx_drop: Option<i32>,
    rx_packets: Option<i32>,
    rx_rate: Option<i32>,
    tx_octets: Option<i32>,
    tx_errors: Option<i32>,
    tx_drop: Option<i32>,
    tx_packets: Option<i32>,
    tx_rate: Option<i32>,
}

impl fmt::Display for ResponseNicDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "mac_address={}",
                self.mac_address
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "rx_octets={}",
                self.rx_octets
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "rx_errors={}",
                self.rx_errors
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "rx_drop={}",
                self.rx_drop
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "rx_packets={}",
                self.rx_packets
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "rx_rate={}",
                self.rx_rate
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "tx_octets={}",
                self.tx_octets
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "tx_errors={}",
                self.tx_errors
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "tx_drop={}",
                self.tx_drop
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "tx_packets={}",
                self.tx_packets
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "tx_rate={}",
                self.tx_rate
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseNicDetails(Vec<ResponseNicDetails>);
impl fmt::Display for VecResponseNicDetails {
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
impl OSCCommand for DiagnosticCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Diagnostic with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.server_id(&self.args.path.server_id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;
        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
