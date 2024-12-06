use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, USER_AGENT};
use serde::{Deserialize};
use tokio;

// Defines a HashMap with type String, CoinData.
// The key is a String (identifier), the value is CoinQuote.
#[derive(Deserialize, Debug)]
struct CoinMarketCapResponse{
    data: std::collections::HashMap<String, CoinData>
}

#[derive(Deserialize, Debug)]
struct CoinData{
    quote: HashMap<String, USDQuote>,
}

#[derive(Deserialize, Debug)]
struct USDQuote{
    price: f64
}

async fn hype_price(api_key: &str, hype: &str) -> Result<f64, reqwest::Error > {
    
    // Set HTTPS
    let url = format!("https://pro-api.coinmarketcap.com/v2/cryptocurrency/quotes/latest");

    // Create HeaderMap
    let mut headers = HeaderMap::new();

    // Inserts values into headers
    headers.insert(
        "X-CMC_PRO_API_KEY",
        // Puts a header value inside headers. 
        HeaderValue::from_str(api_key).expect("Invalid API Key"),
        );
    
    // Set key to USER_AGENT. from_static() = static string slice &'static str - lives for lifetime of program
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));

    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("deflate, gzip"));
    
    // This object represents a HTTP client, which is used to perform HHTP requests
    let client = reqwest::Client::new();

    // Crates a HTTP GET request to the URL provided
    // .headers(headers) adds custom headers to the HTTP request
    // Headers include extra metadata to HTTP request, help describe the request
    // or provide additional information to the server
    // .await means we wait for async operation to finish and retrieve result
    let response = client
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .json::<CoinMarketCapResponse>()
        .await?;

    // Make iterator that goes through the CoinMarketCapResponse and returns value



    Ok(1.0)
}