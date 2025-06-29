mod currency;

use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

fn parse_query_params(url: &str) -> HashMap<String, String> {
    url.split('?')
        .nth(1)
        .map(|q| {
            q.split('&')
                .filter_map(|pair| {
                    let mut parts = pair.split('=');
                    Some((
                        parts.next()?.to_string(),
                        parts.next()?.to_string(),
                    ))
                })
                .collect()
        })
        .unwrap_or_default()
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let query_params = parse_query_params(req.uri().to_string().as_str());
    
    let from = query_params.get("from").ok_or("Missing 'from' parameter")?;
    let to = query_params.get("to").ok_or("Missing 'to' parameter")?;
    let amount: f64 = query_params.get("amount")
        .ok_or("Missing 'amount' parameter")?
        .parse()
        .map_err(|_| "Invalid amount format")?;

    match currency::convert_currency(from, to, amount).await {
        Ok(result) => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(json!(result).to_string().into())?),
        Err(e) => Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("Content-Type", "application/json")
            .body(json!({
                "error": e.to_string()
            }).to_string().into())?)
    }
} 