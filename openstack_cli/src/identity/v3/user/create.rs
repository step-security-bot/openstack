//! Creates a user.
//!
//! Relationship: `https://docs.openstack.org/api/openstack-
//! identity/3/rel/users`
//!
use async_trait::async_trait;
use bytes::Bytes;
use clap::Args;
use http::Response;
use http::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, Command};
use std::fmt;
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use crate::common::parse_json;
use crate::common::parse_key_val;
use openstack_sdk::api::identity::v3::user::create;
use openstack_sdk::api::RawQueryAsync;
use serde_json::Value;
use std::collections::HashMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct UserArgs {
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
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {}
/// Options Body data
#[derive(Args, Debug, Clone)]
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
#[derive(Args, Debug, Clone)]
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
    name: String,

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

/// User create command
pub struct UserCmd {
    pub args: UserArgs,
}
/// User response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {}

#[async_trait]
impl Command for UserCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create User with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.user data
        let args = &self.args.user;
        let mut user_builder = create::UserBuilder::default();
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
            let federated_builder: Vec<create::Federated> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<create::Federated>(v.clone()))
                .collect::<Vec<create::Federated>>();
            user_builder.federated(federated_builder);
        }

        user_builder.name(&args.name);

        if let Some(val) = &args.password {
            user_builder.password(Some(val.into()));
        }

        if let Some(val) = &args.options {
            let mut options_builder = create::OptionsBuilder::default();
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

        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = ResponseData {};
        // Maybe output some headers metadata
        op.output_human::<ResponseData>(&data)?;
        Ok(())
    }
}
