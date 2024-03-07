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

//! Action Server command [microversion = 2.0]
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

use openstack_sdk::api::compute::v2::server::evacuate_20;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Command without description in OpenAPI
///
#[derive(Args)]
#[command(about = "Evacuate Server (evacuate Action) (microversion = 2.0)")]
pub struct ServerCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    evacuate: Evacuate,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.1/servers/{id}/action API
    ///
    #[arg(id = "path_param_id", value_name = "ID")]
    id: String,
}
/// Evacuate Body data
#[derive(Args)]
struct Evacuate {
    /// An administrative password to access the evacuated server. If you omit
    /// this parameter, the operation generates a new password. Up to API
    /// version 2.13, if `onSharedStorage` is set to `True` and this parameter
    /// is specified, an error is raised.
    ///
    #[arg(long)]
    admin_pass: Option<String>,

    /// The name or ID of the host to which the server is evacuated. If you
    /// omit this parameter, the scheduler chooses a host.
    ///
    /// Warning
    ///
    /// Prior to microversion 2.29, specifying a host will bypass validation by
    /// the scheduler, which could result in failures to actually evacuate the
    /// instance to the specified host, or over-subscription of the host. It is
    /// recommended to either not specify a host so that the scheduler will
    /// pick one, or specify a host with microversion >= 2.29 and without
    /// `force=True` set.
    ///
    #[arg(long)]
    host: Option<String>,

    /// Server on shared storage.
    ///
    /// Note
    ///
    /// Starting since version 2.14, Nova automatically detects whether the
    /// server is on shared storage or not. Therefore this parameter was
    /// removed.
    ///
    /// **Available until version 2.13**
    ///
    #[arg(action=clap::ArgAction::Set, long)]
    on_shared_storage: bool,
}

/// Server response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// An administrative password to access the evacuated instance. If you set
    /// `enable_instance_password` configuration option to `False`, the API
    /// wouldn’t return the `adminPass` field in response.
    ///
    /// **Available until version 2.13**
    ///
    #[serde(rename = "adminPass")]
    #[structable(title = "adminPass")]
    admin_pass: String,
}

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

        let mut ep_builder = evacuate_20::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.0");

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.evacuate data
        let args = &self.evacuate;
        let mut evacuate_builder = evacuate_20::EvacuateBuilder::default();
        if let Some(val) = &args.host {
            evacuate_builder.host(val);
        }

        evacuate_builder.on_shared_storage(args.on_shared_storage);

        if let Some(val) = &args.admin_pass {
            evacuate_builder.admin_pass(val);
        }

        ep_builder.evacuate(evacuate_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
