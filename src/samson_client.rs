use std::collections::HashMap;

use reqwest::blocking::Client as ReqwestClient;
use reqwest::header::{self, HeaderValue};
use serde::{Deserialize, Serialize};

pub struct Client {
    pub token: String,
    client: reqwest::blocking::Client
}

#[derive(Debug)]
pub struct ClientError {
    message: &'static str,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    id: i32,
    name: String,
    last_deployed_at: String,
    last_deployed_by: String,
}

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

static base_url : &str= "https://deploy.meisterlabs.com";

impl Client {
    pub fn new(token: &String) -> Result<Self, ClientError> {
        let client = build_client(token);
        match client {
            Ok(client) => Ok(Client { token: token[..].to_string(), client: client }),
            _ => Err(ClientError { message: "Failed to build client" })
        }
    }

    pub fn projects(&self) -> Result<Option<Vec<Project>>, reqwest::Error> {
        let projects_url = format!("{}/projects.json", base_url);
        let body = self.client.get(projects_url)
            .send()?
            .json::<HashMap<String, Vec<Project>>>()?;
        match body.get("projects") {
            Some(projects) => Ok(None),
            None => Ok(None)
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
