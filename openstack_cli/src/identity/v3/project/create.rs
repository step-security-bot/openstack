//! Creates a project, where the project may act as a domain.
//!
//! Relationship: `https://docs.openstack.org/api/openstack-
//! identity/3/rel/projects`
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
use openstack_sdk::api::identity::v3::project::create;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::collections::BTreeMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct ProjectArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    project: Project,
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
    immutable: Option<bool>,
}

/// Project Body data
#[derive(Args, Debug, Clone)]
struct Project {
    /// The description of the project.
    #[arg(long)]
    description: Option<String>,

    /// The ID of the domain for the project.
    ///
    ///
    /// For projects acting as a domain, the `domain\_id` must not be
    /// specified,
    /// it will be generated by the Identity service implementation.
    ///
    ///
    /// For regular projects (i.e. those not acing as a domain), if
    /// `domain\_id`
    /// is not specified, but `parent\_id` is specified, then the domain ID of
    /// the
    /// parent will be used. If neither `domain\_id` or `parent\_id` is
    /// specified, the Identity service implementation will default to the
    /// domain
    /// to which the client’s token is scoped. If both `domain\_id` and
    /// `parent\_id` are specified, and they do not indicate the same domain,
    /// an
    /// `Bad Request (400)` will be returned.
    #[arg(long)]
    domain_id: Option<String>,

    /// If set to `true`, project is enabled. If set to
    /// `false`, project is disabled. The default is `true`.
    #[arg(action=clap::ArgAction::Set, long)]
    enabled: Option<bool>,

    /// If set to `true`, project is enabled. If set to
    /// `false`, project is disabled. The default is `true`.
    #[arg(action=clap::ArgAction::Set, long)]
    is_domain: Option<bool>,

    /// The ID of the parent of the project.
    ///
    ///
    /// If specified on project creation, this places the project within a
    /// hierarchy and implicitly defines the owning domain, which will be the
    /// same domain as the parent specified. If `parent\_id` is
    /// not specified and `is\_domain` is `false`, then the project will use
    /// its
    /// owning domain as its parent. If `is\_domain` is `true` (i.e. the
    /// project
    /// is acting as a domain), then `parent\_id` must not specified (or if it
    /// is,
    /// it must be `null`) since domains have no parents.
    ///
    ///
    /// `parent\_id` is immutable, and can’t be updated after the project is
    /// created - hence a project cannot be moved within the hierarchy.
    ///
    ///
    /// **New in version 3.4**
    #[arg(long)]
    parent_id: Option<String>,

    /// The name of the project, which must be unique within the
    /// owning domain. A project can have the same name as its domain.
    #[arg(long)]
    name: String,

    /// A list of simple strings assigned to a project.
    /// Tags can be used to classify projects into groups.
    #[arg(action=clap::ArgAction::Append, long)]
    tags: Option<Vec<String>>,

    /// The resource options for the project. Available resource options are
    /// `immutable`.
    #[command(flatten)]
    options: Option<Options>,
}

/// Project create command
pub struct ProjectCmd {
    pub args: ProjectArgs,
}
/// Project response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The ID for the project.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The description of the project.
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The ID of the domain for the project.
    #[serde()]
    #[structable(optional, wide)]
    domain_id: Option<String>,

    /// If the user is enabled, this value is `true`.
    /// If the user is disabled, this value is `false`.
    #[serde()]
    #[structable(optional, wide)]
    enabled: Option<bool>,

    /// If the user is enabled, this value is `true`.
    /// If the user is disabled, this value is `false`.
    #[serde()]
    #[structable(optional, wide)]
    is_domain: Option<bool>,

    /// The ID of the parent for the project.
    ///
    ///
    /// **New in version 3.4**
    #[serde()]
    #[structable(optional, wide)]
    parent_id: Option<String>,

    /// The name of the project.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// A list of simple strings assigned to a project.
    #[serde()]
    #[structable(optional, wide)]
    tags: Option<VecString>,

    /// The resource options for the project. Available resource options are
    /// `immutable`.
    #[serde()]
    #[structable(optional, wide)]
    options: Option<ResponseOptions>,
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecString(Vec<String>);
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
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
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

#[async_trait]
impl Command for ProjectCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Project with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.project data
        let args = &self.args.project;
        let mut project_builder = create::ProjectBuilder::default();
        if let Some(val) = &args.description {
            project_builder.description(Some(val.into()));
        }

        if let Some(val) = &args.domain_id {
            project_builder.domain_id(Some(val.into()));
        }

        if let Some(val) = &args.enabled {
            project_builder.enabled(*val);
        }

        if let Some(val) = &args.is_domain {
            project_builder.is_domain(*val);
        }

        if let Some(val) = &args.parent_id {
            project_builder.parent_id(Some(val.into()));
        }

        project_builder.name(&args.name);

        if let Some(val) = &args.tags {
            project_builder.tags(val.iter().map(|v| v.into()).collect::<Vec<_>>());
        }

        if let Some(val) = &args.options {
            let sub = create::OptionsBuilder::default();
            project_builder.options(sub.build().expect("A valid object"));
        }

        ep_builder.project(project_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
