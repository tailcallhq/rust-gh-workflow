use gh_workflow::*;

fn main() {
    Workflow::setup_rust().autorelease_crate().generate().unwrap();
}
