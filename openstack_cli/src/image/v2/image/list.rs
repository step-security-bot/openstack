//! Lists public virtual machine (VM) images.
//! *(Since Image API v2.0)*
//!
//! **Pagination**
//!
//! Returns a subset of the larger collection of images and a link that you can
//! use
//! to get the next set of images. You should always check for the presence of
//! a
//! `next` link and use it as the URI in a subsequent HTTP GET request. You
//! should follow this pattern until a `next` link is no longer provided.
//!
//! The `next` link preserves any query parameters that you send in your
//! initial
//! request. You can use the `first` link to jump back to the first page of the
//! collection. If you prefer to paginate through images manually, use the
//! `limit` and `marker` parameters.
//!
//! **Query Filters**
//!
//! The list operation accepts query parameters to filter the response.
//!
//! A client can provide direct comparison filters by using most image
//! attributes,
//! such as `name=Ubuntu`, `visibility=public`, and so on.
//!
//! To filter using image tags, use the filter `tag` (note the singular). To
//! filter on multiple tags, include each tag separately in the query. For
//! example, to find images with the tag **ready**, include `tag=ready` in your
//! query string. To find images tagged with **ready** and **approved**,
//! include
//! `tag=ready&tag=approved` in your query string. (Note that only images
//! containing *both* tags will be included in the response.)
//!
//! A client cannot use any `link` in the json-schema, such as self, file, or
//! schema, to filter the response.
//!
//! You can list VM images that have a status of `active`, `queued`, or
//! `saving`.
//!
//! **The** `in` **Operator**
//!
//! As a convenience, you may specify several values for any of the following
//! fields by using the `in` operator:
//!
//! For most of these, usage is straight forward. For example, to list images
//! in queued or saving status, use:
//!
//! `GET /v2/images?status=in:saving,queued`
//!
//! To find images in a particular list of image IDs, use:
//!
//! `GET /v2/images?id=in:3afb79c1-131a-4c38-a87c-bc4b801d14e6,2e011209-660f-
//! 44b5-baf2-2eb4babae53d`
//!
//! Using the `in` operator with the `name` property of images can be a bit
//! trickier, depending upon how creatively you have named your images. The
//! general rule is that if an image name contains a comma (`,`), you must
//! enclose the entire name in quotation marks (`"`). As usual, you must URL
//! encode any characters that require it.
//!
//! For example, to find images named `glass, darkly` or `share me`, you would
//! use the following filter specification:
//!
//! `GET v2/images?name=in:"glass,%20darkly",share%20me`
//!
//! As with regular filtering by name, you must specify the complete name you
//! are
//! looking for. Thus, for example, the query string `name=in:glass,share` will
//! only match images with the exact name `glass` or the exact name `share`.
//! It will not find an image named `glass, darkly` or an image named `share
//! me`.
//!
//! **Size Comparison Filters**
//!
//! You can use the `size\_min` and `size\_max` query parameters to filter
//! images
//! that are greater than or less than the image size. The size, in bytes, is
//! the
//! size of an image on disk.
//!
//! For example, to filter the container to include only images that are from 1
//! to
//! 4 MB, set the `size\_min` query parameter to `1048576` and the `size\_max`
//! query parameter to `4194304`.
//!
//! **Time Comparison Filters**
//!
//! You can use a *comparison operator* along with the `created\_at` or
//! `updated\_at` fields to filter your results. Specify the operator first, a
//! colon (`:`) as a separator, and then the time in [ISO 8601
//! Format](https://en.wikipedia.org/wiki/ISO_8601). Available comparison
//! operators
//! are:
//!
//! For example:
//!
//! **Sorting**
//!
//! You can use query parameters to sort the results of this operation.
//!
//! To sort the response, use the `sort\_key` and `sort\_dir` query
//! parameters:
//!
//! Alternatively, specify the `sort` query parameter:
//!
//! Normal response codes: 200
//!
//! Error response codes: 400, 401, 403
//!
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
use structable_derive::StructTable;

use openstack_sdk::AsyncOpenStack;

use openstack_sdk::api::image::v2::image::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;

