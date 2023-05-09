//! Anagolay Parachain Node CLI

#![warn(missing_docs)]
// let clippy disregard expressions like 1 * UNITS
#![allow(clippy::identity_op)]
// clippy ignores a warning originating in ChainSpecGroup macro
#![allow(clippy::derive_partial_eq_without_eq)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;
mod rpc;

fn main() -> sc_cli::Result<()> {
  command::run()
}
