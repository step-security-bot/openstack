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

//! Show Member command
//!
//! Wraps invoking of the `v2/images/{image_id}/members/{member_id}` with `GET` method

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

use openstack_sdk::api::image::v2::image::member::get;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Shows image member details.
/// *(Since Image API v2.1)*
///
/// Response body is a single image member entity.
///
/// Preconditions
///
/// Normal response codes: 200
///
/// Error response codes: 400, 401, 404
#[derive(Args)]
#[command(about = "Show image member details")]
pub struct MemberCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// image_id parameter for /v2/images/{image_id}/members/{member_id} API
    #[arg(id = "path_param_image_id", value_name = "IMAGE_ID")]
    image_id: String,

    /// member_id parameter for /v2/images/{image_id}/members/{member_id} API
    #[arg(id = "path_param_id", value_name = "ID")]
    id: String,
}
/// Member response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The ID of the image member. An image member is usually a project (also
    /// called the “tenant”) with whom the image is shared.
    #[serde()]
    #[structable(optional)]
    member_id: Option<String>,

    /// The UUID of the image.
    #[serde()]
    #[structable(optional)]
    image_id: Option<String>,

    /// The date and time when the resource was created.
    ///
    ///
    /// The date and time stamp format is [ISO
    /// 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    ///
    /// The `±hh:mm` value, if included, is the time zone as an offset
    /// from UTC.
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// The date and time when the resource was updated.
    ///
    ///
    /// The date and time stamp format is [ISO
    /// 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    ///
    /// The `±hh:mm` value, if included, is the time zone as an offset
    /// from UTC. In the previous example, the offset value is `-05:00`.
    ///
    ///
    /// If the `updated\_at` date and time stamp is not set, its value is
    /// `null`.
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The status of this image member. Value is one of `pending`,
    /// `accepted`, `rejected`.
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The URL for the schema describing an image member.
    #[serde()]
    #[structable(optional)]
    schema: Option<String>,
}

impl MemberCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Member");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.image_id(&self.path.image_id);
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