use std::collections::HashMap;
use std::fmt;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct ImagesArgs {
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
    /// Requests a page size of items. Returns a number of items up to a limit
    /// value. Use the limit parameter to make an initial limited request and
    /// use the ID of the last-seen item from the response as the marker
    /// parameter value in a subsequent limited request.
    #[arg(long)]
    limit: Option<i32>,

    /// The ID of the last-seen item. Use the limit parameter to make an
    /// initial limited request and use the ID of the last-seen item from the
    /// response as the marker parameter value in a subsequent limited request.
    #[arg(long)]
    marker: Option<String>,

    /// Filters the response by a name, as a string. A valid value is the name
    /// of an image.
    #[arg(long)]
    name: Option<String>,

    /// id filter parameter
    #[arg(long)]
    id: Option<String>,

    /// Filters the response by a project (also called a “tenant”) ID. Shows
    /// only images that are shared with you by the specified owner.
    #[arg(long)]
    owner: Option<String>,

    /// Filters the response by the ‘protected’ image property. A valid value
    /// is one of ‘true’, ‘false’ (must be all lowercase). Any other value will
    /// result in a 400 response.
    #[arg(long)]
    protected: Option<bool>,

    /// Filters the response by an image status.
    #[arg(long)]
    status: Option<String>,

    /// Filters the response by the specified tag value. May be repeated, but
    /// keep in mind that you're making a conjunctive query, so only images
    /// containing all the tags specified will appear in the response.
    #[arg(long)]
    tag: Option<Vec<String>>,

    /// Filters the response by an image visibility value. A valid value is
    /// public, private, community, shared, or all. (Note that if you filter on
    /// shared, the images included in the response will only be those where
    /// your member status is accepted unless you explicitly include a
    /// member_status filter in the request.) If you omit this parameter, the
    /// response shows public, private, and those shared images with a member
    /// status of accepted.
    #[arg(long)]
    visibility: Option<String>,

    /// When true, filters the response to display only "hidden" images. By
    /// default, "hidden" images are not included in the image-list response.
    /// (Since Image API v2.7)
    #[arg(long)]
    os_hidden: Option<bool>,

    /// Filters the response by a member status. A valid value is accepted,
    /// pending, rejected, or all. Default is accepted.
    #[arg(long)]
    member_status: Option<String>,

    /// Filters the response by a maximum image size, in bytes.
    #[arg(long)]
    size_max: Option<String>,

    /// Filters the response by a minimum image size, in bytes.
    #[arg(long)]
    size_min: Option<String>,

    /// Specify a comparison filter based on the date and time when the
    /// resource was created.
    #[arg(long)]
    created_at: Option<String>,

    /// Specify a comparison filter based on the date and time when the
    /// resource was most recently modified.
    #[arg(long)]
    updated_at: Option<String>,

    /// Sorts the response by a set of one or more sort direction and attribute
    /// (sort_key) combinations. A valid value for the sort direction is asc
    /// (ascending) or desc (descending). If you omit the sort direction in a
    /// set, the default is desc.
    #[arg(long)]
    sort_dir: Option<String>,

    /// Sorts the response by an attribute, such as name, id, or updated_at.
    /// Default is created_at. The API uses the natural sorting direction of
    /// the sort_key image attribute.
    #[arg(long)]
    sort_key: Option<String>,

    /// Sorts the response by one or more attribute and sort direction
    /// combinations. You can also set multiple sort keys and directions.
    /// Default direction is desc. Use the comma (,) character to separate
    /// multiple values. For example: `sort=name:asc,status:desc`
    #[arg(long)]
    sort: Option<String>,
}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {}

