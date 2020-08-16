//! This crate provides some data structures for working with cargo package information,
//! as well as functionality to obtain said info about the current (your) package
#![no_std]

use core::env;
use semver::Version;

// note: maybe add a PackageVersionDifference type in the future

/// Get the package version for your cargo package
///
/// This is also used by `CargoPackageInfo::current()` to obtain the version
#[inline]
pub fn cargo_pkg_version() -> Version {
    Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
}

/// Information about a Cargo package
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CargoPackageInfo {
    version: Version,
    name: &'static str,
    authors: &'static str,
    description: &'static str,
    homepage: &'static str,
    repository: &'static str,
}

impl CargoPackageInfo {
    /// Get the info for your cargo package
    ///
    /// You are able to get the same info via
    /// the various `CARGO_PKG_*` environment variables
    /// but this has a nicer API
    pub fn current() -> Self {
        Self {
            version: cargo_pkg_version(),
            name: env!("CARGO_PKG_NAME"),
            authors: env!("CARGO_PKG_AUTHORS"),
            description: env!("CARGO_PKG_DESCRIPTION"),
            homepage: env!("CARGO_PKG_HOMEPAGE"),
            repository: env!("CARGO_PKG_REPOSITORY"),
        }
    }
}

#[allow(missing_docs)]
impl CargoPackageInfo {
    pub fn version(&self) -> &Version {
        &self.version
    }
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn authors(&self) -> &'static str {
        self.authors
    }
    pub fn description(&self) -> &'static str {
        self.description
    }
    pub fn homepage(&self) -> &'static str {
        self.homepage
    }
    pub fn repository(&self) -> &'static str {
        self.repository
    }
}

/// A Cargo compilation profile
#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
pub enum CargoProfile {
    /// Cargo's `debug` profile
    Debug,
    /// Cargo's `release` profile
    Release,
}

impl CargoProfile {
    /// The profile you are compiling with
    pub const fn current() -> Self {
        #[cfg(profile = "debug")]
        return CargoProfile::Debug;
        #[cfg(profile = "release")]
        return CargoProfile::Release;
        #[cfg(not(any(profile = "debug", profile = "release")))]
        panic!(concat!("unknown value for PROFILE: ", env!("PROFILE"), " (cargo introduced a new profile or the \"PROFILE\" environment variable was not set by cargo)"))
    }
}
