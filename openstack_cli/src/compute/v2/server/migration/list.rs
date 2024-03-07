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

//! List Migrations command
//!
//! Wraps invoking of the `v2.1/servers/{server_id}/migrations` with `GET` method

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

use openstack_sdk::api::compute::v2::server::migration::list;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Lists in-progress live migrations for a given server.
///
/// Policy defaults enable only users with the administrative role to perform
/// this operation. Cloud providers can change these permissions through the
/// `policy.json` file.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)
///
#[derive(Args)]
#[command(about = "List Migrations")]
pub struct MigrationsCommand {
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
    /// server_id parameter for /v2.1/servers/{server_id}/topology API
    ///
    #[arg(id = "path_param_server_id", value_name = "SERVER_ID")]
    server_id: String,
}
/// Migrations response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The date and time when the resource was created. The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm` value, if
    /// included, is the time zone as an offset from UTC. In the previous
    /// example, the offset value is `-05:00`.
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// The target compute for a migration.
    ///
    #[serde()]
    #[structable(optional, wide)]
    dest_compute: Option<String>,

    /// The target host for a migration.
    ///
    #[serde()]
    #[structable(optional, wide)]
    dest_host: Option<String>,

    /// The target node for a migration.
    ///
    #[serde()]
    #[structable(optional, wide)]
    dest_node: Option<String>,

    /// The amount of disk, in bytes, that has been processed during the
    /// migration.
    ///
    #[serde()]
    #[structable(optional, wide)]
    disk_processed_bytes: Option<i32>,

    /// The amount of disk, in bytes, that still needs to be migrated.
    ///
    #[serde()]
    #[structable(optional, wide)]
    disk_remaining_bytes: Option<i32>,

    /// The total amount of disk, in bytes, that needs to be migrated.
    ///
    #[serde()]
    #[structable(optional, wide)]
    disk_total_bytes: Option<i32>,

    /// The ID of the server migration.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<i32>,

    /// The amount of memory, in bytes, that has been processed during the
    /// migration.
    ///
    #[serde()]
    #[structable(optional, wide)]
    memory_processed_bytes: Option<i32>,

    /// The amount of memory, in bytes, that still needs to be migrated.
    ///
    #[serde()]
    #[structable(optional, wide)]
    memory_remaining_bytes: Option<i32>,

    /// The total amount of memory, in bytes, that needs to be migrated.
    ///
    #[serde()]
    #[structable(optional, wide)]
    memory_total_bytes: Option<i32>,

    /// The ID of the project which initiated the server migration. The value
    /// may be `null` for older migration records.
    ///
    /// **New in version 2.80**
    ///
    #[serde()]
    #[structable(optional, wide)]
    project_id: Option<String>,

    /// The UUID of the server.
    ///
    #[serde()]
    #[structable(optional, wide)]
    server_uuid: Option<String>,

    /// The source compute for a migration.
    ///
    #[serde()]
    #[structable(optional, wide)]
    source_compute: Option<String>,

    /// The source node for a migration.
    ///
    #[serde()]
    #[structable(optional, wide)]
    source_node: Option<String>,

    /// The current status of the migration.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The date and time when the resource was updated. The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm` value, if
    /// included, is the time zone as an offset from UTC. In the previous
    /// example, the offset value is `-05:00`.
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The ID of the user which initiated the server migration. The value may
    /// be `null` for older migration records.
    ///
    /// **New in version 2.80**
    ///
    #[serde()]
    #[structable(optional, wide)]
    user_id: Option<String>,

    /// The UUID of the migration.
    ///
    /// **New in version 2.59**
    ///
    #[serde()]
    #[structable(optional)]
    uuid: Option<String>,
}

impl MigrationsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Migrations");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.server_id(&self.path.server_id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
