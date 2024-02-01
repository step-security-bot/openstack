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
use openstack_sdk::api::compute::v2::server::remove_floating_ip_21;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Removes, or disassociates, a floating IP address from a server.
///
/// The IP address is returned to the pool of IP addresses that is available
/// for all projects. When you remove a floating IP address and that IP address
/// is still associated with a running instance, it is automatically
/// disassociated from that instance.
///
/// Specify the `removeFloatingIp` action in the request body.
///
/// Normal response codes: 202
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404), conflict(409)
#[derive(Args, Clone, Debug)]
#[command(
    about = "Remove (Disassociate) Floating Ip (removeFloatingIp Action) (DEPRECATED) (microversion = 2.1)"
)]
pub struct ServerArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    remove_floating_ip: RemoveFloatingIp,
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
/// RemoveFloatingIp Body data
#[derive(Args, Debug, Clone)]
struct RemoveFloatingIp {
    /// The floating IP address.
    #[arg(long)]
    address: String,
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

        let mut ep_builder = remove_floating_ip_21::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.1");

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.remove_floating_ip data
        let args = &self.args.remove_floating_ip;
        let mut remove_floating_ip_builder =
            remove_floating_ip_21::RemoveFloatingIpBuilder::default();

        remove_floating_ip_builder.address(args.address.clone());

        ep_builder.remove_floating_ip(remove_floating_ip_builder.build().unwrap());

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
