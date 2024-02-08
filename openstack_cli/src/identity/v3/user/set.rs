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
use std::fmt;
use structable_derive::StructTable;

/// Updates a user.
///
/// If the back-end driver does not support this functionality, this
/// call might return the HTTP `Not Implemented (501)` response code.
///
/// Relationship: `https://docs.openstack.org/api/openstack-
/// identity/3/rel/user`
#[derive(Args)]
#[command(about = "Update user")]
pub struct UserCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

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
    #[arg(id = "path_param_id", value_name = "ID")]
    id: String,
}
/// Options Body data
#[derive(Args)]
#[group(required = false, multiple = true)]
struct Options {
    #[arg(action=clap::ArgAction::Set, long)]
    ignore_change_password_upon_first_use: Option<bool>,

    #[arg(action=clap::ArgAction::Set, long)]
    ignore_password_expiry: Option<bool>,

    #[arg(action=clap::ArgAction::Set, long)]
    ignore_lockout_failure_attempts: Option<bool>,

    #[arg(action=clap::ArgAction::Set, long)]
    lock_password: Option<bool>,

    #[arg(action=clap::ArgAction::Set, long)]
    ignore_user_inactivity: Option<bool>,

    #[arg(action=clap::ArgAction::Append, long)]
    multi_factor_auth_rules: Option<Vec<String>>,

    #[arg(action=clap::ArgAction::Set, long)]
    multi_factor_auth_enabled: Option<bool>,
}

/// User Body data
#[derive(Args)]
struct User {
    /// The ID of the default project for the user.
    #[arg(long)]
    default_project_id: Option<String>,

    /// The new description of the group.
    #[arg(long)]
    description: Option<String>,

    /// The ID of the domain.
    #[arg(long)]
    domain_id: Option<String>,

    /// If the user is enabled, this value is `true`.
    /// If the user is disabled, this value is `false`.
    #[arg(action=clap::ArgAction::Set, long)]
    enabled: Option<bool>,

    /// List of federated objects associated with a user. Each object in the
    /// list
    /// contains the `idp\_id` and `protocols`. `protocols` is a list of
    /// objects, each of which contains `protocol\_id` and `unique\_id` of
    /// the protocol and user respectively. For example:
    ///
    ///
    ///
    /// ```text
    /// "federated": [
    ///   {
    ///     "idp\_id": "efbab5a6acad4d108fec6c63d9609d83",
    ///     "protocols": [
    ///       {"protocol\_id": "mapped", "unique\_id": "test@example.com"}
    ///     ]
    ///   }
    /// ]
    ///
    /// ```
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    federated: Option<Vec<Value>>,

    /// The user name. Must be unique within the owning domain.
    #[arg(long)]
    name: Option<String>,

    /// The new password for the user.
    #[arg(long)]
    password: Option<String>,

    /// The resource options for the user. Available resource options are
    /// `ignore\_change\_password\_upon\_first\_use`,
    /// `ignore\_password\_expiry`,
    /// `ignore\_lockout\_failure\_attempts`, `lock\_password`,
    /// `multi\_factor\_auth\_enabled`, and `multi\_factor\_auth\_rules`
    /// `ignore\_user\_inactivity`.
    #[command(flatten)]
    options: Option<Options>,
}

/// User response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The user ID.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The ID of the default project for the user.
    #[serde()]
    #[structable(optional)]
    default_project_id: Option<String>,

    /// The new description of the group.
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of the domain.
    #[serde()]
    #[structable(optional)]
    domain_id: Option<String>,

    /// If the user is enabled, this value is `true`.
    /// If the user is disabled, this value is `false`.
    #[serde()]
    #[structable(optional)]
    enabled: Option<bool>,

    /// List of federated objects associated with a user. Each object in the
    /// list
    /// contains the `idp\_id` and `protocols`. `protocols` is a list of
    /// objects, each of which contains `protocol\_id` and `unique\_id` of
    /// the protocol and user respectively. For example:
    ///
    ///
    ///
    /// ```text
    /// "federated": [
    ///   {
    ///     "idp\_id": "efbab5a6acad4d108fec6c63d9609d83",
    ///     "protocols": [
    ///       {"protocol\_id": "mapped", "unique\_id": "test@example.com"}
    ///     ]
    ///   }
    /// ]
    ///
    /// ```
    #[serde()]
    #[structable(optional)]
    federated: Option<Value>,

    /// The user name. Must be unique within the owning domain.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The new password for the user.
    #[serde()]
    #[structable(optional)]
    password: Option<String>,

    /// The resource options for the user. Available resource options are
    /// `ignore\_change\_password\_upon\_first\_use`,
    /// `ignore\_password\_expiry`,
    /// `ignore\_lockout\_failure\_attempts`, `lock\_password`,
    /// `multi\_factor\_auth\_enabled`, and `multi\_factor\_auth\_rules`
    /// `ignore\_user\_inactivity`.
    #[serde()]
    #[structable(optional)]
    options: Option<ResponseOptions>,
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
/// Vector of VecString response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct VecVecString(Vec<VecString>);
impl fmt::Display for VecVecString {
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
/// struct response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseOptions {
    ignore_change_password_upon_first_use: Option<bool>,
    ignore_password_expiry: Option<bool>,
    ignore_lockout_failure_attempts: Option<bool>,
    lock_password: Option<bool>,
    ignore_user_inactivity: Option<bool>,
    multi_factor_auth_rules: Option<VecVecString>,
    multi_factor_auth_enabled: Option<bool>,
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
                "ignore_password_expiry={}",
                self.ignore_password_expiry
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
                "lock_password={}",
                self.lock_password
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
                "multi_factor_auth_rules={}",
                self.multi_factor_auth_rules
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "multi_factor_auth_enabled={}",
                self.multi_factor_auth_enabled
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
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
            user_builder.domain_id(val.clone());
        }

        if let Some(val) = &args.enabled {
            user_builder.enabled(*val);
        }

        if let Some(val) = &args.federated {
            let federated_builder: Vec<set::Federated> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<set::Federated>(v.clone()))
                .collect::<Vec<set::Federated>>();
            user_builder.federated(federated_builder);
        }

        if let Some(val) = &args.name {
            user_builder.name(val.clone());
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
