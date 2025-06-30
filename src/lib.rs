use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[wasm_bindgen]
pub struct CurrencyConverter {
    api_key: String,
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

#[wasm_bindgen]
impl CurrencyConverter {
    #[wasm_bindgen(constructor)]
    pub fn new(api_key: String) -> Self {
        CurrencyConverter { api_key }
    }

    #[wasm_bindgen]
    pub fn list_currencies(&self) -> js_sys::Promise {
        let api_key = self.api_key.clone();
        future_to_promise(async move {
            let url = format!(
                "https://v6.exchangerate-api.com/v6/{}/latest/USD",
                api_key
            );
            let response = reqwest::get(&url)
                .await
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            let rates: ExchangeRates = response
                .json()
                .await
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            let mut currencies: Vec<String> = rates.conversion_rates.keys().cloned().collect();
            currencies.sort();
            Ok(JsValue::from_serde(&currencies).unwrap())
        })
    }

    #[wasm_bindgen]
    pub fn convert(&self, from: String, to: String, amount: f64) -> js_sys::Promise {
        let api_key = self.api_key.clone();
        future_to_promise(async move {
            let url = format!(
                "https://v6.exchangerate-api.com/v6/{}/latest/{}",
                api_key, from
            );
            let response = reqwest::get(&url)
                .await
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            let rates: ExchangeRates = response
                .json()
                .await
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            let rate = rates
                .conversion_rates
                .get(&to)
                .ok_or_else(|| JsValue::from_str("Target currency not found"))?;
            let result = amount * rate;
            let timestamp = chrono::Utc::now().to_rfc3339();
            let conversion = ConversionResult {
                from,
                to,
                amount,
                result,
                rate: *rate,
                timestamp,
            };
            Ok(JsValue::from_serde(&conversion).unwrap())
        })
    }
} 