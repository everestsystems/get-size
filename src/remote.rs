//! Contains implementation of `MemoryUsage` for very common external
//! crates. Each of them must be enable with the `enable-<crate-name>`
//! feature.

#[cfg(feature = "dashmap")]
pub mod dashmap;

#[cfg(feature = "serde_json")]
pub mod serde_json;

#[cfg(feature = "uuid")]
pub mod uuid;
