use std::collections::BTreeMap;
use serde_json;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Stock {
    pub ticker: String,
    pub data: BTreeMap<String, StockData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockData {
    #[serde(rename = "1. open")]
    open: f64, 
    #[serde(rename = "2. high")]
    high: f64,
    #[serde(rename = "3. low")]
    low: f64,
    #[serde(rename = "4. close")]
    close: f64,
    #[serde(rename = "5. volume")]
    volume: u32, 
}
















