use clap::{crate_name, crate_version, App, Arg};
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use url::Url;

const AUTH_ENV_VAR: &str = "GITHUB_API_TOKEN";
#[derive(Deserialize, Debug)]
struct GhResponse {
    total_count: u32,
    incomplete_results: bool,
    items: Vec<ItemMatch>,
}

#[derive(Deserialize, Debug)]
struct ItemMatch {
    url: String,
    repository: Repo,
}

#[derive(Deserialize, Debug)]
struct Repo {
    full_name: String,
}

#[derive(Deserialize, Debug)]
struct SearchHit {
    content: String,
    encoding: String,
    path: String,
    sha: String,
    html_url: String,
}

#[derive(Deserialize, Debug)]
struct GitHubClientError {
    message: String,
}

impl SearchHit {
    fn content_lines(&self) -> String {
        self.content
            .lines()
            .map(|line| base64::decode(line).unwrap())
            .map(|line| String::from_utf8_lossy(&line).to_string())
            .collect::<String>()
    }
}

// #[derive(Sized, Debug)]
struct RequestSearch {
    query: String,
    page: u8,
    // TODO convert to enum
    options: HashMap<String, String>,
}

impl RequestSearch {
    fn new(query: String) -> Self {
        RequestSearch {
            page: 1,
            query: query,
            options: HashMap::new(),
        }
    }

    fn add(&mut self, key: String, value: String) -> &mut Self {
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

struct Repeater {
    // items: std::slice::Iter<'a, ItemMatch>,
    items: Vec<ItemMatch>,
    incomplete_results: bool,
    client: reqwest::Client,
    request: RequestSearch,
}

impl Repeater {
    fn new(request: RequestSearch) -> Self {
        Repeater {
            incomplete_results: true,
            // items: Vec::new().iter(),
            items: Vec::new(),
            // items: iter::empty::<ItemMatch>,
            client: reqwest::Client::new(),
            request: request,
        }
    }
}

// impl<'a> Iterator for &'a Repeater {
//     type Item = &'a ItemMatch;
//
//     async fn next(&mut self) -> Option<Self::Item> {
//         match self.items.iter().next() {
//             Some(i) => return Some(i),
//             None => {
//                 if self.incomplete_results == true {
//                     println!("incomplete");
//                     // request next page
//                     // self.request.add("page".to_string(), "2"); // update page
//                     let gh_response = find_files(&self.client, &self.request).await?;
//                     self.items = gh_response.items;
//                     return None;
//                 } else {
//                     return None; // There are no more pages
//                 }
//             }
//         };
//     }
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_arguments();
    let query = args.value_of("QUERY").unwrap();
    let mut request = RequestSearch::new(query.to_string());

    if let Some(lang) = args.value_of("lang") {
        request.add("lang".to_string(), lang.to_string());
    };

    if let Some(filename) = args.value_of("filename") {
        request.add("filename".to_string(), filename.to_string());
    };

    if let Some(org) = args.value_of("org") {
        request.add("org".to_string(), org.to_string());
    };

    let client = reqwest::Client::new();

    // let foo = Repeater::new(request);
    //
    // for i in foo.into_iter() {
    //     println!("{}", i.url);
    // }

    let gh_response = find_files(&client, &request).await?;
    match gh_response.total_count {
        0 => {
            println!("No results found.");
            return Ok(());
        }
        _ => println!("Found in {} file(s)", gh_response.total_count),
    }

    if gh_response.incomplete_results {
        println!("WARN: Showing incomplete results.");
    }

    for item in gh_response.items.iter() {
        // println!("{:#?}", &item.url);
        let parsed_response = find_search_hits(&client, &item).await?;
        process_search_hits(&request.query, item, parsed_response);
    }

    Ok(())
}

pub fn parse_arguments() -> clap::ArgMatches<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        // .author("Dave Elliott")
        .about("Utility to Search GitHub")
        .arg(
            Arg::with_name("filename")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("File in repository to be searched"),
        )
        .arg(
            Arg::with_name("org")
                .long("org")
                .takes_value(true)
                .required(true)
                .help("Which GitHub Organisation to search"),
        )
        .arg(
            Arg::with_name("lang")
                .long("lang")
                .short("l")
                .takes_value(true)
                .help("Filter scan to only search files of a certain language (e.g. rb, js)"),
        )
        .arg(
            Arg::with_name("QUERY")
                .help("Text to find")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("api-token")
                .env("GITHUB_API_TOKEN")
                .hide_env_values(true)
                .takes_value(true)
                .required(false)
                .help("Personal token for GitHub(https://github.com/settings/tokens/new)"),
        )
        // .arg(
        //     Arg::with_name("v")
        //         .short("v")
        //         .multiple(true)
        //         .help("Sets the level of verbosity. (Add more to increase level e.g. -vv)"),
        // )
        .get_matches()
}

// "https://api.github.com/search/code?q=HACK+in:file+org:Sage"
async fn find_files(
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

async fn find_search_hits(
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

fn process_search_hits(query: &String, item: &ItemMatch, search_hit: SearchHit) {
    let re = Regex::new(query.as_str()).unwrap();
    let repo_name = &item.repository.full_name;
    let mut counter = 0;
    for line in search_hit.content_lines().lines() {
        counter += 1;
        if re.is_match(line) {
            println!("{}/{}:{} {}", repo_name, search_hit.path, counter, line);
            // println!("{} {}:{}", repo_name, search_hit.path, counter);
            // println!("{}#L{}", search_hit.html_url, counter);
            // println!("{}", line);
        }
    }
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
