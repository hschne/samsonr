use clap::{Clap};

use log::*;

use prettytable::{Table};
use prettytable::format;

use crate::errors::SamsonrError;
use crate::samson;
use crate::configuration;

/// List all available projects
#[derive(Clap, Debug)]
pub struct ProjectsCommand {}

impl ProjectsCommand {
    pub fn run(&self, client: &samson::Client) -> Result<(), SamsonrError> {
        debug!("Running projects command");
        let projects = client.projects()?;
        if let Some(projects) = projects.get("projects") {
            let mut table = create_table();
            table.add_row(row![b->"ID",b->"Name",b->"Deployed At",b->"Deployed By"]);
            for project in projects.iter() {
                table.add_row(row![project.id,project.name,project.last_deployed_at, project.last_deployed_by]);
            }
            table.printstd();
            return Ok(())
        };
        Err(SamsonrError { message: format!("No projects found!") })
    }
}

/// List all available stages for a project
#[derive(Clap, Debug)]
pub struct StagesCommand {
    // The project ID to list stages for
    #[clap(short, long)]
    project_id: Option<i32>,
}

impl StagesCommand {
    pub fn run(&self, client: &samson::Client) -> Result<(), SamsonrError> {
        debug!("Running stages command");
        let project_id = load_project(self.project_id)?;
        let stages = client.stages(project_id)?;
        
        if let Some(stages) = stages.get("stages") {
            let mut table = create_table();
            table.add_row(row![b->"ID",b->"Name"]);
            for stage in stages.iter() {
               table.add_row(row![stage.id, stage.name]);
            }
            table.printstd();
            return Ok(())
        };

        Err(SamsonrError { message: format!("No stages found!") })
    }
}

/// Deploy a reference to a specific stage
#[derive(Clap, Debug)]
pub struct DeployCommand {
    // The project ID to deploy to
    #[clap(short, long)]
    project_id: Option<i32>,
    // The ID of the stage to deploy to
    #[clap(short, long)]
    stage_id: i32,
    // The commit or branch reference to deploy
    reference: Option<String>,
}

impl DeployCommand{
    pub fn run(&self, client: &samson::Client) -> Result<(), SamsonrError> {
        debug!("Running deploy command");
        let project_id = load_project(self.project_id)?;
        let reference = get_reference(&self.reference)?;
        client.deploy(project_id, self.stage_id, &reference)?;
        println!("Deploying {} to project {}", reference, project_id);
        Ok(())
    }
}

fn get_reference(command_reference : &Option<String>) -> Result<String, SamsonrError> {
        debug!("Loading reference");
    if let Some(reference) = command_reference {
        debug!("Got reference from argument, reference={}", reference);
        return Ok(reference[..].to_string())
    }

    if let Some(reference) = current_branch() {
        debug!("Got reference from git, reference={}", reference);
        return Ok(reference)
    }

    error!("No valid reference found");
    Err(SamsonrError { message: format!("No branch reference provided") })
}

fn load_project(project_id : Option<i32>) -> Result<i32, SamsonrError> {
    debug!("Loading project ID");
    let configuration = configuration::Configuration::new();

    if let Some(project_id) = project_id {
        debug!("Got Project ID from argument, project_id={}", project_id);
        return Ok(project_id)
    }

    if let Ok(config) = configuration {
        if let Some(project_id) = config.project_id {
            debug!("Got Project ID from configuration, project_id={}", project_id);
            return Ok(project_id);
        }
    }

    error!("No project ID found");
    Err(SamsonrError { message: format!("Missing project ID") })
}

fn current_branch() -> Option<String> {
        debug!("Fetching current branch from git");
    let command_output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output();
    if let Ok(output) = command_output {
        debug!("Received output");
        Some(format!("{}",String::from_utf8_lossy(&output.stdout)))
    }  else {
        error!("No command output received");
        None
    }
}

fn create_table() -> Table {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table
}
