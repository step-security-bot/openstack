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

use bytes::Bytes;
use http::Response;
use openstack_sdk::api::compute::v2::server::add_fixed_ip_21;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Adds a fixed IP address to a server instance, which associates that
/// address with the server. The fixed IP address is retrieved from the
/// network that you specify in the request.
///
/// Specify the `addFixedIp` action and the network ID in the request body.
///
/// Policy defaults enable only users with the administrative role or
/// the owner of the server to perform this operation. Cloud providers
/// can change these permissions through the `policy.json` file.
///
/// Normal response codes: 202
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404)
#[derive(Args, Clone, Debug)]
#[command(about = "Add (Associate) Fixed Ip (addFixedIp Action) (DEPRECATED) (microversion = 2.1)")]
pub struct ServerArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    add_fixed_ip: AddFixedIp,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// id parameter for /v2.1/servers/{id}/action API
    #[arg(value_name = "ID", id = "path_param_id")]
    id: String,
}
/// AddFixedIp Body data
#[derive(Args, Debug, Clone)]
struct AddFixedIp {
    /// The network ID.
    #[arg(long)]
    network_id: String,
}

/// Server action command
pub struct ServerCmd {
    pub args: ServerArgs,
}
/// Server response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {}

#[async_trait]
impl OSCCommand for ServerCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Server with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = add_fixed_ip_21::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.1");

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.add_fixed_ip data
        let args = &self.args.add_fixed_ip;
        let mut add_fixed_ip_builder = add_fixed_ip_21::AddFixedIpBuilder::default();

        add_fixed_ip_builder.network_id(args.network_id.clone());

        ep_builder.add_fixed_ip(add_fixed_ip_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = ResponseData {};
        // Maybe output some headers metadata
        op.output_human::<ResponseData>(&data)?;
        Ok(())
    }
}
