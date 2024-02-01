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
use openstack_sdk::api::compute::v2::server::lock_273;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Locks a server.
///
/// Specify the `lock` action in the request body.
///
/// Most actions by non-admin users are not allowed to the server
/// after this operation is successful and the server is locked.
/// See the “Lock, Unlock” item in [Server
/// actions](https://docs.openstack.org/api-
/// guide/compute/server_concepts.html#server-actions)
/// for the restricted actions.
/// But administrators can perform actions on the server
/// even though the server is locked. Note that from microversion 2.73 it is
/// possible to specify a reason when locking the server.
///
/// The [unlock action](https://docs.openstack.org/api-ref/compute/#unlock-
/// server-unlock-action)
/// will unlock a server in locked state so additional actions can
/// be performed on the server by non-admin users.
///
/// You can know whether a server is locked or not and the `locked\_reason`
/// (if specified, from the 2.73 microversion) by the [List Servers Detailed
/// API](https://docs.openstack.org/api-ref/compute/#list-servers-detailed)
/// or
/// the [Show Server Details API](https://docs.openstack.org/api-
/// ref/compute/#show-server-details).
///
/// Policy defaults enable only users with the administrative role or
/// the owner of the server to perform this operation. Cloud providers
/// can change these permissions through the `policy.json` file.
/// Administrators can overwrite owner’s lock.
///
/// Normal response codes: 202
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)
#[derive(Args, Clone, Debug)]
#[command(about = "Lock Server (lock Action) (microversion = 2.73)")]
pub struct ServerArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    lock: Option<Lock>,
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
/// Lock Body data
#[derive(Args, Debug, Clone)]
struct Lock {
    #[arg(long)]
    locked_reason: Option<String>,
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

        let mut ep_builder = lock_273::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.73");

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.lock data
        let args = &self.args.lock;

        if let Some(llock) = &args {
            let mut lock_builder = lock_273::LockBuilder::default();
            if let Some(val) = &llock.locked_reason {
                lock_builder.locked_reason(val.clone());
            }
            ep_builder.lock(lock_builder.build().expect("A valid object"));
        }

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
