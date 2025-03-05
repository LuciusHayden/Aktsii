use reqwest;
use std::collections::{HashMap, BTreeMap};
use serde_json::json;

use crate::utils::config::Config;
use crate::models::stocks::{StockData, Stock};
use crate::routes::error::ApiError;

const URL: &str = "https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol={TKR}&interval=5min&month={YEAR_MONTH}&outputsize=full&apikey={API_KEY}";

pub async fn query_api(ticker: &str, year_month: &str) -> Result<Stock, ApiError> {
    let config = Config::new();
    let url = &URL
        .replace("{TKR}", ticker)
        .replace("{YEAR_MONTH}", year_month)
        .replace("{API_KEY}", &config.alpha_vantage_api_key);

    let mut parsed_data: serde_json::Value = 
        reqwest::get(url)
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let time_series = match parsed_data.get("Time Series (5min)") {
        Some(value) => value.clone(),
        None => {
            let url = &URL
                .replace("{TKR}", ticker)
                .replace("{YEAR_MONTH}", year_month)
                .replace("{API_KEY}", &config.backup_alpha_vantage_key);

            parsed_data = 
                reqwest::get(url)
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            if let Some(data) = parsed_data.get("Time Series (5min)") {
                data.clone()
            } else if parsed_data.to_string().contains("rate limit"){
                return Err(ApiError::AlphaVantageError) 
            } else {
                return Err(ApiError::AlphaVantageError) 
            }
        }
    };

    let mut stock_data : BTreeMap<String, StockData> = BTreeMap::new();

    let m : HashMap<String, StockData> = serde_json::from_value(time_series).unwrap();

    for (key, value) in m {
        stock_data.insert(key, value);
    }

    Ok(Stock {ticker: ticker.to_string(), data: stock_data })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn api_query() {
        let stock = query_api("IBM", "2001-09").await;

        println!("{:#?}", stock)
    }
}


