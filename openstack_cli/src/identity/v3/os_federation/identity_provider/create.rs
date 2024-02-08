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

//! Create IdentityProvider command
//!
//! Wraps invoking of the `v3/OS-FEDERATION/identity_providers/{idp_id}` with `PUT` method

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

use openstack_sdk::api::identity::v3::os_federation::identity_provider::create;
use openstack_sdk::api::QueryAsync;
use std::fmt;
use structable_derive::StructTable;

/// Create an idp resource for federated authentication.
///
/// PUT /OS-FEDERATION/identity_providers/{idp_id}
#[derive(Args)]
pub struct IdentityProviderCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    identity_provider: IdentityProvider,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// idp_id parameter for /v3/OS-
    /// FEDERATION/identity_providers/{idp_id}/protocols API
    #[arg(id = "path_param_idp_id", value_name = "IDP_ID")]
    idp_id: String,
}
/// IdentityProvider Body data
#[derive(Args)]
struct IdentityProvider {
    /// If the user is enabled, this value is `true`.
    /// If the user is disabled, this value is `false`.
    #[arg(action=clap::ArgAction::Set, long)]
    enabled: Option<bool>,

    #[arg(long)]
    description: Option<String>,

    #[arg(long)]
    domain_id: Option<String>,

    #[arg(long)]
    authorization_ttl: Option<Option<i32>>,

    #[arg(action=clap::ArgAction::Append, long)]
    remote_ids: Option<Vec<String>>,
}

/// IdentityProvider response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The Identity Provider unique ID
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The Identity Provider description
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of a domain that is associated with the Identity Provider.
    #[serde()]
    #[structable(optional)]
    domain_id: Option<String>,

    /// The length of validity in minutes for group memberships carried over
    /// through mapping and persisted in the database.
    #[serde()]
    #[structable(optional)]
    authorization_ttl: Option<i32>,

    /// Whether the Identity Provider is enabled or not
    #[serde()]
    #[structable(optional)]
    enabled: Option<bool>,

    /// List of the unique Identity Provider’s remote IDs
    #[serde()]
    #[structable(optional)]
    remote_ids: Option<VecString>,
}
/// Vector of String response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct VecString(Vec<String>);
impl fmt::Display for VecString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl IdentityProviderCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create IdentityProvider");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        ep_builder.idp_id(&self.path.idp_id);
        // Set query parameters
        // Set body parameters
        // Set Request.identity_provider data
        let args = &self.identity_provider;
        let mut identity_provider_builder = create::IdentityProviderBuilder::default();
        if let Some(val) = &args.enabled {
            identity_provider_builder.enabled(*val);
        }

        if let Some(val) = &args.description {
            identity_provider_builder.description(Some(val.into()));
        }

        if let Some(val) = &args.domain_id {
            identity_provider_builder.domain_id(Some(val.into()));
        }

        if let Some(val) = &args.authorization_ttl {
            identity_provider_builder.authorization_ttl(*val);
        }

        if let Some(val) = &args.remote_ids {
            identity_provider_builder.remote_ids(val.iter().map(|v| v.into()).collect::<Vec<_>>());
        }

        ep_builder.identity_provider(identity_provider_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
