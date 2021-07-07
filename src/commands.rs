use clap::{Clap};

use crate::samson;

/// List all available projects
#[derive(Clap)]
pub struct ProjectsCommand {}

impl ProjectsCommand {
    pub fn run(&self, client: &samson::Client) {
        let projects = client.projects();
        println!("{:?}", projects)
    }
}

/// List all available stages
#[derive(Clap)]
pub struct StagesCommand {
    project_id: i32,
}

impl StagesCommand {
    pub fn run(&self, client: &samson::Client) {
        let projects = client.stages(self.project_id);
        println!("{:?}", projects)
    }
}

/// Deploy a reference to a specific stage
#[derive(Clap)]
pub struct DeployCommand {
    project_id: i32,
    stage_id: i32,
    reference: String,
}

impl DeployCommand {
    pub fn run(&self, client: &samson::Client) {
        let projects = client.deploy(self.project_id, self.stage_id, self.reference[..].to_string());
        println!("{:?}", projects)
    }
}
