use tokio::fs;
use async_trait::async_trait;
use anyhow::{anyhow, Result};

/// 数据源获取的途径和方法
async fn retrieve_remote_data(source: impl AsRef<str>) -> Result<String> {
    let name = source.as_ref();
    match &name[..4] {
        "http" => UrlFetch(name).fetch().await,
        "file" => FileFetch(name).fetch().await,
        _ => Err(anyhow!("We only support http/file method")),
    }
}

#[async_trait]
pub trait Fetch {
    type Error;
    async fn fetch(&self) -> Result<String, Self::Error>;
}

struct UrlFetch<'a>(&'a str);
struct FileFetch<'a>(&'a str);

#[async_trait]
impl<'a> Fetch for UrlFetch<'a> {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String, Self::Error> {
        Ok(reqwest::get(self.0).await?.text().await?)
    }
}

#[async_trait]
impl<'a> Fetch for FileFetch<'a> {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String, Self::Error> {
        Ok(fs::read_to_string(&self.0[7..]).await?)
    }
}
