use reqwest;
use std::collections::BTreeMap;
use serde_json;
use crate::models::stocks;

use crate::models;

const URL: &str = "https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol={TKR}&interval=5min&month={YEAR_MONTH}&outputsize=full&apikey={API_KEY}";


async fn query_api() {

    let data = reqwest::get(URL).await.unwrap().text().await.unwrap();

    let ticker = "IBM";

    let stock_data: BTreeMap<String, stocks::StockData> = serde_json::from_str(data.as_str()).unwrap();

    let stock = stocks::Stock{ ticker, data: stock_data }; 
    

}






#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn api_query() {
        query_api().await;
    }
}















