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

use openstack_sdk::api::compute::v2::server::rescue;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Puts a server in rescue mode and changes its status to `RESCUE`.
///
/// Specify the `rescue` action in the request body.
///
/// If you specify the `rescue\_image\_ref` extended attribute,
/// the image is used to rescue the instance. If you omit an image
/// reference, the base image reference is used by default.
///
/// **Asynchronous Postconditions**
///
/// After you successfully rescue a server and make a `GET
/// /servers/​{server\_id}​` request, its status changes to `RESCUE`.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404), conflict(409), notImplemented(501)
#[derive(Args, Clone, Debug)]
#[command(about = "Rescue Server (rescue Action)")]
pub struct ServerArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    rescue: Option<Rescue>,
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
/// Rescue Body data
#[derive(Args, Debug, Clone)]
struct Rescue {
    #[arg(long)]
    admin_pass: Option<String>,

    #[arg(long)]
    rescue_image_ref: Option<String>,
}

/// Server action command
pub struct ServerCmd {
    pub args: ServerArgs,
}
/// Server response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// An administrative password to access the evacuated instance.
    /// If you set `enable\_instance\_password` configuration option to
    /// `False`,
    /// the API wouldn’t return the `adminPass` field in response.
    ///
    ///
    /// **Available until version 2.13**
    #[serde(rename = "adminPass")]
    #[structable(title = "adminPass")]
    admin_pass: String,
}

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

        let mut ep_builder = rescue::Request::builder();

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.rescue data
        let args = &self.args.rescue;

        if let Some(lrescue) = &args {
            let mut rescue_builder = rescue::RescueBuilder::default();
            if let Some(val) = &lrescue.admin_pass {
                rescue_builder.admin_pass(val.clone());
            }
            if let Some(val) = &lrescue.rescue_image_ref {
                rescue_builder.rescue_image_ref(val.clone());
            }
            ep_builder.rescue(rescue_builder.build().expect("A valid object"));
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
