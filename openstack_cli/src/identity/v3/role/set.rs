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

//! Set Role command
//!
//! Wraps invoking of the `v3/roles/{role_id}` with `PATCH` method

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
use openstack_sdk::api::find;
use openstack_sdk::api::identity::v3::role::find;
use openstack_sdk::api::identity::v3::role::set;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use structable_derive::StructTable;

/// Updates a role.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/role`
///
#[derive(Args)]
#[command(about = "Update role")]
pub struct RoleCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(long)]
    name: Option<String>,
    #[arg(long)]
    description: Option<String>,
    #[command(flatten)]
    options: Option<Options>,
    /// Additional properties to be sent with the request
    #[arg(long="property", value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    properties: Option<Vec<(String, Value)>>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// role_id parameter for /v3/roles/{role_id} API
    ///
    #[arg(id = "path_param_id", value_name = "ID")]
    id: String,
}
/// Options Body data
#[derive(Args)]
struct Options {
    #[arg(action=clap::ArgAction::Set, long)]
    immutable: Option<bool>,
}

/// Role response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The role ID.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The link to the resources in question.
    ///
    #[serde()]
    #[structable(optional)]
    links: Option<HashMapStringOptionString>,

    /// The role name.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The role description.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The resource options for the role. Available resource options are
    /// `immutable`.
    ///
    #[serde()]
    #[structable(optional)]
    options: Option<ResponseOptions>,
}
/// HashMap of `Option<String>` response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct HashMapStringOptionString(HashMap<String, Option<String>>);
impl fmt::Display for HashMapStringOptionString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|v| format!("{}={}", v.0, v.1.clone().unwrap_or("".to_string())))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
/// `struct` response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseOptions {
    immutable: Option<bool>,
}

impl fmt::Display for ResponseOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([format!(
            "immutable={}",
            self.immutable
                .map(|v| v.to_string())
                .unwrap_or("".to_string())
        )]);
        write!(f, "{}", data.join(";"))
    }
}

impl RoleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Role");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;
        let mut ep_builder = set::Request::builder();

        // Set path parameters
        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();
        ep_builder.id(resource_id.clone());
        // Set query parameters
        // Set body parameters
        // Set Request.name data
        if let Some(args) = &self.name {
            ep_builder.name(args.clone());
        }

        // Set Request.description data
        if let Some(args) = &self.description {
            ep_builder.description(args.clone());
        }

        // Set Request.options data
        if let Some(args) = &self.options {
            let mut options_builder = set::OptionsBuilder::default();
            if let Some(val) = &args.immutable {
                options_builder.immutable(*val);
            }

            ep_builder.options(options_builder.build().unwrap());
        }

        if let Some(properties) = &self.properties {
            ep_builder.properties(properties.iter().cloned());
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
