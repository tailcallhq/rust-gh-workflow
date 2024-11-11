use gh_workflow::{Step, Use};

#[derive(Clone, Default)]
pub struct ReleasePlz {
    /// The release-plz command to run. Accepted values: release-pr, release.
    /// (By default it runs both commands).
    pub command: Option<Command>,

    /// Registry where the packages are stored. The registry name needs to be
    /// present in the Cargo config. If unspecified, the publish field of the
    /// package manifest is used. If the publish field is empty, crates.io is
    /// used.
    pub registry: Option<String>,

    /// Path to the Cargo.toml of the project you want to update. Both Cargo
    /// workspaces and single packages are supported. (Defaults to the root
    /// directory).
    pub manifest_path: Option<String>,

    /// Release-plz version to use. E.g. 0.3.70. (Default: latest version).
    pub version: Option<String>,

    /// Release-plz config file location. (Defaults to release-plz.toml or
    /// .release-plz.toml).
    pub config: Option<String>,

    /// Token used to publish to the cargo registry.
    pub token: Option<String>,

    /// Forge backend. Valid values: github, gitea. (Defaults to github).
    pub backend: Option<Backend>,
}

#[derive(Clone)]
pub enum Command {
    ReleasePR,
    Release,
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::ReleasePR => write!(f, "release-pr"),
            Command::Release => write!(f, "release"),
        }
    }
}

#[derive(Clone)]
pub enum Backend {
    GitHub,
    Gitea,
}

impl std::fmt::Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Backend::GitHub => write!(f, "github"),
            Backend::Gitea => write!(f, "gitea"),
        }
    }
}

impl From<ReleasePlz> for Step<Use> {
    fn from(value: ReleasePlz) -> Self {
        let mut step = Step::uses("MarcoIeni", "release-plz-action", 0.5).name("Release Plz");

        if let Some(command) = value.command {
            step = step.add_with(("command", command.to_string()));
        }

        if let Some(registry) = value.registry {
            step = step.add_with(("registry", registry));
        }

        if let Some(manifest_path) = value.manifest_path {
            step = step.add_with(("manifest_path", manifest_path));
        }

        if let Some(version) = value.version {
            step = step.add_with(("version", version));
        }

        if let Some(config) = value.config {
            step = step.add_with(("config", config));
        }

        if let Some(token) = value.token {
            step = step.add_with(("token", token));
        }

        if let Some(backend) = value.backend {
            step = step.add_with(("backend", backend.to_string()));
        }

        step
    }
}
