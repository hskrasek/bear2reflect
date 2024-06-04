use std::fmt::{Debug, Display, Formatter};

use anyhow::anyhow;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use serde_json::{json, Value};

pub struct Client<'a> {
    access_token: &'a str,
    base_url: &'a str,
    client: reqwest::Client,
}

impl<'a> Client<'a> {
    pub fn new(access_token: &'a str) -> Self {
        let mut default_headers: HeaderMap = HeaderMap::new();
        default_headers.insert("Accept", "application/json".parse().unwrap());
        default_headers.insert("User-Agent", format!("Bear2Reflect/{}", env!("CARGO_PKG_VERSION")).parse().unwrap());

        let client = reqwest::Client::builder()
            .default_headers(default_headers)
            .build()
            .unwrap();

        Self {
            access_token,
            base_url: "https://reflect.app/api",
            client
        }
    }

    pub async fn get_graphs(
        &self
    ) -> Result<Vec<Graph>, Box<dyn std::error::Error>> {
        let graphs: Vec<Graph> = self
            .client
            .get(&format!("{}/graphs", self.base_url))
            .bearer_auth(self.access_token)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to fetch graphs: {}", e))?
            .json::<Vec<Graph>>()
            .await
            .map_err(|e| { anyhow!("Failed to decode response: {}", e)})?;

        Ok(graphs)
    }

    pub async fn create_note(
        &self,
        graph: &str,
        payload: &Value,
    ) -> Result<SuccessfulResponse, Box<dyn std::error::Error>> {
        let response: SuccessfulResponse = self
            .client
            .post(&format!("{}/graphs/{}/notes", self.base_url, graph))
            .header("Content-Type", "application/json")
            .bearer_auth(self.access_token)
            .json(payload)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to create new note: {}", e))?
            .json::<SuccessfulResponse>()
            .await?;

        Ok(response)
    }
}

#[derive(Deserialize, Debug)]
pub struct SuccessfulResponse {
    id: String,
    created_at: String,
    updated_at: String,
}

impl Display for SuccessfulResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", json!({"id": self.id, "created_at": self.created_at, "updated_at": self.updated_at}))
    }
}

#[derive(Deserialize, Debug)]
pub struct Graph {
    pub id: String,
    pub name: String,
}
