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

//! Block storage Message commands
//!

use clap::{Parser, Subcommand};

use crate::{Cli, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod delete;
mod list;
mod show;

/// Messages (messages)
///
/// Lists all, shows, and deletes messages. These are error messages generated by failed operations
/// as a way to find out what happened when an asynchronous operation failed.
#[derive(Parser)]
pub struct MessageCommand {
    /// subcommand
    #[command(subcommand)]
    command: MessageCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum MessageCommands {
    Delete(delete::MessageCommand),
    List(list::MessagesCommand),
    Show(show::MessageCommand),
}

impl MessageCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            MessageCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            MessageCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            MessageCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
