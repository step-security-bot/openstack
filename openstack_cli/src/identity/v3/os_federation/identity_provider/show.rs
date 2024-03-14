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

//! Show IdentityProvider command
//!
//! Wraps invoking of the `v3/OS-FEDERATION/identity_providers/{idp_id}` with `GET` method

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

use openstack_sdk::api::identity::v3::os_federation::identity_provider::get;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Get registered identity providers.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/ext/OS-FEDERATION/1.0/rel/identity_provider`
///
#[derive(Args)]
#[command(about = "Get identity provider")]
pub struct IdentityProviderCommand {
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
    /// idp_id parameter for
    /// /v3/OS-FEDERATION/identity_providers/{idp_id}/protocols API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_idp_id",
        value_name = "IDP_ID"
    )]
    idp_id: String,
}
/// IdentityProvider response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The length of validity in minutes for group memberships carried over
    /// through mapping and persisted in the database. If left unset, the
    /// default value configured in keystone will be used, if enabled.
    ///
    #[serde()]
    #[structable(optional)]
    authorization_ttl: Option<i32>,

    /// The Identity Provider description
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of a domain that is associated with the Identity Provider.
    /// Federated users that authenticate with the Identity Provider will be
    /// created under the domain specified.
    ///
    #[serde()]
    #[structable(optional)]
    domain_id: Option<String>,

    /// Whether the Identity Provider is enabled or not
    ///
    #[serde()]
    #[structable(optional)]
    enabled: Option<bool>,

    /// The Identity Provider unique ID
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// List of the unique Identity Provider’s remote IDs
    ///
    #[serde()]
    #[structable(optional, pretty)]
    remote_ids: Option<Value>,
}

impl IdentityProviderCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show IdentityProvider");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.idp_id(&self.path.idp_id);
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
