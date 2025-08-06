
use waki::{Client, Method};

use crate::exports::betty_blocks::http_request::http;

wit_bindgen::generate!({ with: {
    "wasi:io/streams@0.2.6": ::wasi::io::streams,
    "wasi:io/error@0.2.6": ::wasi::io::error,
    "wasi:clocks/monotonic-clock@0.2.6": ::wasi::clocks::monotonic_clock,
    "wasi:io/poll@0.2.6": ::wasi::io::poll,
    "wasi:http/types@0.2.6": ::wasi::http::types,
    "wasi:http/outgoing-handler@0.2.6": ::wasi::http::outgoing_handler,
    }
});

struct HttpSender;

const TYPICODE_URL: &str = "https://jsonplaceholder.typicode.com/";

impl http::Guest for HttpSender {
    fn run(endpoint: String) -> Result<String, String> {
        // build the url
        let url = format!("{TYPICODE_URL}{}", endpoint.trim_start_matches('/'));

        let client = Client::new();
        let request = client.request(Method::Get, &url);
        // send the request
        let response = request.send().map_err(|e| e.to_string())?;

        // check the response if success
        if response.status_code().to_string().starts_with("2") {
            let body = response.body().map_err(|e| e.to_string())?;
            let text_data = String::from_utf8(body).map_err(|e| e.to_string())?;
            // parse the response to json
            let json_data: serde_json::Value = serde_json::from_str(&text_data).map_err(|e| e.to_string())?;
            // return the json
            Ok(json_data.to_string())
        } else {
            Err(format!("Request failed with status code: {}", response.status_code()))
        }
    }
}

export! {HttpSender}
