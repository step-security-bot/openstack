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

//! List Users command
//!
//! Wraps invoking of the `v3/groups/{group_id}/users` with `GET` method

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

use openstack_sdk::api::identity::v3::group::user::list;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::fmt;
use structable_derive::StructTable;

/// Lists the users that belong to a group.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/group_users`
///
#[derive(Args)]
#[command(about = "List users in group")]
pub struct UsersCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {
    /// Filter results based on which user passwords have expired. The query
    /// should include an operator and a timestamp with a colon (:) separating
    /// the two, for example: `password_expires_at={operator}:{timestamp}`.
    /// Valid operators are: `lt`, `lte`, `gt`, `gte`, `eq`, and `neq`. Valid
    /// timestamps are of the form: YYYY-MM-DDTHH:mm:ssZ.
    ///
    #[arg(long)]
    password_expires_at: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// group_id parameter for /v3/groups/{group_id}/users/{user_id} API
    ///
    #[arg(id = "path_param_group_id", value_name = "GROUP_ID")]
    group_id: String,
}
/// Users response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The ID of the default project for the user.
    ///
    #[serde()]
    #[structable(optional, wide)]
    default_project_id: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The ID of the domain.
    ///
    #[serde()]
    #[structable(optional, wide)]
    domain_id: Option<String>,

    /// If the user is enabled, this value is `true`. If the user is disabled,
    /// this value is `false`.
    ///
    #[serde()]
    #[structable(optional, wide)]
    enabled: Option<bool>,

    /// List of federated objects associated with a user. Each object in the
    /// list contains the `idp_id` and `protocols`. `protocols` is a list of
    /// objects, each of which contains `protocol_id` and `unique_id` of the
    /// protocol and user respectively. For example:
    ///
    /// ```text
    /// "federated": [
    ///   {
    ///     "idp_id": "efbab5a6acad4d108fec6c63d9609d83",
    ///     "protocols": [
    ///       {"protocol_id": "mapped", "unique_id": "test@example.com"}
    ///     ]
    ///   }
    /// ]
    ///
    /// ```
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    federated: Option<Value>,

    /// The user ID.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The user name. Must be unique within the owning domain.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The resource options for the user. Available resource options are
    /// `ignore_change_password_upon_first_use`, `ignore_password_expiry`,
    /// `ignore_lockout_failure_attempts`, `lock_password`,
    /// `multi_factor_auth_enabled`, and `multi_factor_auth_rules`
    /// `ignore_user_inactivity`.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    options: Option<Value>,

    /// The new password for the user.
    ///
    #[serde()]
    #[structable(optional, wide)]
    password: Option<String>,
}
/// `struct` response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseOptions {
    ignore_change_password_upon_first_use: Option<bool>,
    ignore_lockout_failure_attempts: Option<bool>,
    ignore_password_expiry: Option<bool>,
    ignore_user_inactivity: Option<bool>,
    lock_password: Option<bool>,
    multi_factor_auth_enabled: Option<bool>,
    multi_factor_auth_rules: Option<Value>,
}

impl fmt::Display for ResponseOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "ignore_change_password_upon_first_use={}",
                self.ignore_change_password_upon_first_use
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "ignore_lockout_failure_attempts={}",
                self.ignore_lockout_failure_attempts
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "ignore_password_expiry={}",
                self.ignore_password_expiry
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "ignore_user_inactivity={}",
                self.ignore_user_inactivity
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "lock_password={}",
                self.lock_password
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "multi_factor_auth_enabled={}",
                self.multi_factor_auth_enabled
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "multi_factor_auth_rules={}",
                self.multi_factor_auth_rules
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}

impl UsersCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Users");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.group_id(&self.path.group_id);
        // Set query parameters
        if let Some(val) = &self.query.password_expires_at {
            ep_builder.password_expires_at(val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
