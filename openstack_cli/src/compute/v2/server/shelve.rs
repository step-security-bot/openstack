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
use openstack_sdk::api::compute::v2::server::shelve;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Shelves a server.
///
/// Specify the `shelve` action in the request body.
///
/// All associated data and resources are kept but anything still in memory is
/// not retained. To restore a shelved instance, use the `unshelve` action. To
/// remove a shelved instance, use the `shelveOffload` action.
///
/// Policy defaults enable only users with the administrative role or the owner
/// of the server to perform this operation. Cloud providers can change these
/// permissions through the `policy.json` file.
///
/// **Preconditions**
///
/// The server status must be `ACTIVE`, `SHUTOFF`, `PAUSED`, or `SUSPENDED`.
///
/// If the server is locked, you must have administrator privileges to shelve
/// the server.
///
/// **Asynchronous Postconditions**
///
/// After you successfully shelve a server, its status changes to `SHELVED` and
/// the image status is `ACTIVE`. The server instance data appears on the
/// compute node that the Compute service manages.
///
/// If you boot the server from volumes or set the `shelved\_offload\_time`
/// option to 0, the Compute service automatically deletes the instance on
/// compute nodes and changes the server status to `SHELVED\_OFFLOADED`.
///
/// **Troubleshooting**
///
/// If the server status does not change to `SHELVED` or `SHELVED\_OFFLOADED`,
/// the shelve operation failed. Ensure that you meet the preconditions and run
/// the request again. If the request fails again, investigate whether another
/// operation is running that causes a race condition.
///
/// Normal response codes: 202
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404),
/// conflict(409)
#[derive(Args, Clone, Debug)]
#[command(about = "Shelve Server (shelve Action)")]
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

        let mut ep_builder = shelve::Request::builder();

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
