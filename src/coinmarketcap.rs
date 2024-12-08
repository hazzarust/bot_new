use std::collections::HashMap;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, USER_AGENT};
use serde::Deserialize;
use thiserror::Error;
use flate2::read::GzDecoder;
use std::io::Read;
use std::env;

#[derive(Error, Debug)]
pub enum HypePriceError {
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Symbol '{0}' not found in the response data")]
    SymbolNotFound(String),

    #[error("Failed to decode the response body: {0}")]
    ResponseDecodeError(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    JsonParseError(#[from] serde_json::Error),
}

#[derive(Deserialize, Debug)]
struct CoinMarketCapResponse {
    data: HashMap<String, CoinData>,
}

#[derive(Deserialize, Debug)]
struct CoinData {
    symbol: String,
    quote: HashMap<String, USDQuote>,
}

#[derive(Deserialize, Debug)]
struct USDQuote {
    price: f64,
}

pub async fn hype_price(hype: &str) -> Result<f64, HypePriceError> {
    let url = format!(
        "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?symbol={}&convert=USD",
        hype
    );

    // Load the token from environment variable
    let api_key = env::var("COINMARKETCAP_TOKEN").unwrap_or_else(|_| {
        println!("COINMARKETCAP_TOKEN is not set!");
        String::new() // Or handle this error case as needed
    });

    let mut headers = HeaderMap::new();
    headers.insert(
        "X-CMC_PRO_API_KEY",
        HeaderValue::from_str(&api_key).expect("Invalid API Key"),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate"));

    let client = reqwest::Client::new();
    let response = client.get(&url).headers(headers).send().await?;

    // Check the status first to ensure the request was successful
    if !response.status().is_success() {
        return Err(HypePriceError::RequestError(
            response.error_for_status().unwrap_err(), // Extract the error
        ));
    }

    // Get the response body as raw bytes
    let response_body = response.bytes().await?;

    // Decompress the response body directly (assuming it's gzip)
    let mut decoder = GzDecoder::new(&response_body[..]);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;

    // Parse the decompressed bytes as JSON
    let response_json: CoinMarketCapResponse = match serde_json::from_slice(&decompressed) {
        Ok(data) => data,
        Err(e) => return Err(HypePriceError::JsonParseError(e)),
    };

    // Check if the data contains the symbol and retrieve the price
    if let Some(coin_data) = response_json.data.get(hype) {
        if let Some(usd_quote) = coin_data.quote.get("USD") {
            return Ok(usd_quote.price);  // Return the price if found
        }
    }

    // If the symbol is not found in the response data, return an error
    Err(HypePriceError::SymbolNotFound(hype.to_string()))
}
