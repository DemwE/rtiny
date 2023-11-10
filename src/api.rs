use serde::Deserialize;
use std::error::Error;

#[derive(Clone, Deserialize)]
pub struct Response {
    pub status: u32,
    pub message: String,
    pub short: String,
    pub long: String,
}

pub async fn request_api(url: &Option<String>) -> Result<Response, Box<dyn Error>> {

    // Create the reqwest client and send the request
    let client = reqwest::Client::new();
    let post = client
        .get(&format!("https://csclub.uwaterloo.ca/~phthakka/1pt/addURL.php?url={}", url.as_ref().unwrap()))
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let error_message = if post.status().is_success() {
        let info = post.text().await?;

        let parsed: Response = serde_json::from_str(&info)?;

        return Ok(parsed);
    } else {
        format!("HTTP error: {}", post.status())
    };

    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_message)))
}
