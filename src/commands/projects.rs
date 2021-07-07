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
