use async_trait::async_trait;
use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

use openstack_sdk::api::compute::v2::server::instance_action::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use std::fmt;
use structable_derive::StructTable;

/// Lists actions for a server.
///
/// Action information of deleted instances can be returned for requests
/// starting with microversion 2.21.
///
/// Policy defaults enable only users with the administrative role or the owner
/// of
/// the server to perform this operation. Cloud providers can change these
/// permissions
/// through the `policy.json` file.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404)
#[derive(Args, Clone, Debug)]
#[command(about = "List Actions For Server")]
pub struct InstanceActionsArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Total limit of entities count to return. Use this when there are too many entries.
    #[arg(long, default_value_t = 10000)]
    max_items: usize,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {
    #[arg(long)]
    limit: Option<i32>,

    #[arg(long)]
    marker: Option<String>,

    #[arg(long)]
    changes_since: Option<String>,

    #[arg(long)]
    changes_before: Option<String>,
}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// server_id parameter for /v2.1/servers/{server_id}/topology API
    #[arg(value_name = "SERVER_ID", id = "path_param_server_id")]
    server_id: String,
}

/// InstanceActions list command
pub struct InstanceActionsCmd {
    pub args: InstanceActionsArgs,
}
/// InstanceActions response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The name of the action.
    #[serde()]
    #[structable(optional)]
    action: Option<String>,

    /// The events which occurred in this action in descending order of
    /// creation.
    ///
    ///
    /// Policy defaults enable only users with the administrative role or the
    /// owner
    /// of the server to see instance action event information. Cloud providers
    /// can
    /// change these permissions through the `policy.json` file.
    ///
    ///
    /// **New in version 2.51**
    #[serde()]
    #[structable(optional)]
    events: Option<VecResponseEvents>,

    /// The related error message for when an action fails.
    #[serde()]
    #[structable(optional)]
    message: Option<String>,

    /// The ID of the project which initiated the server action.
    #[serde()]
    #[structable(optional)]
    project_id: Option<String>,

    /// The request id generated when execute the API of this action.
    #[serde()]
    #[structable(optional)]
    request_id: Option<String>,

    /// The date and time when the action was started.
    #[serde()]
    #[structable(optional)]
    start_time: Option<String>,

    /// The ID of the user which initiated the server action.
    #[serde()]
    #[structable(optional)]
    user_id: Option<String>,

    /// The date and time when the instance action or the action event of
    /// instance action was updated. The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm`
    /// value, if included, is the time zone as an offset from UTC. In
    /// the previous example, the offset value is `-05:00`.
    ///
    ///
    /// **New in version 2.58**
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseEvents {
    event: Option<String>,
    start_time: Option<String>,
    finish_time: Option<String>,
    result: Option<String>,
    traceback: Option<String>,
    host_id: Option<String>,
    host: Option<String>,
    details: Option<String>,
}

impl fmt::Display for ResponseEvents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "event={}",
                self.event
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "start_time={}",
                self.start_time
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "finish_time={}",
                self.finish_time
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "result={}",
                self.result
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "traceback={}",
                self.traceback
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "host_id={}",
                self.host_id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "host={}",
                self.host
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "details={}",
                self.details
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseEvents(Vec<ResponseEvents>);
impl fmt::Display for VecResponseEvents {
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

#[async_trait]
impl OSCCommand for InstanceActionsCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List InstanceActions with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.server_id(&self.args.path.server_id);
        // Set query parameters
        if let Some(val) = &self.args.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.args.query.marker {
            ep_builder.marker(val.clone());
        }
        if let Some(val) = &self.args.query.changes_since {
            ep_builder.changes_since(val.clone());
        }
        if let Some(val) = &self.args.query.changes_before {
            ep_builder.changes_before(val.clone());
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.args.max_items))
            .query_async(client)
            .await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
