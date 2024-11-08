//! The typed version of https://github.com/actions-rust-lang/setup-rust-toolchain

use std::fmt::{Display, Formatter};

use derive_setters::Setters;

use crate::{AddStep, Job, RustFlags, Step};

#[derive(Clone)]
pub enum Toolchain {
    Stable,
    Nightly,
    Custom((u64, u64, u64)),
}

impl Display for Toolchain {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Toolchain::Stable => write!(f, "stable"),
            Toolchain::Nightly => write!(f, "nightly"),
            Toolchain::Custom(s) => write!(f, "{}.{}.{}", s.0, s.1, s.2),
        }
    }
}

impl Toolchain {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Toolchain::Custom((major, minor, patch))
    }
}

#[derive(Clone, Debug)]
pub enum Component {
    Clippy,
    Rustfmt,
    RustDoc,
}

impl Display for Component {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Component::Clippy => "clippy",
            Component::Rustfmt => "rustfmt",
            Component::RustDoc => "rust-doc",
        };
        write!(f, "{}", val)
    }
}

#[derive(Clone)]
pub enum Arch {
    X86_64,
    Aarch64,
    Arm,
}

impl Display for Arch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Arch::X86_64 => "x86_64",
            Arch::Aarch64 => "aarch64",
            Arch::Arm => "arm",
        };
        write!(f, "{}", val)
    }
}

#[derive(Clone)]
pub enum Vendor {
    Unknown,
    Apple,
    PC,
}

impl Display for Vendor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Vendor::Unknown => "unknown",
            Vendor::Apple => "apple",
            Vendor::PC => "pc",
        };
        write!(f, "{}", val)
    }
}

#[derive(Clone)]
pub enum System {
    Unknown,
    Windows,
    Linux,
    Darwin,
}

impl Display for System {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            System::Unknown => "unknown",
            System::Windows => "windows",
            System::Linux => "linux",
            System::Darwin => "darwin",
        };
        write!(f, "{}", val)
    }
}

#[derive(Clone)]
pub enum Abi {
    Unknown,
    Gnu,
    Msvc,
    Musl,
}

impl Display for Abi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Abi::Unknown => "unknown",
            Abi::Gnu => "gnu",
            Abi::Msvc => "msvc",
            Abi::Musl => "musl",
        };
        write!(f, "{}", val)
    }
}

#[derive(Clone, Setters)]
pub struct Target {
    arch: Arch,
    vendor: Vendor,
    system: System,
    abi: Option<Abi>,
}

///
/// A Rust representation for the inputs of the setup-rust action.
/// More information can be found [here](https://github.com/actions-rust-lang/setup-rust-toolchain/blob/main/action.yml).
/// NOTE: The public API should be close to the original action as much as
/// possible.
#[derive(Default, Clone, Setters)]
#[setters(strip_option)]
pub struct ToolchainStep {
    pub toolchain: Vec<Toolchain>,
    pub target: Option<Target>,
    pub components: Vec<Component>,
    pub cache: Option<bool>,
    pub cache_directories: Vec<String>,
    pub cache_workspaces: Vec<String>,
    pub cache_on_failure: Option<bool>,
    pub cache_key: Option<String>,
    pub matcher: Option<bool>,
    pub rust_flags: Option<RustFlags>,
    pub override_default: Option<bool>,
}

impl ToolchainStep {
    pub fn add_toolchain(mut self, version: Toolchain) -> Self {
        self.toolchain.push(version);
        self
    }
}

impl AddStep for ToolchainStep {
    fn apply(self, job: Job) -> Job {
        let mut step = Step::uses("actions-rust-lang", "setup-rust-toolchain", 1);

        let toolchain = self
            .toolchain
            .iter()
            .map(|t| match t {
                Toolchain::Stable => "stable".to_string(),
                Toolchain::Nightly => "nightly".to_string(),
                Toolchain::Custom((major, minor, patch)) => {
                    format!("{}.{}.{}", major, minor, patch)
                }
            })
            .reduce(|acc, a| format!("{}, {}", acc, a));

        if let Some(toolchain) = toolchain {
            step = step.with(("toolchain", toolchain));
        }

        if let Some(target) = self.target {
            let target = format!(
                "{}-{}-{}{}",
                target.arch,
                target.vendor,
                target.system,
                target.abi.map(|v| v.to_string()).unwrap_or_default(),
            );

            step = step.with(("target", target));
        }

        if !self.components.is_empty() {
            let components = self
                .components
                .iter()
                .map(|c| c.to_string())
                .reduce(|acc, a| format!("{}, {}", acc, a))
                .unwrap_or_default();

            step = step.with(("components", components));
        }

        if let Some(cache) = self.cache {
            step = step.with(("cache", cache));
        }

        if !self.cache_directories.is_empty() {
            let cache_directories = self
                .cache_directories
                .iter()
                .fold("".to_string(), |acc, a| format!("{}\n{}", acc, a));

            step = step.with(("cache-directories", cache_directories));
        }

        if !self.cache_workspaces.is_empty() {
            let cache_workspaces = self
                .cache_workspaces
                .iter()
                .fold("".to_string(), |acc, a| format!("{}\n{}", acc, a));

            step = step.with(("cache-workspaces", cache_workspaces));
        }

        if let Some(cache_on_failure) = self.cache_on_failure {
            step = step.with(("cache-on-failure", cache_on_failure));
        }

        if let Some(cache_key) = self.cache_key {
            step = step.with(("cache-key", cache_key));
        }

        if let Some(matcher) = self.matcher {
            step = step.with(("matcher", matcher));
        }

        if let Some(rust_flags) = self.rust_flags {
            step = step.with(("rust-flags", rust_flags.to_string()));
        }

        if let Some(override_default) = self.override_default {
            step = step.with(("override", override_default));
        }

        job.add_step(step)
    }
}
