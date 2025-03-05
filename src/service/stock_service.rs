use reqwest;
use std::collections::BTreeMap;
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

    let mut time_series = parsed_data.get("Time Series (5min)").unwrap().clone();

    // just in case first api key doesnt work
    if time_series.is_null() {
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

        time_series = parsed_data.get("Time Series (5min)").unwrap().clone();
    }

    if !time_series.is_null() {
        let stock_data: BTreeMap<String, StockData> = serde_json::from_value(time_series).iter().map(|(timestamp, data): &(String, serde_json::Value)| {
            let data: StockData = serde_json::from_value(data.clone()).unwrap();
            (timestamp.to_string().clone(), data)
        }).collect();

        Ok(Stock {ticker: ticker.to_string(), data: stock_data })
    } else {
        Err(ApiError::AlphaVantageError) 
    }
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


