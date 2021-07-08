use std::collections::HashMap;

use crate::errors::SamsonrError;

use reqwest::blocking::Client as ReqwestClient;
use reqwest::header::{self};
use serde::{Deserialize, Serialize};


pub struct Client {
    pub token: String,
    client: reqwest::blocking::Client
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    id: i32,
    name: String,
    last_deployed_at: String,
    last_deployed_by: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stage {
    id: i32,
    name: String,
}

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

static BASE_URL : &str= "https://deploy.meisterlabs.com";

impl Client {
    pub fn new(token: &String) -> Result<Self, SamsonrError> {
        let client = build_client(token);
        match client {
            Ok(client) => Ok(Client { token: token[..].to_string(), client: client }),
            _ => Err(SamsonrError { message: format!("Failed to build client") })
        }
    }

    pub fn projects(&self) -> Result<HashMap<String, Vec<Project>>, reqwest::Error> {
        let projects_url = format!("{}/projects.json", BASE_URL);
        Ok(self.client.get(projects_url)
            .send()?
            .json::<HashMap<String, Vec<Project>>>()?)
    }

    pub fn stages(&self, project_id: i32) -> Result<HashMap<String, Vec<Stage>>, reqwest::Error> {
        let stages_url = format!("{}/projects/{}/stages.json", BASE_URL, project_id );
        Ok(self.client.get(stages_url)
            .send()?
            .json::<HashMap<String, Vec<Stage>>>()?)
    }

    pub fn deploy(&self, project_id: i32, stage_id: i32, reference: String) -> Result<(), reqwest::Error> {
        let deploy_url = format!("{}/projects/{}/stages/{}/deploys", BASE_URL, project_id, stage_id );
        let mut deploy = HashMap::new();
        deploy.insert("reference", &reference);
        let mut map = HashMap::new();
        map.insert("deploy", deploy);
        self.client.post(deploy_url)
            .json(&map)
            .send()?
            .text()?;
        Ok(())
    }
}

fn build_client(token: &String) -> Result<ReqwestClient, reqwest::Error> {
    let mut headers = header::HeaderMap::new();
    let auth_header = &format!("Bearer {}", token)[..];
    let header_value = header::HeaderValue::from_str(auth_header);
    if let Ok(header) = header_value {
        headers.insert(header::AUTHORIZATION, header);
    }

    ReqwestClient::builder()
        .user_agent(APP_USER_AGENT)
        .default_headers(headers)
        .build()
}
