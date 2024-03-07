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

//! List Catalogs command
//!
//! Wraps invoking of the `v3/auth/catalog` with `GET` method

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

use openstack_sdk::api::identity::v3::auth::catalog::list;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// New in version 3.3
///
/// This call returns a service catalog for the X-Auth-Token provided in the
/// request, even if the token does not contain a catalog itself (for example,
/// if it was generated using ?nocatalog).
///
/// The structure of the catalog object is identical to that contained in a
/// token.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/auth_catalog`
///
#[derive(Args)]
#[command(about = "Get service catalog")]
pub struct CatalogsCommand {
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
struct PathParameters {}
/// Catalogs response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// A list of `endpoint` objects.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    endpoints: Option<Value>,

    /// The UUID of the service to which the endpoint belongs.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The service name.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The service type, which describes the API implemented by the service.
    /// Value is `compute`, `ec2`, `identity`, `image`, `network`, or `volume`.
    ///
    #[serde(rename = "type")]
    #[structable(optional, title = "type", wide)]
    _type: Option<String>,
}

impl CatalogsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Catalogs");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
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
