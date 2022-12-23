//! Command handlers.

use crate::capability::ClientCapabilitySet;

use super::*;
use futures::future::BoxFuture;
use sable_network::prelude::*;
use messages::*;
use client::*;
use crate::utils::ClientCommandExt;

use std::{
    collections::HashMap,
    sync::Arc,
    str::FromStr,
};

mod argument_list;
pub use argument_list::*;

mod client_command;
pub use client_command::*;

mod action;
pub use action::*;

mod error;
pub use error::*;

mod dispatcher;
pub use dispatcher::*;

mod plumbing;
pub use plumbing::CommandContext;

/// A convenience definition for the result type returned from command handlers
pub type CommandResult = Result<(), CommandError>;

pub type AsyncHandler = BoxFuture<'static, ()>;

mod handlers
{
    // These are here so the handler modules can import everything easily
    use super::*;
    use sable_macros::command_handler;
    use plumbing::*;
    use std::{
        ops::Deref,
    };

    mod cap;
    mod nick;
    mod user;
    mod join;
    mod part;
    mod notice;
    mod privmsg;
    mod quit;
    mod mode;
    mod ping;
    mod names;
    mod who;
    mod whois;
    mod topic;
    mod invite;
    mod kill;
    mod kline;
    mod oper;
    mod chathistory;
    pub mod register;

    // Interim solutions that need refinement
    mod session;

    // Services compatibility command layer
    mod ns;
    mod cs;

    // Dev/test tools
    #[cfg(debug)]
    mod async_wait;
    #[cfg(debug)]
    mod sping;
}
