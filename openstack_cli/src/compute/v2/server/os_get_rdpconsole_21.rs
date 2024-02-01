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

use clap::ValueEnum;
use openstack_sdk::api::compute::v2::server::os_get_rdpconsole_21;
use openstack_sdk::api::QueryAsync;
use std::fmt;
use structable_derive::StructTable;

/// Gets an [RDP](https://technet.microsoft.com/en-us/windowsserver/ee236407)
/// console for a server.
///
/// The only supported connect type is `rdp-html5`. The `type` parameter should
/// be set as `rdp-html5`.
///
/// Specify the `os-getRDPConsole` action in the request body.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404),
/// conflict(409), notImplemented(501)
#[derive(Args, Clone, Debug)]
#[command(about = "Get RDP Console (os-getRDPConsole Action) (DEPRECATED) (microversion = 2.1)")]
pub struct ServerArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    os_get_rdpconsole: OsGetRdpconsole,
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
    RdpHtml5,
}

/// OsGetRdpconsole Body data
#[derive(Args, Debug, Clone)]
struct OsGetRdpconsole {
    /// The type of RDP console. The only valid value is `rdp-html5`.
    #[arg(long)]
    _type: Type,
}

/// Server action command
pub struct ServerCmd {
    pub args: ServerArgs,
}
/// Server response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The remote console object.
    #[serde()]
    #[structable()]
    console: ResponseConsole,
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseConsole {
    _type: Option<String>,
    url: Option<String>,
}

impl fmt::Display for ResponseConsole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "_type={}",
                self._type
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "url={}",
                self.url
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
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

        let mut ep_builder = os_get_rdpconsole_21::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.1");

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.os_get_rdpconsole data
        let args = &self.args.os_get_rdpconsole;
        let mut os_get_rdpconsole_builder = os_get_rdpconsole_21::OsGetRdpconsoleBuilder::default();

        let tmp = match &args._type {
            Type::RdpHtml5 => os_get_rdpconsole_21::Type::RdpHtml5,
        };
        os_get_rdpconsole_builder._type(tmp);

        ep_builder.os_get_rdpconsole(os_get_rdpconsole_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
