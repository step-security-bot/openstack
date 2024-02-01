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
use openstack_sdk::api::compute::v2::server::resize;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Resizes a server.
///
/// Specify the `resize` action in the request body.
///
/// **Preconditions**
///
/// You can only resize a server when its status is `ACTIVE` or `SHUTOFF`.
///
/// If the server is locked, you must have administrator privileges
/// to resize the server.
///
/// **Asynchronous Postconditions**
///
/// A successfully resized server shows a `VERIFY\_RESIZE` status and
/// `finished`
/// migration status. If the cloud has configured the [resize\_confirm\_window]
/// (https://docs.openstack.org/nova/latest/configuration/config.html#DEFAULT.r
/// esize_confirm_window)
/// option of the Compute service to a positive value, the Compute service
/// automatically confirms the resize operation after the configured interval.
///
/// Normal response codes: 202
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404), conflict(409)
#[derive(Args, Clone, Debug)]
#[command(about = "Resize Server (resize Action)")]
pub struct ServerArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    resize: Resize,
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
enum OsDcfDiskConfig {
    Auto,
    Manual,
}

/// Resize Body data
#[derive(Args, Debug, Clone)]
struct Resize {
    /// The flavor ID for resizing the server. The size of the disk in the
    /// flavor
    /// being resized to must be greater than or equal to the size of the disk
    /// in
    /// the current flavor.
    ///
    ///
    /// If a specified flavor ID is the same as the current one of the server,
    /// the request returns a `Bad Request (400)` response code.
    #[arg(long)]
    flavor_ref: String,

    /// Controls how the API partitions the disk when you create, rebuild, or
    /// resize servers.
    /// A server inherits the `OS-DCF:diskConfig` value from the image from
    /// which it
    /// was created, and an image inherits the `OS-DCF:diskConfig` value from
    /// the server
    /// from which it was created. To override the inherited setting, you can
    /// include
    /// this attribute in the request body of a server create, rebuild, or
    /// resize request. If
    /// the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot
    /// create
    /// a server from that image and set its `OS-DCF:diskConfig` value to
    /// `AUTO`.
    /// A valid value is:
    ///
    ///
    /// * `AUTO`. The API builds the server with a single partition the size of
    /// the
    /// target flavor disk. The API automatically adjusts the file system to
    /// fit the
    /// entire partition.
    /// * `MANUAL`. The API builds the server by using whatever partition
    /// scheme and
    /// file system is in the source image. If the target flavor disk is
    /// larger, the API
    /// does not partition the remaining disk space.
    #[arg(long)]
    os_dcf_disk_config: Option<OsDcfDiskConfig>,
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

        let mut ep_builder = resize::Request::builder();

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.resize data
        let args = &self.args.resize;
        let mut resize_builder = resize::ResizeBuilder::default();

        resize_builder.flavor_ref(args.flavor_ref.clone());

        if let Some(val) = &args.os_dcf_disk_config {
            let tmp = match val {
                OsDcfDiskConfig::Auto => resize::OsDcfDiskConfig::Auto,
                OsDcfDiskConfig::Manual => resize::OsDcfDiskConfig::Manual,
            };
            resize_builder.os_dcf_disk_config(tmp);
        }

        ep_builder.resize(resize_builder.build().unwrap());

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
