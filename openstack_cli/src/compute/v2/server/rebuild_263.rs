// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0
//
// WARNING: This file is automatically generated from OpenAPI schema using
// `openstack-codegenerator`.

//! Action Server command [microversion = 2.63]
//!
//! Wraps invoking of the `v2.1/servers/{id}/action` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::parse_key_val;
use bytes::Bytes;
use clap::ValueEnum;
use http::Response;
use openstack_sdk::api::compute::v2::server::rebuild_263;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Command without description in OpenAPI
///
#[derive(Args)]
#[command(about = "Rebuild Server (rebuild Action) (microversion = 2.63)")]
pub struct ServerCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// The action to rebuild a server.
    ///
    #[command(flatten)]
    rebuild: Rebuild,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.1/servers/{id}/action API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum OsDcfDiskConfig {
    Auto,
    Manual,
}

/// Rebuild Body data
#[derive(Args, Clone)]
struct Rebuild {
    /// IPv4 address that should be used to access this server.
    ///
    #[arg(help_heading = "Body parameters", long)]
    access_ipv4: Option<String>,

    /// IPv6 address that should be used to access this server.
    ///
    #[arg(help_heading = "Body parameters", long)]
    access_ipv6: Option<String>,

    /// The administrative password of the server. If you omit this parameter,
    /// the operation generates a new password.
    ///
    #[arg(help_heading = "Body parameters", long)]
    admin_pass: Option<String>,

    /// A free form description of the server. Limited to 255 characters in
    /// length. Before microversion 2.19 this was set to the server name.
    ///
    /// **New in version 2.19**
    ///
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// The UUID of the image to rebuild for your server instance. It must be a
    /// valid UUID otherwise API will return 400. To rebuild a volume-backed
    /// server with a new image, at least microversion 2.93 needs to be
    /// provided in the request else the request will fall back to old
    /// behaviour i.e. the API will return 400 (for an image different from the
    /// image used when creating the volume). For non-volume-backed servers,
    /// specifying a new image will result in validating that the image is
    /// acceptable for the current compute host on which the server exists. If
    /// the new image is not valid, the server will go into `ERROR` status.
    ///
    #[arg(help_heading = "Body parameters", long)]
    image_ref: String,

    /// Key pair name for rebuild API. If `null` is specified, the existing
    /// keypair is unset.
    ///
    /// Note
    ///
    /// Users within the same project are able to rebuild other user’s
    /// instances in that project with a new keypair. Keys are owned by users
    /// (which is the only resource that’s true of). Servers are owned by
    /// projects. Because of this a rebuild with a key_name is looking up the
    /// keypair by the user calling rebuild.
    ///
    /// **New in version 2.54**
    ///
    #[arg(help_heading = "Body parameters", long)]
    key_name: Option<String>,

    /// Metadata key and value pairs. The maximum size of the metadata key and
    /// value is 255 bytes each.
    ///
    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    metadata: Option<Vec<(String, String)>>,

    /// The server name.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// Controls how the API partitions the disk when you create, rebuild, or
    /// resize servers. A server inherits the `OS-DCF:diskConfig` value from
    /// the image from which it was created, and an image inherits the
    /// `OS-DCF:diskConfig` value from the server from which it was created. To
    /// override the inherited setting, you can include this attribute in the
    /// request body of a server create, rebuild, or resize request. If the
    /// `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a
    /// server from that image and set its `OS-DCF:diskConfig` value to `AUTO`.
    /// A valid value is:
    ///
    /// - `AUTO`. The API builds the server with a single partition the size of
    ///   the target flavor disk. The API automatically adjusts the file system
    ///   to fit the entire partition.
    /// - `MANUAL`. The API builds the server by using whatever partition
    ///   scheme and file system is in the source image. If the target flavor
    ///   disk is larger, the API does not partition the remaining disk space.
    ///
    #[arg(help_heading = "Body parameters", long)]
    os_dcf_disk_config: Option<OsDcfDiskConfig>,

    /// Indicates whether the server is rebuilt with the preservation of the
    /// ephemeral partition (`true`).
    ///
    /// Note
    ///
    /// This only works with baremetal servers provided by Ironic. Passing it
    /// to any other server instance results in a fault and will prevent the
    /// rebuild from happening.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    preserve_ephemeral: Option<bool>,

    /// A list of trusted certificate IDs, which are used during image
    /// signature verification to verify the signing certificate. The list is
    /// restricted to a maximum of 50 IDs. This parameter is optional in server
    /// rebuild requests if allowed by policy, and is not supported for
    /// volume-backed instances.
    ///
    /// If `null` is specified, the existing trusted certificate IDs are either
    /// unset or reset to the configured defaults.
    ///
    /// **New in version 2.63**
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    trusted_image_certificates: Option<Vec<String>>,

    /// Configuration information or scripts to use upon rebuild. Must be
    /// Base64 encoded. Restricted to 65535 bytes. If `null` is specified, the
    /// existing user_data is unset.
    ///
    /// **New in version 2.57**
    ///
    #[arg(help_heading = "Body parameters", long)]
    user_data: Option<String>,
}

/// Server response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl ServerCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Server");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = rebuild_263::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.63");

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.rebuild data
        let args = &self.rebuild;
        let mut rebuild_builder = rebuild_263::RebuildBuilder::default();
        if let Some(val) = &args.name {
            rebuild_builder.name(val);
        }

        rebuild_builder.image_ref(&args.image_ref);

        if let Some(val) = &args.admin_pass {
            rebuild_builder.admin_pass(val);
        }

        if let Some(val) = &args.metadata {
            rebuild_builder.metadata(val.iter().cloned());
        }

        if let Some(val) = &args.preserve_ephemeral {
            rebuild_builder.preserve_ephemeral(*val);
        }

        if let Some(val) = &args.os_dcf_disk_config {
            let tmp = match val {
                OsDcfDiskConfig::Auto => rebuild_263::OsDcfDiskConfig::Auto,
                OsDcfDiskConfig::Manual => rebuild_263::OsDcfDiskConfig::Manual,
            };
            rebuild_builder.os_dcf_disk_config(tmp);
        }

        if let Some(val) = &args.access_ipv4 {
            rebuild_builder.access_ipv4(val);
        }

        if let Some(val) = &args.access_ipv6 {
            rebuild_builder.access_ipv6(val);
        }

        if let Some(val) = &args.description {
            rebuild_builder.description(Some(val.into()));
        }

        if let Some(val) = &args.key_name {
            rebuild_builder.key_name(Some(val.into()));
        }

        if let Some(val) = &args.user_data {
            rebuild_builder.user_data(Some(val.into()));
        }

        if let Some(val) = &args.trusted_image_certificates {
            rebuild_builder
                .trusted_image_certificates(val.iter().map(|v| v.into()).collect::<Vec<_>>());
        }

        ep_builder.rebuild(rebuild_builder.build().unwrap());

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
