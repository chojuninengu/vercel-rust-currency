use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::error::Error;

lazy_static::lazy_static! {
    static ref API_KEY: String = env::var("EXCHANGE_RATE_API_KEY")
        .expect("EXCHANGE_RATE_API_KEY must be set in environment variables");
    static ref API_BASE_URL: String = format!(
        "https://v6.exchangerate-api.com/v6/{}/latest",
        *API_KEY
    );
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeRates {
    pub result: String,
    pub documentation: String,
    pub terms_of_use: String,
    pub time_last_update_unix: i64,
    pub time_last_update_utc: String,
    pub time_next_update_unix: i64,
    pub time_next_update_utc: String,
    pub base_code: String,
    pub conversion_rates: HashMap<String, f64>,
}

#[derive(Debug, Serialize)]
pub struct ConversionResult {
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub result: f64,
    pub rate: f64,
    pub timestamp: String,
}

pub async fn fetch_exchange_rates(base_currency: &str) -> Result<ExchangeRates, Box<dyn Error + Send + Sync>> {
    let client = Client::new();
    let url = format!("{}/{}", *API_BASE_URL, base_currency);
    
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<ExchangeRates>()
        .await?;
    
    Ok(response)
}

pub async fn convert_currency(
    from: &str,
    to: &str,
    amount: f64,
) -> Result<ConversionResult, Box<dyn Error + Send + Sync>> {
    let rates = fetch_exchange_rates(from).await?;
    let rate = rates.conversion_rates.get(to).ok_or("Target currency not found")?;
    
    let result = amount * rate;
    let timestamp = chrono::Utc::now().to_rfc3339();
    
    Ok(ConversionResult {
        from: from.to_string(),
        to: to.to_string(),
        amount,
        result,
        rate: *rate,
        timestamp,
    })
}

pub async fn list_available_currencies() -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    let rates = fetch_exchange_rates("USD").await?;
    let mut currencies: Vec<String> = rates.conversion_rates.keys().cloned().collect();
    currencies.push("USD".to_string());
    currencies.sort();
    Ok(currencies)
} 