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
use openstack_sdk::api::compute::v2::server::shelve_offload;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Shelf-offloads, or removes, a shelved server.
///
/// Specify the `shelveOffload` action in the request body.
///
/// Data and resource associations are deleted. If an instance is no longer
/// needed, you can remove that instance from the hypervisor to minimize
/// resource usage.
///
/// Policy defaults enable only users with the administrative role or the owner
/// of the server to perform this operation. Cloud providers can change these
/// permissions through the `policy.json` file.
///
/// **Preconditions**
///
/// The server status must be `SHELVED`.
///
/// If the server is locked, you must have administrator privileges to shelve-
/// offload the server.
///
/// **Asynchronous Postconditions**
///
/// After you successfully shelve-offload a server, its status changes to
/// `SHELVED\_OFFLOADED`. The server instance data appears on the compute node.
///
/// **Troubleshooting**
///
/// If the server status does not change to `SHELVED\_OFFLOADED`, the shelve-
/// offload operation failed. Ensure that you meet the preconditions and run
/// the request again. If the request fails again, investigate whether another
/// operation is running that causes a race condition.
///
/// Normal response codes: 202
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404),
/// conflict(409)
#[derive(Args, Clone, Debug)]
#[command(about = "Shelf-Offload (Remove) Server (shelveOffload Action)")]
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

        let mut ep_builder = shelve_offload::Request::builder();

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
