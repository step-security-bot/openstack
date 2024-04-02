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

//! Create Backup command [microversion = 3.51]
//!
//! Wraps invoking of the `v3/backups` with `POST` method

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
use openstack_sdk::api::block_storage::v3::backup::create_351;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Create a new backup.
///
#[derive(Args)]
pub struct BackupCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `backup` object.
    ///
    #[command(flatten)]
    backup: Backup,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Backup Body data
#[derive(Args, Clone)]
struct Backup {
    /// The backup availability zone key value pair.
    ///
    /// **New in version 3.51**
    ///
    #[arg(help_heading = "Body parameters", long)]
    availability_zone: Option<String>,

    /// The container name or null.
    ///
    #[arg(help_heading = "Body parameters", long)]
    container: Option<String>,

    /// The backup description or null.
    ///
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// Indicates whether to backup, even if the volume is attached. Default is
    /// `false`. See [valid boolean values](#valid-boolean-values)
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    force: Option<bool>,

    /// Indicates whether to backup, even if the volume is attached. Default is
    /// `false`. See [valid boolean values](#valid-boolean-values)
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    incremental: Option<bool>,

    /// The backup metadata key value pairs.
    ///
    /// **New in version 3.43**
    ///
    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    metadata: Option<Vec<(String, String)>>,

    /// The name of the Volume Backup.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// The UUID of the source snapshot that you want to back up.
    ///
    #[arg(help_heading = "Body parameters", long)]
    snapshot_id: Option<String>,

    /// The UUID of the volume that you want to back up.
    ///
    #[arg(help_heading = "Body parameters", long)]
    volume_id: String,
}

/// Backup response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The name of the availability zone.
    ///
    #[serde()]
    #[structable(optional)]
    availability_zone: Option<String>,

    /// The container name or null.
    ///
    #[serde()]
    #[structable(optional)]
    container: Option<String>,

    /// The date and time when the resource was created. The date and time
    /// stamp format is ISO 8601
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// The time when the data on the volume was first saved. If it is a backup
    /// from volume, it will be the same as created_at for a backup. If it is a
    /// backup from a snapshot, it will be the same as created_at for the
    /// snapshot.
    ///
    #[serde()]
    #[structable(optional)]
    data_timestamp: Option<String>,

    /// The backup description or null.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// If the backup failed, the reason for the failure. Otherwise, null.
    ///
    #[serde()]
    #[structable(optional)]
    fail_reason: Option<String>,

    /// If this value is true, there are other backups depending on this
    /// backup.
    ///
    #[serde()]
    #[structable(optional)]
    has_dependent_backups: Option<bool>,

    /// The UUID of the backup.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Indicates whether the backup mode is incremental. If this value is
    /// true, the backup mode is incremental. If this value is false, the
    /// backup mode is full.
    ///
    #[serde()]
    #[structable(optional)]
    is_incremental: Option<bool>,

    /// Links for the backup.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    links: Option<Value>,

    /// The backup metadata key value pairs.
    ///
    /// **New in version 3.43**
    ///
    #[serde()]
    #[structable(optional, pretty)]
    metadata: Option<Value>,

    /// The backup name.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The number of objects in the backup.
    ///
    #[serde()]
    #[structable(optional)]
    object_count: Option<i32>,

    /// The size of the volume, in gibibytes (GiB).
    ///
    #[serde()]
    #[structable(optional)]
    size: Option<i64>,

    /// The UUID of the source volume snapshot.
    ///
    #[serde()]
    #[structable(optional)]
    snapshot_id: Option<String>,

    /// The backup status. Refer to Backup statuses table for the possible
    /// status value.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The date and time when the resource was updated. The date and time
    /// stamp format is ISO 8601
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The UUID of the volume.
    ///
    #[serde()]
    #[structable(optional)]
    volume_id: Option<String>,
}

impl BackupCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Backup");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create_351::Request::builder();
        ep_builder.header("OpenStack-API-Version", "volume 3.51");

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.backup data
        let args = &self.backup;
        let mut backup_builder = create_351::BackupBuilder::default();

        backup_builder.volume_id(&args.volume_id);

        if let Some(val) = &args.container {
            backup_builder.container(Some(val.into()));
        }

        if let Some(val) = &args.description {
            backup_builder.description(Some(val.into()));
        }

        if let Some(val) = &args.incremental {
            backup_builder.incremental(*val);
        }

        if let Some(val) = &args.force {
            backup_builder.force(*val);
        }

        if let Some(val) = &args.name {
            backup_builder.name(Some(val.into()));
        }

        if let Some(val) = &args.snapshot_id {
            backup_builder.snapshot_id(Some(val.into()));
        }

        if let Some(val) = &args.metadata {
            backup_builder.metadata(val.iter().cloned());
        }

        if let Some(val) = &args.availability_zone {
            backup_builder.availability_zone(Some(val.into()));
        }

        ep_builder.backup(backup_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
