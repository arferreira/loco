#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::module_name_repetitions)]
//! `TODO::`
//!
//!
//! ## Starting A New Project
//!
//! To start a new project, you can use cargo-generate:
//!
//! ```sh
//! cargo install loco-cli
//! ❯ loco new
//! ✔ ❯ App name? · myapp
//! ? ❯ What would you like to build? ›
//! ❯ lightweight-service (minimal, only controllers and views)
//!   Rest API (with DB and user auth)
//!   Saas app (with DB and user auth)
//! ```
//!
//! ## Available Features
//!
//! To avoid compiling unused dependencies, loco gates certain features.
//!
//! | Feature    | Default | Description                 |
//! |------------|---------|-----------------------------|
//! | `auth_jwt` | true    | Enable user authentication. |
//! | `cli`      | true    | Expose Cli commands.        |
//! | `testing   | false   | Test Utilities Module.      |
//! | `with-db`  | true    | with-db.                    |
pub use self::errors::Error;

mod banner;
pub mod prelude;

#[cfg(feature = "with-db")]
pub mod doctor;

#[cfg(feature = "with-db")]
pub mod db;
#[cfg(feature = "with-db")]
pub mod model;
#[cfg(feature = "with-db")]
pub mod schema;
mod tera;

pub mod app;
#[cfg(feature = "cli")]
pub mod cli;

pub mod auth;
pub mod boot;
pub mod config;
pub mod controller;
pub mod environment;
pub mod errors;
mod gen;
pub mod hash;
mod logger;
pub mod mailer;
mod redis;
pub mod task;
#[cfg(feature = "testing")]
pub mod testing;
#[cfg(feature = "testing")]
pub use axum_test::TestServer;
pub mod validation;
pub mod worker;
pub use validator;

/// Application results options list
pub type Result<T> = std::result::Result<T, Error>;
