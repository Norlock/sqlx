#![feature(non_exhaustive, async_await, async_closure)]
#![cfg_attr(test, feature(test))]
#![allow(clippy::needless_lifetimes)]
// FIXME: Remove this once API has matured
#![allow(dead_code, unused_imports, unused_variables)]

#[cfg(test)]
extern crate test;

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate enum_tryfrom_derive;

mod options;
pub use self::options::ConnectOptions;

// Helper macro for writing long complex tests
#[macro_use]
pub mod macros;

pub mod backend;
pub mod deserialize;

#[macro_use]
pub mod row;

pub mod serialize;
pub mod types;

#[cfg(feature = "mariadb")]
pub mod mariadb;

#[cfg(feature = "postgres")]
mod postgres;

// TODO: This module is not intended to be directly public
pub mod connection;
pub mod pool;
