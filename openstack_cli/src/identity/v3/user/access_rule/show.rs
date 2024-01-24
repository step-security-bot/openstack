//! Show details of an access rule.
//!
//! Relationship: `https://docs.openstack.org/api/openstack-
//! identity/3/rel/access\_rules`
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

use openstack_sdk::api::identity::v3::user::access_rule::get;
use openstack_sdk::api::QueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct AccessRuleArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id}
    /// API
    #[arg()]
    user_id: String,

    /// access_rule_id parameter for
    /// /v3/users/{user_id}/access_rules/{access_rule_id} API
    #[arg()]
    access_rule_id: String,
}

/// AccessRule show command
pub struct AccessRuleCmd {
    pub args: AccessRuleArgs,
}
/// AccessRule response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    #[serde()]
    #[structable(optional, wide)]
    path: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    method: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    service: Option<String>,

    #[serde()]
    #[structable(optional)]
    id: Option<String>,
}

#[async_trait]
impl Command for AccessRuleCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show AccessRule with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.user_id(&self.args.path.user_id);
        ep_builder.access_rule_id(&self.args.path.access_rule_id);
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
