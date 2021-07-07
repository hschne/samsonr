use clap::{Clap};

use crate::samson;

#[derive(Debug)]
pub struct CommandError { }

/// List all available projects
#[derive(Clap)]
pub struct ProjectsCommand {}

impl ProjectsCommand {
    pub fn run(&self, client: &samson::Client) {
        let projects = client.projects();
        println!("{:?}", projects)
    }
}

/// List all available stages for a project
#[derive(Clap)]
pub struct StagesCommand {
    // The project ID to list stages for
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
    // The project ID to deploy to
    project_id: i32,
    // The ID of the stage to deploy to
    stage_id: i32,
    // The commit or branch reference to deploy
    reference: Option<String>,
}

impl DeployCommand{
    pub fn run(&self, client: &samson::Client) -> Result<(), CommandError> {
        let reference = get_reference(&self.reference)?;
        let projects = client.deploy(self.project_id, self.stage_id, reference[..].to_string());
        println!("{:?}", projects);

        Ok(())
    }
}

fn get_reference(command_reference : &Option<String>) -> Result<String, CommandError> {
    if let Some(reference) = command_reference {
        return Ok(reference[..].to_string())
    }

    if let Some(reference) = current_branch() {
        return Ok(reference)
    }

    Err(CommandError {})
}

fn current_branch() -> Option<String> {
    let command_output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output();
    if let Ok(output) = command_output {
        Some(format!("{}",String::from_utf8_lossy(&output.stdout)))
    }  else {
        None
    }
}
