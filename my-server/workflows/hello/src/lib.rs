use flawless::{workflow, workflow::Input};
use serde::{Deserialize, Serialize};

flawless::module! { name = "hello", version = "0.0.1" }

#[workflow("start_job")]
pub fn start_job(input: Input<Job>) {
    log::info!("👋 Hello job: {}!", input.name)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub name: String,
}