/// Images list command
pub struct ImagesCmd {
    pub args: ImagesArgs,
}
/// Images response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// An identifier for the image
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Descriptive name for the image
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// Status of the image
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// Scope of image accessibility
    #[serde()]
    #[structable(optional, wide)]
    visibility: Option<String>,

    /// If true, image will not be deletable.
    #[serde()]
    #[structable(optional, wide)]
    protected: Option<bool>,

    /// If true, image will not appear in default image list response.
    #[serde()]
    #[structable(optional, wide)]
    os_hidden: Option<bool>,

    /// md5 hash of image contents.
    #[serde()]
    #[structable(optional, wide)]
    checksum: Option<String>,

    /// Algorithm to calculate the os_hash_value
    #[serde()]
    #[structable(optional, wide)]
    os_hash_algo: Option<String>,

    /// Hexdigest of the image contents using the algorithm specified by the
    /// os_hash_algo
    #[serde()]
    #[structable(optional, wide)]
    os_hash_value: Option<String>,

    /// Owner of the image
    #[serde()]
    #[structable(optional, wide)]
    owner: Option<String>,

    /// Size of image file in bytes
    #[serde()]
    #[structable(optional, wide)]
    size: Option<i64>,

    /// Virtual size of image in bytes
    #[serde()]
    #[structable(optional, wide)]
    virtual_size: Option<i64>,

    /// Format of the container
    #[serde()]
    #[structable(optional, wide)]
    container_format: Option<String>,

    /// Format of the disk
    #[serde()]
    #[structable(optional, wide)]
    disk_format: Option<String>,

    /// Date and time of image registration
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// Date and time of the last image modification
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// List of strings related to the image
    #[serde()]
    #[structable(optional, wide)]
    tags: Option<VecString>,

    /// URL to access the image file kept in external store
    #[serde()]
    #[structable(optional, wide)]
    direct_url: Option<String>,

    /// Amount of ram (in MB) required to boot image.
    #[serde()]
    #[structable(optional, wide)]
    min_ram: Option<i32>,

    /// Amount of disk space (in GB) required to boot image.
    #[serde()]
    #[structable(optional, wide)]
    min_disk: Option<i32>,

    /// An image self url
    #[serde(rename = "self")]
    #[structable(optional, title = "self", wide)]
    _self: Option<String>,

    /// An image file url
    #[serde()]
    #[structable(optional, wide)]
    file: Option<String>,

    /// Store in which image data resides.  Only present when the operator has
    /// enabled multiple stores.  May be a comma-separated list of store
    /// identifiers.
    #[serde()]
    #[structable(optional, wide)]
    stores: Option<String>,

    /// An image schema url
    #[serde()]
    #[structable(optional, wide)]
    schema: Option<String>,

    /// A set of URLs to access the image file kept in external store
    #[serde()]
    #[structable(optional, wide)]
    locations: Option<VecResponseLocations>,
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
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct HashMapStringValue(HashMap<String, Value>);
impl fmt::Display for HashMapStringValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|v| format!("{}={}", v.0, v.1))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseValidationData {
    checksum: Option<String>,
    os_hash_algo: String,
    os_hash_value: String,
}

impl fmt::Display for ResponseValidationData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "checksum={}",
                self.checksum
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!("os_hash_algo={}", self.os_hash_algo),
            format!("os_hash_value={}", self.os_hash_value),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseLocations {
    url: String,
    metadata: HashMapStringValue,
    validation_data: Option<ResponseValidationData>,
}

impl fmt::Display for ResponseLocations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!("url={}", self.url),
            format!("metadata={}", self.metadata),
            format!(
                "validation_data={}",
                self.validation_data
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseLocations(Vec<ResponseLocations>);
impl fmt::Display for VecResponseLocations {
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
impl OSCCommand for ImagesCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Images with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.args.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.args.query.marker {
            ep_builder.marker(val.clone());
        }
        if let Some(val) = &self.args.query.name {
            ep_builder.name(val.clone());
        }
        if let Some(val) = &self.args.query.id {
            ep_builder.id(val.clone());
        }
        if let Some(val) = &self.args.query.owner {
            ep_builder.owner(val.clone());
        }
        if let Some(val) = &self.args.query.protected {
            ep_builder.protected(*val);
        }
        if let Some(val) = &self.args.query.status {
            ep_builder.status(val.clone());
        }
        if let Some(val) = &self.args.query.tag {
            ep_builder.tag(val.iter());
        }
        if let Some(val) = &self.args.query.visibility {
            ep_builder.visibility(val.clone());
        }
        if let Some(val) = &self.args.query.os_hidden {
            ep_builder.os_hidden(*val);
        }
        if let Some(val) = &self.args.query.member_status {
            ep_builder.member_status(val.clone());
        }
        if let Some(val) = &self.args.query.size_max {
            ep_builder.size_max(val.clone());
        }
        if let Some(val) = &self.args.query.size_min {
            ep_builder.size_min(val.clone());
        }
        if let Some(val) = &self.args.query.created_at {
            ep_builder.created_at(val.clone());
        }
        if let Some(val) = &self.args.query.updated_at {
            ep_builder.updated_at(val.clone());
        }
        if let Some(val) = &self.args.query.sort_dir {
            ep_builder.sort_dir(val.clone());
        }
        if let Some(val) = &self.args.query.sort_key {
            ep_builder.sort_key(val.clone());
        }
        if let Some(val) = &self.args.query.sort {
            ep_builder.sort(val.clone());
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
