//!
//! The serde representation of Github Actions Workflow.
//!

use derive_setters::Setters;
use indexmap::IndexMap;
use merge::Merge;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::fmt::Display;

use crate::error::Result;
use crate::generate::Generate;
use crate::{Cargo, Event, EventValue, RustFlags, ToolchainStep};

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct Jobs(IndexMap<String, Job>);
impl Jobs {
    pub fn insert(&mut self, key: String, value: Job) {
        self.0.insert(key, value);
    }
}

#[derive(Debug, Default, Setters, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Workflow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Env>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<EventValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Jobs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<Concurrency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<Defaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<IndexMap<String, Secret>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_minutes: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct EventAction {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    branches: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    branches_ignore: Vec<String>,
}

impl Workflow {
    pub fn new<T: ToString>(name: T) -> Self {
        Self { name: Some(name.to_string()), ..Default::default() }
    }
    pub fn to_string(&self) -> Result<String> {
        Ok(serde_yaml::to_string(self)?)
    }

    pub fn add_job<T: ToString, J: Into<Job>>(mut self, id: T, job: J) -> Self {
        let key = id.to_string();
        let mut jobs = self.jobs.unwrap_or_default();

        jobs.insert(key, job.into());

        self.jobs = Some(jobs);
        self
    }

    pub fn parse(yml: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(yml)?)
    }

    pub fn generate(self) -> Result<()> {
        Generate::new(self).generate()
    }

    pub fn add_event<T: Into<EventValue>>(mut self, that: T) -> Self {
        let mut this = self.on.unwrap_or_default();
        let that: EventValue = that.into();
        this.merge(that);
        self.on = Some(this);
        self
    }

    pub fn add_env<T: Into<Env>>(mut self, new_env: T) -> Self {
        let mut env = self.env.unwrap_or_default();

        env.0.extend(new_env.into().0);
        self.env = Some(env);
        self
    }

    pub fn setup_rust() -> Self {
        let build_job = Job::new("Build and Test")
            .add_step(Step::checkout())
            .add_step(
                Step::setup_rust()
                    .add_stable_toolchain()
                    .add_nightly_toolchain()
                    .add_clippy()
                    .add_fmt(),
            )
            .add_step(Step::cargo(Cargo::test().all_features().workspace()).name("Cargo Test"))
            .add_step(Step::cargo_nightly(Cargo::fmt().check()).name("Cargo Fmt"))
            .add_step(
                Step::cargo_nightly(
                    Cargo::clippy()
                        .all_features()
                        .workspace()
                        .add_arg("--")
                        .add_arg("-D warnings"),
                )
                .name("Cargo Clippy"),
            );

        let push_event = Event::push().branch("main");

        let pr_event = Event::pull_request_target()
            .open()
            .synchronize()
            .reopen()
            .branch("main");

        let event = push_event.combine(pr_event);

        let rust_flags = RustFlags::deny("warnings");

        Workflow::new("Build and Test")
            .env(rust_flags)
            .permissions(Permissions::read())
            .on(event)
            .add_job("build", build_job)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityType {
    Created,
    Edited,
    Deleted,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(transparent)]
pub struct RunsOn(Value);
impl From<Value> for RunsOn {
    fn from(value: Value) -> Self {
        RunsOn(value)
    }
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Job {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "if")]
    pub if_condition: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runs_on: Option<RunsOn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<Strategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<StepValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<Container>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<IndexMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<Concurrency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<IndexMap<String, Container>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<IndexMap<String, Secret>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<Defaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Env>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_on_error: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Artifacts>,
}

impl Job {
    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            name: Some(name.to_string()),
            runs_on: Some(RunsOn(Value::from("ubuntu-latest"))),
            ..Default::default()
        }
    }

    pub fn add_step<S: Into<StepValue>>(mut self, step: S) -> Self {
        let mut steps = self.steps.unwrap_or_default();
        steps.push(step.into().into());
        self.steps = Some(steps);
        self
    }
}

impl From<&str> for RunsOn {
    fn from(value: &str) -> Self {
        RunsOn(Value::String(value.to_string()))
    }
}

impl From<Vec<&str>> for RunsOn {
    fn from(value: Vec<&str>) -> Self {
        RunsOn(Value::Array(
            value
                .into_iter()
                .map(|v| v.to_string())
                .map(Value::String)
                .collect(),
        ))
    }
}

