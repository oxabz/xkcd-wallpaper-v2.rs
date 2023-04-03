use serde;
use serde::Deserialize;
use serde::Serialize;

use crate::error;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Xkcd {
    pub month: String,
    pub num: i64,
    pub link: String,
    pub year: String,
    pub news: String,
    #[serde(rename = "safe_title")]
    pub safe_title: String,
    pub transcript: String,
    pub alt: String,
    pub img: String,
    pub title: String,
    pub day: String,
}

impl Xkcd {
    pub async fn get_newest() -> Result<Xkcd, error::Error> {
        let url = "https://xkcd.com/info.0.json";
        let resp = reqwest::get(url).await?.json().await?;
        Ok(resp)
    }

    pub async fn get(id: u64) -> Result<Xkcd, error::Error> {
        let url = format!("https://xkcd.com/{}/info.0.json", id);
        let resp = reqwest::get(&url).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_image(&self) -> Result<Vec<u8>, error::Error> {
        let resp = reqwest::get(&self.img).await?.bytes().await?;
        Ok(resp.to_vec())
    }
}