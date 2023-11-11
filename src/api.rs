use serde::Deserialize;
use std::error::Error;
use log::{debug, error, info};

#[derive(Clone, Deserialize, Debug)]
pub struct Response {
    pub status: u32,
    pub message: String,
    pub short: String,
    pub long: String,
}

fn format_url(url: Option<&String>, custom: Option<&String>) -> String {
    let formatted_url = format!(
        "https://csclub.uwaterloo.ca/~phthakka/1pt/addURL.php?url={}&cu={}",
        url.as_ref().unwrap_or(&&"".to_string()),
        custom.as_ref().unwrap_or(&&"".to_string())
    );
    debug!("Formatted URL is: {}", formatted_url);
    formatted_url
}

pub async fn request_api(url: Option<&String>, custom: Option<&String>) -> Result<Response, Box<dyn Error>> {

    // Create the reqwest client and send the request
    info!("Starting new reqwest client and sending a request");
    let client = reqwest::Client::new();
    let url = format_url(url, custom);

    debug!("Sending a GET request to {}", url);
    let response = client
        .get(&url)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    if response.status().is_success() {
        debug!("Received HTTP response with a success status: {}", response.status());
        info!("Parsing response body to JSON");
        let response_body: Response = response.json().await?;
        Ok(response_body)
    } else {
        error!("Received HTTP response with an error status: {}", response.status());
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("HTTP error: {}", response.status()))))
    }
}