impl<V> From<Vec<(&str, V)>> for RunsOn
where
    V: Into<RunsOn>,
{
    fn from(value: Vec<(&str, V)>) -> Self {
        let val = value.into_iter().map(|(a, b)| (a.to_string(), b.into()));
        let mut map = Map::new();
        for (k, v) in val {
            map.insert(k.to_string(), v.0);
        }

        RunsOn(Value::Object(map))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum StepValue {
    Run(Step<Run>),
    Use(Step<Use>),
}

impl From<Step<Run>> for StepValue {
    fn from(step: Step<Run>) -> Self {
        StepValue::Run(step)
    }
}

impl From<Step<Use>> for StepValue {
    fn from(step: Step<Use>) -> Self {
        StepValue::Use(step)
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Use;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Run;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(transparent)]
pub struct Env(IndexMap<String, Value>);
impl From<IndexMap<String, Value>> for Env {
    fn from(value: IndexMap<String, Value>) -> Self {
        Env(value)
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct Input(IndexMap<String, Value>);
impl From<IndexMap<String, Value>> for Input {
    fn from(value: IndexMap<String, Value>) -> Self {
        Input(value)
    }
}

impl Input {
    pub fn add<S: ToString, V: Into<Value>>(mut self, key: S, value: V) -> Self {
        self.0.insert(key.to_string(), value.into());
        self
    }
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Step<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "if")]
    pub if_condition: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub uses: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with: Option<Input>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[setters(skip)]
    pub run: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Env>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_on_error: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Artifacts>,

    #[serde(skip)]
    marker: std::marker::PhantomData<T>,
}

impl Step<Run> {
    pub fn run<T: ToString>(cmd: T) -> Self {
        Step { run: Some(cmd.to_string()), ..Default::default() }
    }

    fn prepare_args<P: ToString>(params: Vec<P>) -> String {
        params
            .iter()
            .map(|a| a.to_string())
            .reduce(|a, b| format!("{} {}", a, b))
            .unwrap_or_default()
    }
    fn cargo_command<T>(prefix: Option<&str>, cargo: Cargo<T>) -> Self {
        Step::run(format!(
            "cargo{} {} {}",
            prefix.map(|v| format!(" {}", v)).unwrap_or_default(),
            cargo.command.to_string().to_lowercase(),
            Self::prepare_args(cargo.args)
        ))
    }

    pub fn cargo<T>(cargo: Cargo<T>) -> Self {
        Self::cargo_command(None, cargo)
    }

    pub fn cargo_nightly<T>(cargo: Cargo<T>) -> Self {
        Self::cargo_command(Some("+nightly"), cargo)
    }
}

impl Step<Use> {
    pub fn uses<Owner: ToString, Repo: ToString>(owner: Owner, repo: Repo, version: u64) -> Self {
        Step {
            uses: Some(format!(
                "{}/{}@v{}",
                owner.to_string(),
                repo.to_string(),
                version
            )),
            ..Default::default()
        }
    }

    pub fn checkout() -> Self {
        Step::uses("actions", "checkout", 4).name("Checkout Code")
    }

    pub fn setup_rust() -> ToolchainStep {
        ToolchainStep::default()
    }
}

impl<S1: ToString, S2: ToString> From<(S1, S2)> for Input {
    fn from(value: (S1, S2)) -> Self {
        let mut index_map: IndexMap<String, Value> = IndexMap::new();
        index_map.insert(value.0.to_string(), Value::String(value.1.to_string()));
        Input(index_map)
    }
}

impl<S1: Display, S2: Display> From<(S1, S2)> for Env {
    fn from(value: (S1, S2)) -> Self {
        let mut index_map: IndexMap<String, Value> = IndexMap::new();
        index_map.insert(value.0.to_string(), Value::String(value.1.to_string()));
        Env(index_map)
    }
}

impl From<Step<Use>> for Step<StepValue> {
    fn from(value: Step<Use>) -> Self {
        Step {
            id: value.id,
            name: value.name,
            if_condition: value.if_condition,
            uses: value.uses,
            with: value.with,
            run: value.run,
            env: value.env,
            timeout_minutes: value.timeout_minutes,
            continue_on_error: value.continue_on_error,
            working_directory: value.working_directory,
            retry: value.retry,
            artifacts: value.artifacts,
            marker: Default::default(),
        }
    }
}

impl From<Step<Run>> for Step<StepValue> {
    fn from(value: Step<Run>) -> Self {
        Step {
            id: value.id,
            name: value.name,
            if_condition: value.if_condition,
            uses: value.uses,
            with: value.with,
            run: value.run,
            env: value.env,
            timeout_minutes: value.timeout_minutes,
            continue_on_error: value.continue_on_error,
            working_directory: value.working_directory,
            retry: value.retry,
            artifacts: value.artifacts,
            marker: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Runner {
    #[default]
    Linux,
    MacOS,
    Windows,
    Custom(String),
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Container {
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Env>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<Port>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Port {
    Number(u16),
    Name(String),
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Volume {
    pub source: String,
    pub destination: String,
}

impl Volume {
    pub fn new(volume_str: &str) -> Option<Self> {
        let parts: Vec<&str> = volume_str.split(':').collect();
        if parts.len() == 2 {
            Some(Volume {
                source: parts[0].to_string(),
                destination: parts[1].to_string(),
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Concurrency {
    pub group: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_in_progress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Permissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_requests: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<PermissionLevel>,
}

impl Permissions {
    pub fn read() -> Self {
        Self { contents: Some(PermissionLevel::Read), ..Default::default() }
    }

    pub fn write() -> Self {
        Self { contents: Some(PermissionLevel::Write), ..Default::default() }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum PermissionLevel {
    Read,
    Write,
    #[default]
    None,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Strategy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_fast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parallel: Option<u32>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Environment {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Defaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<RunDefaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryDefaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<Concurrency>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct RunDefaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct RetryDefaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct Expression(String);

impl Expression {
    pub fn new<T: ToString>(expr: T) -> Self {
        Self(expr.to_string())
    }
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Secret {
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct RetryStrategy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<u32>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Artifacts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload: Option<Vec<Artifact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<Vec<Artifact>>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Artifact {
    pub name: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<u32>,
}
