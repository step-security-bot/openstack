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

//! Set User command
//!
//! Wraps invoking of the `v3/users/{user_id}` with `PATCH` method

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

use openstack_sdk::api::find;
use openstack_sdk::api::identity::v3::user::find;
use openstack_sdk::api::identity::v3::user::set;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Updates a user.
///
/// If the back-end driver does not support this functionality, this call might
/// return the HTTP `Not Implemented (501)` response code.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/user`
///
#[derive(Args)]
#[command(about = "Update user")]
pub struct UserCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `user` object
    ///
    #[command(flatten)]
    user: User,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id}
    /// API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Options Body data
#[derive(Args, Clone)]
#[group(required = false, multiple = true)]
struct Options {
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    ignore_change_password_upon_first_use: Option<bool>,

    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    ignore_lockout_failure_attempts: Option<bool>,

    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    ignore_password_expiry: Option<bool>,

    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    ignore_user_inactivity: Option<bool>,

    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    lock_password: Option<bool>,

    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    multi_factor_auth_enabled: Option<bool>,

    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    multi_factor_auth_rules: Option<Vec<String>>,
}

/// User Body data
#[derive(Args, Clone)]
struct User {
    /// The ID of the default project for the user.
    ///
    #[arg(help_heading = "Body parameters", long)]
    default_project_id: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// The ID of the domain.
    ///
    #[arg(help_heading = "Body parameters", long)]
    domain_id: Option<String>,

    /// Whether the Service Provider is enabled or not
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
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
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long, value_name="JSON", value_parser=parse_json)]
    federated: Option<Vec<Value>>,

    /// The user name. Must be unique within the owning domain.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// The resource options for the user. Available resource options are
    /// `ignore_change_password_upon_first_use`, `ignore_password_expiry`,
    /// `ignore_lockout_failure_attempts`, `lock_password`,
    /// `multi_factor_auth_enabled`, and `multi_factor_auth_rules`
    /// `ignore_user_inactivity`.
    ///
    #[command(flatten)]
    options: Option<Options>,

    /// The new password for the user.
    ///
    #[arg(help_heading = "Body parameters", long)]
    password: Option<String>,
}

/// User response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The ID of the default project for the user.
    ///
    #[serde()]
    #[structable(optional)]
    default_project_id: Option<String>,

    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of the domain.
    ///
    #[serde()]
    #[structable(optional)]
    domain_id: Option<String>,

    /// Whether the Service Provider is enabled or not
    ///
    #[serde()]
    #[structable(optional)]
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
    #[structable(optional, pretty)]
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
    #[structable(optional, pretty)]
    options: Option<Value>,

    /// The new password for the user.
    ///
    #[serde()]
    #[structable(optional)]
    password: Option<String>,
}

impl UserCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set User");

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
        // Set Request.user data
        let args = &self.user;
        let mut user_builder = set::UserBuilder::default();
        if let Some(val) = &args.default_project_id {
            user_builder.default_project_id(Some(val.into()));
        }

        if let Some(val) = &args.description {
            user_builder.description(Some(val.into()));
        }

        if let Some(val) = &args.domain_id {
            user_builder.domain_id(val);
        }

        if let Some(val) = &args.enabled {
            user_builder.enabled(*val);
        }

        if let Some(val) = &args.federated {
            let federated_builder: Vec<set::Federated> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<set::Federated>(v.to_owned()))
                .collect::<Vec<set::Federated>>();
            user_builder.federated(federated_builder);
        }

        if let Some(val) = &args.name {
            user_builder.name(val);
        }

        if let Some(val) = &args.password {
            user_builder.password(Some(val.into()));
        }

        if let Some(val) = &args.options {
            let mut options_builder = set::OptionsBuilder::default();
            if let Some(val) = &val.ignore_change_password_upon_first_use {
                options_builder.ignore_change_password_upon_first_use(*val);
            }
            if let Some(val) = &val.ignore_password_expiry {
                options_builder.ignore_password_expiry(*val);
            }
            if let Some(val) = &val.ignore_lockout_failure_attempts {
                options_builder.ignore_lockout_failure_attempts(*val);
            }
            if let Some(val) = &val.lock_password {
                options_builder.lock_password(*val);
            }
            if let Some(val) = &val.ignore_user_inactivity {
                options_builder.ignore_user_inactivity(*val);
            }
            if let Some(val) = &val.multi_factor_auth_rules {
                options_builder.multi_factor_auth_rules(
                    val.iter()
                        .cloned()
                        .map(|x| Vec::from([x.split(',').collect()]))
                        .collect::<Vec<_>>(),
                );
            }
            if let Some(val) = &val.multi_factor_auth_enabled {
                options_builder.multi_factor_auth_enabled(*val);
            }
            user_builder.options(options_builder.build().expect("A valid object"));
        }

        ep_builder.user(user_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
