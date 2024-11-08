use gh_workflow::{
    Cargo, Component, Event, Job, Permissions, PullRequest, Push, RustFlags, Step, Toolchain,
    Workflow,
};

fn main() {
    // TODO: embed all flags as enum
    let rust_flags = RustFlags::deny("warnings");

    let build = Job::new("Build and Test")
        .add_step(Step::checkout())
        .add_step(
            Step::setup_rust()
                .add_toolchain(Toolchain::Stable)
                .add_toolchain(Toolchain::Nightly)
                .components(vec![Component::Clippy, Component::Rustfmt]),
        )
        // .add_step(Step::cargo(
        //     Cargo::Test,
        //     vec!["--all-features", "--workspace"],
        // ))
        .add_step(Cargo::test().all_features().workspace())
        // .add_step(Step::cargo_nightly(Cargo::Fmt, vec!["--check"])))
        .add_step(Cargo::fmt().check().nightly())
        .add_step(Cargo::clippy().all_features().workspace().nightly())
        .add_step(Step::cargo_nightly(
            Cargo::Clippy,
            vec!["--all-features", "--workspace"],
        ));

    Workflow::new("CI")
        .env(rust_flags)
        .permissions(Permissions::read())
        .on(Event::default()
            .push(Push::default().branch("main"))
            .pull_request(
                PullRequest::default()
                    .open()
                    .synchronize()
                    .reopen()
                    .branch("main"),
            ))
        .add_job("build", build)
        .unwrap()
        .generate(format!(
            "{}/../../.github/workflows/ci.yml",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
}
