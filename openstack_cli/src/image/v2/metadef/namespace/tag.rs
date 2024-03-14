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

//! Glance Metadef namespace properties

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod delete_all;
mod list;
mod set;
mod show;

/// Metadata definition tags
///
/// Creates, lists, shows details for, updates, and deletes metadata definition tags.
#[derive(Parser)]
pub struct TagCommand {
    /// subcommand
    #[command(subcommand)]
    command: TagCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum TagCommands {
    Create(create::TagCommand),
    Delete(delete::TagCommand),
    List(list::TagsCommand),
    Purge(delete_all::TagCommand),
    Set(set::TagCommand),
    Show(show::TagCommand),
}

impl TagCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            TagCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            TagCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            TagCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            TagCommands::Purge(cmd) => cmd.take_action(parsed_args, session).await,
            TagCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            TagCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
