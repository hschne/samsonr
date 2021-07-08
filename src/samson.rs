use std::collections::HashMap;

use crate::errors::SamsonrError;

use reqwest::blocking::Client as ReqwestClient;
use reqwest::header::{self};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use log::*;

pub struct Client {
    pub token: String,
    client: reqwest::blocking::Client
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub last_deployed_at: String,
    pub last_deployed_by: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stage {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeployResponse { 
    summary: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Error {
    pub status: i32,
    pub error: String,
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

    pub fn projects(&self) -> Result<HashMap<String, Vec<Project>>, SamsonrError> {
        let projects_url = format!("{}/projects.json", BASE_URL);
        let response = self.client.get(projects_url)
            .send()?;


        let result = parse_response::<HashMap<String, Vec<Project>>>(response)?;
        Ok(result)
    }

    pub fn stages(&self, project_id: i32) -> Result<HashMap<String, Vec<Stage>>, SamsonrError> {
        let stages_url = format!("{}/projects/{}/stages.json", BASE_URL, project_id );

        let response = self.client.get(stages_url)
            .send()?;

        let result = parse_response::<HashMap<String, Vec<Stage>>>(response)?;
        Ok(result)
    }

    pub fn deploy(&self, project_id: i32, stage_id: i32, reference: &String) -> Result<(), SamsonrError> {
        let deploy_url = format!("{}/projects/{}/stages/{}/deploys.json", BASE_URL, project_id, stage_id );
        let mut deploy = HashMap::new();
        deploy.insert("reference", &reference);
        let mut map = HashMap::new();
        map.insert("deploy", deploy);
        let response = self.client.post(deploy_url)
            .json(&map)
            .send()?;

        parse_response::<DeployResponse>(response)?;
        Ok(())
    }
}

fn parse_response<T>(response: reqwest::blocking::Response) -> Result<T, SamsonrError> where T: DeserializeOwned
{
    match response.status() {
        StatusCode::OK | StatusCode::CREATED => { 
            info!("Received {:?}", response); 

            let result = response.json::<T>()?;
            Ok(result)
        },
        _ => { 
            error!("Received {:?}", response); 
            let error = response.json::<Error>()?;
            Err(SamsonrError { message: format!("{} - {}", error.status, error.error) })
        }            
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
