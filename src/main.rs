mod config;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Response,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

async fn make_request(client: Client, current_link: &str, headers: &HeaderMap) -> Response {
    client
        .get(current_link)
        .headers(headers.clone())
        .send()
        .await
        .unwrap()
}

async fn make_requests(url: &str, api_path: &str, access_token: &str) -> Vec<Project> {
    let mut responses = Vec::new();

    let mut current_link = format!("{}{}", url, api_path);
    let mut headers = HeaderMap::new();
    headers.insert(
        "PRIVATE-TOKEN",
        HeaderValue::from_str(access_token).unwrap(),
    );

    let mut links = Vec::new();
    links.push(current_link.clone());

    loop {
        let client = Client::new();
        let headers = headers.clone();
        let response = make_request(client, current_link.as_str(), &headers).await;
        let response_headers = response.headers().clone();
        let link_header = response_headers.get("Link").unwrap().to_str().unwrap();

        let next_link = parse_link_header::parse(link_header).unwrap();
        let next_link = next_link.get(&Some("next".to_string()));

        if let Some(link) = &next_link {
            links.push(link.raw_uri.clone());
            current_link = format!("{}", link.raw_uri);
        } else {
            break;
        }
    }

    let mut response: Response;
    let mut tasks = Vec::new();

    for link in &links {
        let headers = headers.clone();
        let task = tokio::spawn(async move {
            let client = Client::new();
            let response = make_request(client, link, &headers).await;
            let response = response.json::<Vec<Project>>().await;
            match response {
                Ok(res) => {
                    println!("response: {:#?}", res);
                    res
                }
                Err(ref e) => {
                    eprintln!("Error making request: {}", e);
                    Vec::new()
                }
            }
        });
        tasks.push(task);
    }

    for task in tasks {
        responses.append(&mut task.await.unwrap());
    }

    responses
}

#[tokio::main]
async fn main() {
    let cfg = config::ApplicationConfiguration::new("./config/config.toml");
    let gitlab_config = cfg.clone().gitlab;

    // println!("config: {:?}", cfg);
    // println!("gitlab_config: {:?}", gitlab_config);

    let url = gitlab_config.base_url;
    let access_token = gitlab_config.access_token;

    let responses = make_requests(&url, "/api/v4/projects", &access_token).await;

    for response in &responses {
        println!("response: {:?}", response);
    }
}
