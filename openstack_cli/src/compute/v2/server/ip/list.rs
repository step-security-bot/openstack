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

use openstack_sdk::api::compute::v2::server::ip::list;
use openstack_sdk::api::QueryAsync;

use structable_derive::StructTable;

/// Lists IP addresses that are assigned to an instance.
///
/// Policy defaults enable only users with the administrative role or the owner
/// of
/// the server to perform this operation. Cloud providers can change these
/// permissions through the `policy.json` file.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)
#[derive(Args, Clone, Debug)]
#[command(about = "List Ips")]
pub struct IpsArgs {
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

/// Ips list command
pub struct IpsCmd {
    pub args: IpsArgs,
}
/// Ips response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The IP address.
    #[serde()]
    #[structable(optional)]
    addr: Option<String>,

    /// The IP version of the address associated with server.
    #[serde()]
    #[structable(optional)]
    version: Option<i32>,
}

#[async_trait]
impl OSCCommand for IpsCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Ips with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = list::Request::builder();

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
