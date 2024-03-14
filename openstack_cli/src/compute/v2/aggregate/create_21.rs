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

//! Create Aggregate command [microversion = 2.1]
//!
//! Wraps invoking of the `v2.1/os-aggregates` with `POST` method

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

use openstack_sdk::api::compute::v2::aggregate::create_21;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Creates an aggregate. If specifying an option availability_zone, the
/// aggregate is created as an availability zone and the availability zone is
/// visible to normal users.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// conflict(409)
///
#[derive(Args)]
#[command(about = "Create Aggregate (microversion = 2.1)")]
pub struct AggregateCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// The host aggregate object.
    ///
    #[command(flatten)]
    aggregate: Aggregate,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Aggregate Body data
#[derive(Args, Clone)]
struct Aggregate {
    /// The availability zone of the host aggregate. You should use a custom
    /// availability zone rather than the default returned by the
    /// os-availability-zone API. The availability zone must not include ‘:’ in
    /// its name.
    ///
    #[arg(help_heading = "Body parameters", long)]
    availability_zone: Option<String>,

    /// The name of the host aggregate.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: String,
}

/// Aggregate response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The availability zone of the host aggregate.
    ///
    #[serde()]
    #[structable(optional)]
    availability_zone: Option<String>,

    /// The date and time when the resource was created. The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm` value, if
    /// included, is the time zone as an offset from UTC. In the previous
    /// example, the offset value is `-05:00`.
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// A boolean indicates whether this aggregate is deleted or not, if it has
    /// not been deleted, `false` will appear.
    ///
    #[serde()]
    #[structable(optional)]
    deleted: Option<bool>,

    /// The date and time when the resource was deleted. If the resource has
    /// not been deleted yet, this field will be `null`, The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm` value, if
    /// included, is the time zone as an offset from UTC. In the previous
    /// example, the offset value is `-05:00`.
    ///
    #[serde()]
    #[structable(optional)]
    deleted_at: Option<String>,

    /// An array of host information.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    hosts: Option<Value>,

    /// The ID of the host aggregate.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<i32>,

    /// Metadata key and value pairs associated with the aggregate.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    metadata: Option<Value>,

    /// The date and time when the resource was updated, if the resource has
    /// not been updated, this field will show as `null`. The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm` value, if
    /// included, is the time zone as an offset from UTC. In the previous
    /// example, the offset value is `-05:00`.
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The UUID of the host aggregate.
    ///
    /// **New in version 2.41**
    ///
    #[serde()]
    #[structable(optional)]
    uuid: Option<String>,
}

impl AggregateCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Aggregate");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create_21::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.1");

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.aggregate data
        let args = &self.aggregate;
        let mut aggregate_builder = create_21::AggregateBuilder::default();

        aggregate_builder.name(&args.name);

        if let Some(val) = &args.availability_zone {
            aggregate_builder.availability_zone(Some(val.into()));
        }

        ep_builder.aggregate(aggregate_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
