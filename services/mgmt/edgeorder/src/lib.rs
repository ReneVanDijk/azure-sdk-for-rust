#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2022-05-preview")]
pub mod package_2022_05_preview;
#[cfg(all(feature = "package-2022-05-preview", not(feature = "no-default-tag")))]
pub use package_2022_05_preview::*;
#[cfg(feature = "package-2021-12")]
pub mod package_2021_12;
#[cfg(all(feature = "package-2021-12", not(feature = "no-default-tag")))]
pub use package_2021_12::*;
#[cfg(feature = "package-2020-12-preview")]
pub mod package_2020_12_preview;
#[cfg(all(feature = "package-2020-12-preview", not(feature = "no-default-tag")))]
pub use package_2020_12_preview::*;
