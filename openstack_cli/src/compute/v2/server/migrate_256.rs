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

//! Action Server command [microversion = 2.56]
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

use bytes::Bytes;
use http::Response;
use openstack_sdk::api::compute::v2::server::migrate_256;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Migrates a server to a host.
///
/// Specify the `migrate` action in the request body.
///
/// Up to microversion 2.55, the scheduler chooses the host. Starting from
/// microversion 2.56, the `host` parameter is available to specify the
/// destination host. If you specify `null` or don’t specify this parameter,
/// the scheduler chooses a host.
///
/// **Asynchronous Postconditions**
///
/// A successfully migrated server shows a `VERIFY_RESIZE` status and
/// `finished` migration status. If the cloud has configured the
/// [resize_confirm_window](https://docs.openstack.org/nova/latest/configuration/config.html#DEFAULT.resize_confirm_window)
/// option of the Compute service to a positive value, the Compute service
/// automatically confirms the migrate operation after the configured interval.
///
/// There are two different policies for this action, depending on whether the
/// host parameter is set. Both defaults enable only users with the
/// administrative role to perform this operation. Cloud providers can change
/// these permissions through the `policy.json` file.
///
/// Normal response codes: 202
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403)
/// itemNotFound(404), conflict(409)
///
#[derive(Args)]
#[command(about = "Migrate Server (migrate Action) (microversion = 2.56)")]
pub struct ServerCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// The action to cold migrate a server. This parameter can be `null`. Up
    /// to microversion 2.55, this parameter should be `null`.
    ///
    #[command(flatten)]
    migrate: Option<Migrate>,
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
/// Migrate Body data
#[derive(Args, Clone)]
struct Migrate {
    #[arg(help_heading = "Body parameters", long)]
    host: Option<String>,
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

        let mut ep_builder = migrate_256::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.56");

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.migrate data
        if let Some(lmigrate) = &self.migrate {
            let mut migrate_builder = migrate_256::MigrateBuilder::default();
            if let Some(val) = &lmigrate.host {
                migrate_builder.host(Some(val.into()));
            }
            ep_builder.migrate(migrate_builder.build().expect("A valid object"));
        }

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
