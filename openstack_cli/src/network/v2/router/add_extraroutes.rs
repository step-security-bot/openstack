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

//! Action Router command
//!
//! Wraps invoking of the `v2.0/routers/{id}/add_extraroutes` with `PUT` method

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

use crate::common::parse_json;
use openstack_sdk::api::network::v2::router::add_extraroutes;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Request body
///
#[derive(Args)]
#[command(about = "Add extra routes to router")]
pub struct RouterCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    router: Router,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.0/routers/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Router Body data
#[derive(Args, Clone)]
struct Router {
    /// The extra routes configuration for L3 router. A list of dictionaries
    /// with `destination` and `nexthop` parameters. It is available when
    /// `extraroute` extension is enabled.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long, value_name="JSON", value_parser=parse_json)]
    routes: Option<Vec<Value>>,
}

/// Router response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The ID of the router.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The name of the router.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The extra routes configuration for L3 router. A list of dictionaries
    /// with `destination` and `nexthop` parameters. It is available when
    /// `extraroute` extension is enabled.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    routes: Option<Value>,
}

impl RouterCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Router");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = add_extraroutes::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.router data
        let args = &self.router;
        let mut router_builder = add_extraroutes::RouterBuilder::default();
        if let Some(val) = &args.routes {
            let routes_builder: Vec<add_extraroutes::Routes> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<add_extraroutes::Routes>(v.to_owned()))
                .collect::<Vec<add_extraroutes::Routes>>();
            router_builder.routes(routes_builder);
        }

        ep_builder.router(router_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
