use clap::{Clap};

use crate::errors::SamsonrError;
use crate::samson;
use crate::configuration;

/// List all available projects
#[derive(Clap, Debug)]
pub struct ProjectsCommand {}

impl ProjectsCommand {
    pub fn run(&self, client: &samson::Client) {
        let projects = client.projects();
        println!("{:?}", projects)
    }
}

/// List all available stages for a project
#[derive(Clap, Debug)]
pub struct StagesCommand {
    // The project ID to list stages for
    project_id: Option<i32>,
}

impl StagesCommand {
    pub fn run(&self, client: &samson::Client) -> Result<(), SamsonrError> {
        let project_id = load_project(self.project_id)?;
        let projects = client.stages(project_id);
        println!("{:?}", projects);
        Ok(())
    }
}

/// Deploy a reference to a specific stage
#[derive(Clap, Debug)]
pub struct DeployCommand {
    // The ID of the stage to deploy to
    stage_id: i32,
    // The project ID to deploy to
    project_id: Option<i32>,
    // The commit or branch reference to deploy
    reference: Option<String>,
}

impl DeployCommand{
    pub fn run(&self, client: &samson::Client) -> Result<(), SamsonrError> {
        let project_id = load_project(self.project_id)?;
        let reference = get_reference(&self.reference)?;
        let projects = client.deploy(project_id, self.stage_id, reference);
        println!("{:?}", projects);

        Ok(())
    }
}

fn get_reference(command_reference : &Option<String>) -> Result<String, SamsonrError> {
    if let Some(reference) = command_reference {
        return Ok(reference[..].to_string())
    }

    if let Some(reference) = current_branch() {
        return Ok(reference)
    }

    Err(SamsonrError { message: format!("No branch reference provided") })
}

fn load_project(project_id : Option<i32>) -> Result<i32, SamsonrError> {
    let configuration = configuration::Configuration::new();

    if let Some(project_id) = project_id {
        return Ok(project_id)
    }

    if let Ok(config) = configuration {
        if let Some(project_id) = config.project_id {
            return Ok(project_id);
        }
    }


    Err(SamsonrError { message: format!("Missing project ID") })
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
