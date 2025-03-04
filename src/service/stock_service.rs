use reqwest;
use std::collections::BTreeMap;
use serde_json::json;
use crate::models::stocks;
use crate::utils::config::Config;

use crate::models::stocks::{StockData, Stock};

const URL: &str = "https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol={TKR}&interval=5min&month={YEAR_MONTH}&outputsize=full&apikey={API_KEY}";


async fn query_api(ticker: &str, year_month: &str) -> Stock {
    let config = Config::new();
    let url = &URL
        .replace("{TKR}", ticker)
        .replace("{YEAR_MONTH}", year_month)
        .replace("{API_KEY}", &config.alpha_vantage_api_key);

    let parsed_data = 
        json!(reqwest::get(url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        .replace("\"", ""));

    let time_series = parsed_data["Time Series (5min)"].clone();
    let stock_data: BTreeMap<String, StockData> = serde_json::from_value(time_series).iter().map(|(timestamp, data): &(String, serde_json::Value)| {
        let data: StockData = serde_json::from_value(data.clone()).unwrap();
        (timestamp.to_string().clone(), data)
    }).collect();

    Stock {ticker: ticker.to_string(), data: stock_data }
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



