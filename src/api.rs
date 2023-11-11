use serde::Deserialize;
use std::error::Error;

/// This module provides a struct `Response` that represents a response from some API.
///
/// # Fields
///
/// - `status`: An unsigned 32-bit integer representing the status of the response.
/// - `message`: A string containing a message associated with the response.
/// - `short`: A string containing a short url of the response.
/// - `long`: A string containing a long url of the response.
///
/// # Remarks
///
/// - The `Clone` trait is implemented for `Response` to allow creating clones of the struct.
/// - The `Deserialize` trait is implemented for `Response` to enable deserialization from a data format such as JSON.
///
#[derive(Clone, Deserialize)]
pub struct Response {
    pub status: u32,
    pub message: String,
    pub short: String,
    pub long: String,
}

/// Formats a URL with optional custom parameters.
///
/// # Arguments
///
/// * `url` - An optional reference to a `String` containing the URL.
/// * `custom` - An optional reference to a `String` containing custom short url.
///
/// # Returns
///
/// A `String` containing the formatted URL.
///
fn format_url(url: Option<&String>, custom: Option<&String>) -> String {
    format!(
        "https://csclub.uwaterloo.ca/~phthakka/1pt/addURL.php?url={}&cu={}",
        url.as_ref().unwrap_or(&&"".to_string()),
        custom.as_ref().unwrap_or(&&"".to_string())
    )
}
/// Sends an HTTP GET request to the specified URL and returns the response.
///
/// # Arguments
///
/// * `url` - An optional reference to a `String` containing the URL.
/// * `custom` - An optional reference to a `String` containing custom short url.
///
/// # Returns
///
/// Returns a `Result` with the response as `Ok` if the request is successful, or an `Error` if there was an error.
/// The `Error` type is a trait object that can represent any type that implements the `Error` trait.
///
pub async fn request_api(url: Option<&String>, custom: Option<&String>) -> Result<Response, Box<dyn Error>> {

    // Create the reqwest client and send the request
    let client = reqwest::Client::new();
    let url = format_url(url, custom);
    let post = client
        .get(&url)
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
