mod currency;

use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    match currency::list_available_currencies().await {
        Ok(currencies) => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(json!({
                "currencies": currencies
            }).to_string().into())?),
        Err(e) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "application/json")
            .body(json!({
                "error": e.to_string()
            }).to_string().into())?)
    }
} 