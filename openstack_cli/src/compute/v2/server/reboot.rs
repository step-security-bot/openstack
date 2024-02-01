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
use clap::ValueEnum;
use http::Response;
use openstack_sdk::api::compute::v2::server::reboot;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Reboots a server.
///
/// Specify the `reboot` action in the request body.
///
/// **Preconditions**
///
/// The preconditions for rebooting a server depend on the type of reboot.
///
/// You can only *SOFT* reboot a server when its status is `ACTIVE`.
///
/// You can only *HARD* reboot a server when its status is one of:
///
/// If the server is locked, you must have administrator privileges
/// to reboot the server.
///
/// **Asynchronous Postconditions**
///
/// After you successfully reboot a server, its status changes to `ACTIVE`.
///
/// Normal response codes: 202
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404),
/// conflict(409)
#[derive(Args, Clone, Debug)]
#[command(about = "Reboot Server (reboot Action)")]
pub struct ServerArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    reboot: Reboot,
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

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Type {
    Hard,
    Soft,
}

/// Reboot Body data
#[derive(Args, Debug, Clone)]
struct Reboot {
    /// The type of the reboot action. The valid values are `HARD` and `SOFT`.
    /// A `SOFT` reboot attempts a graceful shutdown and restart of the server.
    /// A `HARD` reboot attempts a forced shutdown and restart of the server.
    /// The `HARD` reboot corresponds to the power cycles of the server.
    #[arg(long)]
    _type: Type,
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

        let mut ep_builder = reboot::Request::builder();

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.reboot data
        let args = &self.args.reboot;
        let mut reboot_builder = reboot::RebootBuilder::default();

        let tmp = match &args._type {
            Type::Hard => reboot::Type::Hard,
            Type::Soft => reboot::Type::Soft,
        };
        reboot_builder._type(tmp);

        ep_builder.reboot(reboot_builder.build().unwrap());

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
