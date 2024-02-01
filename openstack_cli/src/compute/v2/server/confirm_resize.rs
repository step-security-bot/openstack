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
use openstack_sdk::api::compute::v2::server::confirm_resize;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Confirms a pending resize action for a server.
///
/// Specify the `confirmResize` action in the request body.
///
/// After you make this request, you typically must keep polling the server
/// status to determine whether the request succeeded. A successfully
/// confirming resize operation shows a status of `ACTIVE` or `SHUTOFF`
/// and a migration status of `confirmed`. You can also see the resized
/// server in the compute node that OpenStack Compute manages.
///
/// **Preconditions**
///
/// You can only confirm the resized server where the status is
/// `VERIFY\_RESIZE`.
///
/// If the server is locked, you must have administrator privileges
/// to confirm the server.
///
/// **Troubleshooting**
///
/// If the server status remains `VERIFY\_RESIZE`, the request failed. Ensure
/// you
/// meet the preconditions and run the request again. If the request fails
/// again, the server status should be `ERROR` and a migration status of
/// `error`. Investigate the compute back end or ask your cloud provider.
/// There are some options for trying to correct the server status:
///
/// Note that the cloud provider may still need to cleanup any orphaned
/// resources
/// on the source hypervisor.
///
/// Normal response codes: 204
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404), conflict(409)
#[derive(Args, Clone, Debug)]
#[command(about = "Confirm Resized Server (confirmResize Action)")]
pub struct ServerArgs {
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
    /// id parameter for /v2.1/servers/{id}/action API
    #[arg(value_name = "ID", id = "path_param_id")]
    id: String,
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

        let mut ep_builder = confirm_resize::Request::builder();

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters

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
