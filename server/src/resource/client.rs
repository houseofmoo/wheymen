use crate::model::{db::DbResponse, error::LocalError};
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub struct DbClient {
    pub client: reqwest::Client,
    pub url: String,
    pub user: String,
    pub pass: String,
}

impl DbClient {
    pub fn new(url: String, user: String, pass: String) -> DbClient {
        DbClient {
            client: reqwest::Client::new(),
            url,
            user,
            pass,
        }
    }

    fn build_request<S: Into<String>>(&self, query: S) -> RequestBuilder
    where
        reqwest::Body: From<S>,
    {
        self.client
            .post(&self.url)
            .header("Accept", "*/*")
            .header("Content-Type", "application/json")
            .header("NS", "wheymen")
            .header("DB", "wheymen")
            .basic_auth(&self.user, Some(&self.pass))
            .body(query)
    }

    pub async fn send_query<T>(&self, query: String) -> Result<Vec<DbResponse<T>>, LocalError>
    where
        T: DeserializeOwned,
    {
        let res = self.build_request(query).send().await?;
        let res_text = res.text().await?;
        let obj: Vec<DbResponse<T>> = serde_json::from_str(&res_text)?;
        Ok(obj)
    }
}
