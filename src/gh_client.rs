use clap::{crate_name, crate_version};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use url::Url;

const AUTH_ENV_VAR: &str = "GITHUB_API_TOKEN";

#[derive(Deserialize, Debug)]
pub struct SearchHit {
    content: String,
    encoding: String,
    pub path: String,
    pub sha: String,
    pub html_url: String,
}

#[derive(Deserialize, Debug)]
struct GitHubClientError {
    message: String,
}

impl SearchHit {
    pub fn content_lines(&self) -> String {
        self.content
            .lines()
            .map(|line| base64::decode(line).unwrap())
            .map(|line| String::from_utf8_lossy(&line).to_string())
            .collect::<String>()
    }
}

#[derive(Deserialize, Debug)]
pub struct GhResponse {
    pub total_count: u32,
    pub incomplete_results: bool,
    pub items: Vec<ItemMatch>,
}

#[derive(Deserialize, Debug)]
pub struct ItemMatch {
    pub url: String,
    pub repository: Repo,
}

#[derive(Deserialize, Debug)]
pub struct Repo {
    pub full_name: String,
}

// #[derive(Sized, Debug)]
pub struct RequestSearch {
    pub query: String,
    pub page: u8,
    // TODO convert to enum
    pub options: HashMap<String, String>,
}

impl RequestSearch {
    pub fn new(query: String) -> Self {
        RequestSearch {
            page: 1,
            query: query,
            options: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: String, value: String) -> &mut Self {
        self.options.insert(key, value);
        self
    }

    fn to_query(&self) -> String {
        let mut url = format!("{}+in:file", self.query);
        url = format!("{}+org:{}", url, self.options[&"org".to_string()]);
        if self.options.contains_key(&"filename".to_string()) {
            url = format!("{}+filename:{}", url, self.options[&"filename".to_string()]);
        }
        if self.options.contains_key(&"lang".to_string()) {
            url = format!("{}+language:{}", url, self.options[&"lang".to_string()]);
        }
        format!("{}&page={}", url, self.page)
    }
}

// "https://api.github.com/search/code?q=HACK+in:file+org:Sage"
pub async fn find_files(
    client: &reqwest::Client,
    request: &RequestSearch,
) -> Result<GhResponse, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.github.com/search/code?q={}",
        request.to_query()
    );

    println!("Calling {}", url);

    let resp = client
        // .get("https://api.github.com/search/code")
        //
        .get(url.as_str())
        .headers(construct_headers())
        // .query(&[("q", request.to_query().as_str())])
        .send()
        .await
        .expect("Somthing went wrong making the API call");

    // println!("Status: {:#?}", resp.text().await?);
    match resp.status() {
        StatusCode::OK => (),
        status => {
            eprintln!("{}", resp.status());
            if status.is_client_error() {
                let error_response = resp.json::<GitHubClientError>().await?;
                eprintln!("Error: {}", error_response.message);
            }
            panic!();
        }
    }

    let parsed_response = resp
        .json::<GhResponse>()
        .await
        .expect("Couldnt parse JSON response");
    Ok(parsed_response)
}

pub async fn find_search_hits(
    client: &reqwest::Client,
    item: &ItemMatch,
) -> Result<SearchHit, Box<dyn std::error::Error>> {
    let url = Url::parse(item.url.as_str())?;
    let resp = client.get(url).headers(construct_headers()).send().await?;

    match resp.status() {
        StatusCode::OK => (),
        status => {
            eprintln!("{}", resp.status());
            if status.is_client_error() {
                let error_response = resp.json::<GitHubClientError>().await?;
                eprintln!("Error: {}", error_response.message);
            }
            panic!();
        }
    }

    let parsed_response = resp.json::<SearchHit>().await?;
    Ok(parsed_response)
}

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_str(&format!("{} {}", crate_name!(), crate_version!()).as_str()).unwrap(),
    );
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    if let Ok(api_token) = env::var(AUTH_ENV_VAR) {
        let value = format!("token {}", api_token.as_str());
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&value).unwrap());
    }
    headers
}
