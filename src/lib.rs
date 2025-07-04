#![warn(missing_docs)]

//! This library provides an extendable Bitcoin-Electrum client that supports batch calls,
//! notifications and multiple transport methods.
//!
//! By default this library is compiled with support for SSL servers using [`rustls`](https://docs.rs/rustls) and support for
//! plaintext connections over a socks proxy, useful for Onion servers. Using different features,
//! the SSL implementation can be removed or replaced with [`openssl`](https://docs.rs/openssl).
//!
//! A `minimal` configuration is also provided, which only includes the plaintext TCP client.
//!
//! # Example
//!
//! ```no_run
//! use electrum::{Client, ElectrumApi};
//!
//! let mut client = Client::new("tcp://electrum.blockstream.info:50001")?;
//! let response = client.server_features()?;
//! # Ok::<(), electrum::Error>(())
//! ```

extern crate core;
extern crate log;
#[cfg(feature = "use-openssl")]
extern crate openssl;
#[cfg(all(
    any(
        feature = "default",
        feature = "use-rustls",
        feature = "use-rustls-ring"
    ),
    not(feature = "use-openssl")
))]
extern crate rustls;
extern crate serde;
extern crate serde_json;

#[cfg(any(
    feature = "default",
    feature = "use-rustls",
    feature = "use-rustls-ring"
))]
extern crate webpki_roots;

#[cfg(any(feature = "default", feature = "proxy"))]
extern crate byteorder;

extern crate amplify;
extern crate bp;
#[cfg(all(unix, any(feature = "default", feature = "proxy")))]
extern crate libc;
extern crate sha2;
#[cfg(all(windows, any(feature = "default", feature = "proxy")))]
extern crate winapi;

#[cfg(any(feature = "default", feature = "proxy"))]
pub mod socks;

mod api;
mod batch;

#[cfg(any(
    all(feature = "proxy", feature = "use-openssl"),
    all(feature = "proxy", feature = "use-rustls"),
    all(feature = "proxy", feature = "use-rustls-ring")
))]
pub mod client;

mod config;

pub mod raw_client;
mod stream;
mod types;
#[cfg(test)]
pub mod utils;

pub use api::ElectrumApi;
pub use batch::Batch;
#[cfg(any(
    all(feature = "proxy", feature = "use-openssl"),
    all(feature = "proxy", feature = "use-rustls"),
    all(feature = "proxy", feature = "use-rustls-ring")
))]
pub use client::*;
pub use config::{Config, ConfigBuilder, Socks5Config};
pub use types::*;
