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

use openstack_sdk::api::compute::v2::server::volume_attachment::find;
use openstack_sdk::api::compute::v2::server::volume_attachment::set_285;
use openstack_sdk::api::find;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Update a volume attachment.
///
/// Policy default role is ‘rule:system\_admin\_or\_owner’, its scope is
/// [system, project], which allow project members or system admins to
/// change the fields of an attached volume of a server. Policy defaults
/// enable only users with the administrative role to change `volumeId`
/// via this operation. Cloud providers can change these permissions
/// through the `policy.json` file.
///
/// Updating, or what is commonly referred to as “swapping”, volume attachments
/// with volumes that have more than one read/write attachment, is not
/// supported.
///
/// Normal response codes: 202
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404), conflict(409)
#[derive(Args, Clone, Debug)]
#[command(about = "Update a volume attachment (microversion = 2.85)")]
pub struct VolumeAttachmentArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    volume_attachment: VolumeAttachment,
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

    /// id parameter for /v2.1/servers/{server_id}/os-volume_attachments/{id}
    /// API
    #[arg(value_name = "ID", id = "path_param_id")]
    id: String,
}
/// VolumeAttachment Body data
#[derive(Args, Debug, Clone)]
struct VolumeAttachment {
    /// The UUID of the volume to attach instead of the attached volume.
    #[arg(long)]
    volume_id: String,

    /// Name of the device in the attachment object, such as, `/dev/vdb`.
    ///
    ///
    /// **New in version 2.85**
    #[arg(long)]
    device: Option<String>,

    /// The device tag applied to the volume block device or `null`.
    ///
    ///
    /// **New in version 2.85**
    #[arg(long)]
    tag: Option<String>,

    /// A flag indicating if the attached volume will be deleted when the
    /// server is
    /// deleted.
    ///
    ///
    /// **New in version 2.85**
    #[arg(action=clap::ArgAction::Set, long)]
    delete_on_termination: Option<bool>,

    /// The UUID of the server.
    ///
    ///
    /// **New in version 2.85**
    #[arg(long)]
    server_id: Option<String>,

    /// The UUID of the attachment.
    ///
    ///
    /// **New in version 2.85**
    #[arg(long)]
    id: Option<String>,
}

/// VolumeAttachment set command
pub struct VolumeAttachmentCmd {
    pub args: VolumeAttachmentArgs,
}
/// VolumeAttachment response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// Name of the device in the attachment object, such as, `/dev/vdb`.
    #[serde()]
    #[structable(optional)]
    device: Option<String>,

    /// The volume ID of the attachment.
    ///
    ///
    /// **Available until version 2.88**
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The UUID of the server.
    #[serde(rename = "serverId")]
    #[structable(optional, title = "serverId")]
    server_id: Option<String>,

    /// The UUID of the attached volume.
    #[serde(rename = "volumeId")]
    #[structable(optional, title = "volumeId")]
    volume_id: Option<String>,

    /// The device tag applied to the volume block device or `null`.
    ///
    ///
    /// **New in version 2.70**
    #[serde()]
    #[structable(optional)]
    tag: Option<String>,

    /// A flag indicating if the attached volume will be deleted when the
    /// server is
    /// deleted.
    ///
    ///
    /// **New in version 2.79**
    #[serde()]
    #[structable(optional)]
    delete_on_termination: Option<bool>,

    /// The UUID of the associated volume attachment in Cinder.
    ///
    ///
    /// **New in version 2.89**
    #[serde()]
    #[structable(optional)]
    attachment_id: Option<String>,

    /// The UUID of the block device mapping record in Nova for the attachment.
    ///
    ///
    /// **New in version 2.89**
    #[serde()]
    #[structable(optional)]
    bdm_uuid: Option<String>,
}

#[async_trait]
impl OSCCommand for VolumeAttachmentCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set VolumeAttachment with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut find_builder = find::Request::builder();

        find_builder.server_id(&self.args.path.server_id);
        find_builder.id(&self.args.path.id);
        find_builder.header("OpenStack-API-Version", "compute 2.85");
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;
        let mut ep_builder = set_285::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.85");

        // Set path parameters
        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();
        ep_builder.server_id(resource_id.clone());
        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();
        ep_builder.id(resource_id.clone());
        // Set query parameters
        // Set body parameters
        // Set Request.volume_attachment data
        let args = &self.args.volume_attachment;
        let mut volume_attachment_builder = set_285::VolumeAttachmentBuilder::default();

        volume_attachment_builder.volume_id(args.volume_id.clone());

        if let Some(val) = &args.device {
            volume_attachment_builder.device(Some(val.into()));
        }

        if let Some(val) = &args.tag {
            volume_attachment_builder.tag(val.clone());
        }

        if let Some(val) = &args.delete_on_termination {
            volume_attachment_builder.delete_on_termination(*val);
        }

        if let Some(val) = &args.server_id {
            volume_attachment_builder.server_id(val.clone());
        }

        if let Some(val) = &args.id {
            volume_attachment_builder.id(val.clone());
        }

        ep_builder.volume_attachment(volume_attachment_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
