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

use openstack_sdk::api::compute::v2::server::interface::get;
use openstack_sdk::api::QueryAsync;
use std::fmt;
use structable_derive::StructTable;

/// Shows details for a port interface that is attached to a server.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)
#[derive(Args, Clone, Debug)]
#[command(about = "Show Port Interface Details")]
pub struct InterfaceArgs {
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

    /// id parameter for /v2.1/servers/{server_id}/os-interface/{id} API
    #[arg(value_name = "ID", id = "path_param_id")]
    id: String,
}

/// Interface show command
pub struct InterfaceCmd {
    pub args: InterfaceArgs,
}
/// Interface response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// Fixed IP addresses with subnet IDs.
    #[serde()]
    #[structable(optional)]
    fixed_ips: Option<VecResponseFixedIps>,

    /// The MAC address.
    #[serde()]
    #[structable(optional)]
    mac_addr: Option<String>,

    /// The network ID.
    #[serde()]
    #[structable(optional)]
    net_id: Option<String>,

    /// The port ID.
    #[serde()]
    #[structable(optional)]
    port_id: Option<String>,

    /// The port state.
    #[serde()]
    #[structable(optional)]
    port_state: Option<String>,

    /// The device tag applied to the virtual network interface or `null`.
    ///
    ///
    /// **New in version 2.70**
    #[serde()]
    #[structable(optional)]
    tag: Option<String>,
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseFixedIps {
    ip_address: Option<String>,
    subnet_id: Option<String>,
}

impl fmt::Display for ResponseFixedIps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "ip_address={}",
                self.ip_address
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "subnet_id={}",
                self.subnet_id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseFixedIps(Vec<ResponseFixedIps>);
impl fmt::Display for VecResponseFixedIps {
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
impl OSCCommand for InterfaceCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Interface with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.server_id(&self.args.path.server_id);
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
