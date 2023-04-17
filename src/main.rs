use reqwest::Client;
use std::{env, error::Error};
use tokio;
use anyhow::{Result, Context, anyhow};

mod confluence {
    use reqwest::{Client, StatusCode};
    use serde::{Deserialize, Serialize};
    use std::fmt;
    use super::*;

    #[derive(Debug)]
    pub struct ConfluenceError {
        pub url: String,
        pub status: StatusCode,
        pub body: String,
    }

    impl fmt::Display for ConfluenceError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Error downloading page from Confluence: URL: {}, status: {}, body: {}",
                self.url, self.status, self.body
            )
        }
    }

    impl Error for ConfluenceError {}

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ConfluencePage {
        pub id: String,
        pub title: String,
        pub body: ConfluencePageBody,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ConfluencePageBody {
        pub view: ConfluencePageBodyView
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ConfluencePageBodyView {
        pub value: String,
    }

    #[derive(Deserialize)]
    struct PageResponse {
        results: Vec<ConfluencePage>,
    }

    async fn download_page(
        client: &Client,
        base_url: &str,
        space_key: &str,
        start: usize,
        limit: usize,
        auth: &str,
    ) -> Result<PageResponse, ConfluenceError> {
        let url = format!(
            "{}/rest/api/content?spaceKey={}&limit={}&start={}&expand=body.view",
            base_url, space_key, limit, start
        );

        eprintln!("Downloading page from Confluence: {}", url);

        let response = client
            .get(&url)
            .header("Authorization", format!("Basic {}", auth))
            .send()
            .await
            .map_err(|e| ConfluenceError {
                url: url.clone(),
                status: StatusCode::default(),
                body: e.to_string(),
            })?;

        let status = response.status();
        let body = response.text().await.unwrap_or_else(|_| String::new());

        if !status.is_success() {
            return Err(ConfluenceError { url, status, body });
        }

        let page_response = serde_json::from_str(&body).map_err(|_| ConfluenceError {
            url,
            status,
            body,
        })?;

        Ok(page_response)
    }

    pub async fn download_all_pages(
        client: &Client,
        base_url: &str,
        space_key: &str,
        auth: &str,
    ) -> Result<Vec<ConfluencePage>> {
        let mut all_pages = Vec::new();
        let mut start: usize = 0;
        let limit: usize = 25;

        loop {
            let response = download_page(client, base_url, space_key, start, limit, auth).await?;

            if response.results.is_empty() {
                break;
            }

            all_pages.extend(response.results);
            start += limit;
        }

        Ok(all_pages)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let base_url = env::var("CONFLUENCE_BASE_URL").context("getting CONFLUENCE_BASE_URL")?;
    let space_key = env::var("CONFLUENCE_SPACE_KEY").context("getting CONFLUENCE_SPACE_KEY")?;
    let auth = env::var("CONFLUENCE_AUTH").context("getting CONFLUENCE_AUTH")?;

    eprintln!("Downloading pages from Confluence space '{}'", space_key);
    eprintln!("  Base URL: {}", base_url);
    eprintln!("  Auth: {}", auth);

    let client = Client::new();
    let all_pages = confluence::download_all_pages(&client, &base_url, &space_key, &auth).await?;

    eprintln!("Downloaded {} pages from Confluence space '{}':", all_pages.len(), space_key);
    for page in &all_pages {
        eprintln!("  - [{}] {}", page.id, page.title);
    }

    let json = serde_json::to_string_pretty(&all_pages)?;
    println!("{}", json);

    Ok(())
}

