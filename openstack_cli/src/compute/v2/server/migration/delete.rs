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
use openstack_sdk::api::compute::v2::server::migration::delete;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Abort an in-progress live migration.
///
/// Policy defaults enable only users with the administrative role to perform
/// this operation. Cloud providers can change these permissions through the
/// `policy.json` file.
///
/// **Preconditions**
///
/// The server OS-EXT-STS:task\_state value must be `migrating`.
///
/// If the server is locked, you must have administrator privileges to force
/// the
/// completion of the server migration.
///
/// For microversions from 2.24 to 2.64 the migration status must be `running`,
/// for microversion 2.65 and greater, the migration status can also be
/// `queued`
/// and `preparing`.
///
/// **Asynchronous Postconditions**
///
/// After you make this request, you typically must keep polling the server
/// status
/// to determine whether the request succeeded. You may also monitor the
/// migration
/// using:
///
/// **Troubleshooting**
///
/// If the server status remains `MIGRATING` for an inordinate amount of
/// time, the request may have failed. Ensure you meet the preconditions and
/// run
/// the request again. If the request fails again, investigate the compute back
/// end.
///
/// Normal response codes: 202
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404), conflict(409)
#[derive(Args, Clone, Debug)]
#[command(about = "Delete (Abort) Migration")]
pub struct MigrationArgs {
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

    /// id parameter for /v2.1/servers/{server_id}/migrations/{id}/action API
    #[arg(value_name = "ID", id = "path_param_id")]
    id: String,
}

/// Migration delete command
pub struct MigrationCmd {
    pub args: MigrationArgs,
}
/// Migration response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {}

#[async_trait]
impl OSCCommand for MigrationCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Delete Migration with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = delete::Request::builder();

        // Set path parameters
        ep_builder.server_id(&self.args.path.server_id);
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        Ok(())
    }
}
