use serde::{Deserialize, Serialize};
use std::error::Error;
use colorful::{Color, Colorful};

#[derive(Serialize)]
struct Url {
    input: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct Response {
    pub long: String,
    pub code: String,
}

pub async fn request_api(url: &Option<String>) -> Result<Response, Box<dyn Error>> {
    let json_body = Url { input: url.clone() };

    // Serialize the struct to a JSON string
    let json_string = serde_json::to_string(&json_body)?;

    // Create the reqwest client and send the request
    let client = reqwest::Client::new();
    let post = client
        .post("https://gotiny.cc/api")
        .header("Content-Type", "application/json")
        .body(json_string)
        .send()
        .await?;

    let error_message = if post.status().is_success() {
        let info = post.text().await?;

        let parsed: Vec<Response> = serde_json::from_str(&info)?;

        if let Some(output) = parsed.first() {
            return Ok(output.clone());
        }

        "Error: No data in response".to_string().color(Color::Red).to_string()
    } else {
        format!("HTTP error: {}", post.status())
    };

    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_message)))
}
