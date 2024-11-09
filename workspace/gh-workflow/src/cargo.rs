use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, strum_macros::Display)]
#[serde(rename_all = "kebab-case")]
pub enum CargoCommand {
    Add,
    Bench,
    Build,
    Check,
    Chef,
    Clean,
    Clippy,
    Component,
    Config,
    Criterion,
    Doc,
    Expand,
    Fetch,
    Fix,
    Flamegraph,
    Fmt,
    Generate,
    GenerateLockfile,
    GitCheckout,
    Help,
    Info,
    Init,
    Insta,
    Install,
    LocateProject,
    Login,
    Logout,
    Make,
    Metadata,
    Miri,
    New,
    Owner,
    Package,
    ReadManifest,
    Remove,
    Report,
    Run,
    Rustc,
    Rustdoc,
    Search,
    Test,
    Tree,
    Udeps,
    Uninstall,
    Update,
    Vendor,
    VerifyProject,
    Version,
    Watch,
    Yank,
}
#[derive(Debug, PartialEq, Eq)]
pub struct CargoTest;

#[derive(Debug, PartialEq, Eq)]
pub struct CargoClippy;

#[derive(Debug, PartialEq, Eq)]
pub struct CargoFmt;

#[derive(Debug, PartialEq, Eq)]
pub struct CargoCheck;

#[derive(Debug, PartialEq, Eq)]
pub struct CargoClean;

#[derive(Debug, PartialEq, Eq)]
pub struct Cargo<Ty> {
    pub command: CargoCommand,
    pub args: Vec<String>,
    marker: PhantomData<Ty>,
}

impl<Ty> Cargo<Ty> {
    // TODO: add function to handle command
    // specific sub command
    pub fn add_arg<S: ToString>(mut self, arg: S) -> Self {
        self.args.push(arg.to_string());

        self
    }
}

impl Cargo<CargoClean> {
    pub fn clean() -> Cargo<CargoClean> {
        Cargo {
            command: CargoCommand::Clean,
            args: vec![],
            marker: Default::default(),
        }
    }
}

impl Cargo<CargoCheck> {
    pub fn check() -> Cargo<CargoCheck> {
        Cargo {
            command: CargoCommand::Check,
            args: vec![],
            marker: Default::default(),
        }
    }
}

impl Cargo<CargoTest> {
    pub fn test() -> Cargo<CargoTest> {
        Cargo {
            command: CargoCommand::Test,
            args: vec![],
            marker: Default::default(),
        }
    }
    pub fn all_features(mut self) -> Self {
        if !self.args.iter().any(|arg| arg == "--all-features") {
            self.args.push("--all-features".to_string());
        }

        self
    }
    pub fn workspace(mut self) -> Self {
        if !self.args.iter().any(|arg| arg == "--workspace") {
            self.args.push("--workspace".to_string());
        }
        self
    }
}

impl Cargo<CargoClippy> {
    pub fn clippy() -> Cargo<CargoClippy> {
        Cargo {
            command: CargoCommand::Clippy,
            args: vec![],
            marker: Default::default(),
        }
    }
    pub fn all_features(mut self) -> Self {
        if !self.args.iter().any(|arg| arg == "--all-features") {
            self.args.push("--all-features".to_string());
        }
        self
    }
    pub fn workspace(mut self) -> Self {
        if !self.args.iter().any(|arg| arg == "--workspace") {
            self.args.push("--workspace".to_string());
        }
        self
    }
}

impl Cargo<CargoFmt> {
    pub fn fmt() -> Cargo<CargoFmt> {
        Cargo {
            command: CargoCommand::Fmt,
            args: vec![],
            marker: Default::default(),
        }
    }
    pub fn all(mut self) -> Self {
        if !self.args.iter().any(|arg| arg == "--all") {
            self.args.push("--all".to_string());
        }

        self
    }
    pub fn check(mut self) -> Self {
        if !self.args.iter().any(|arg| arg == "--check") {
            self.args.push("--check".to_string());
        }
        self
    }
}
