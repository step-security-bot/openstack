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

use openstack_sdk::api::block_storage::v3::volume::os_migrate_volume_316;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Args, Clone, Debug)]
pub struct VolumeArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    os_migrate_volume: OsMigrateVolume,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// id parameter for /v3/volumes/{id} API
    #[arg(id = "path_param_id", value_name = "ID")]
    id: String,
}
/// OsMigrateVolume Body data
#[derive(Args, Debug, Clone)]
struct OsMigrateVolume {
    #[arg(long)]
    host: Option<String>,

    #[arg(action=clap::ArgAction::Set, long)]
    force_host_copy: Option<bool>,

    #[arg(action=clap::ArgAction::Set, long)]
    lock_volume: Option<bool>,

    #[arg(long)]
    cluster: Option<String>,
}

/// Volume action command
pub struct VolumeCmd {
    pub args: VolumeArgs,
}
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct ResponseData(HashMap<String, serde_json::Value>);

impl StructTable for ResponseData {
    fn build(&self, _options: &OutputConfig) -> (Vec<String>, Vec<Vec<String>>) {
        let headers: Vec<String> = Vec::from(["Name".to_string(), "Value".to_string()]);
        let mut rows: Vec<Vec<String>> = Vec::new();
        rows.extend(self.0.iter().map(|(k, v)| {
            Vec::from([
                k.clone(),
                serde_json::to_string(&v).expect("Is a valid data"),
            ])
        }));
        (headers, rows)
    }
}

#[async_trait]
impl OSCCommand for VolumeCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Volume with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = os_migrate_volume_316::Request::builder();
        ep_builder.header("OpenStack-API-Version", "volume 3.16");

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.os_migrate_volume data
        let args = &self.args.os_migrate_volume;
        let mut os_migrate_volume_builder =
            os_migrate_volume_316::OsMigrateVolumeBuilder::default();
        if let Some(val) = &args.host {
            os_migrate_volume_builder.host(Some(val.into()));
        }

        if let Some(val) = &args.force_host_copy {
            os_migrate_volume_builder.force_host_copy(*val);
        }

        if let Some(val) = &args.lock_volume {
            os_migrate_volume_builder.lock_volume(*val);
        }

        if let Some(val) = &args.cluster {
            os_migrate_volume_builder.cluster(Some(val.into()));
        }

        ep_builder.os_migrate_volume(os_migrate_volume_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
