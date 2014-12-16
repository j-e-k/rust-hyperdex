#![feature(globs)]
#![feature(concat_idents)]
#![feature(macro_rules)]
#![feature(slicing_syntax)]
#![feature(default_type_params)]

extern crate libc;

pub use common::HyperError;
pub use client::Client;
pub use client_types::*;
pub use admin::Admin;

mod client;
mod admin;

mod hyperdex;
mod hyperdex_client;
mod hyperdex_admin;
mod hyperdex_datastructures;
mod hyperdex_hyperspace_builder;
mod common;
mod test;
mod client_types;
