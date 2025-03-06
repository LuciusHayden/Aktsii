use std::collections::BTreeMap;
use serde_json;
use serde::{Serialize, Deserialize, Deserializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct Stock {
    pub ticker: String,
    pub data: BTreeMap<String, StockData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockData {
    #[serde(rename = "1. open", deserialize_with = "de_string_to_f64")]
    open: f64, 
    #[serde(rename = "2. high", deserialize_with = "de_string_to_f64")]
    high: f64,
    #[serde(rename = "3. low", deserialize_with = "de_string_to_f64")]
    low: f64,
    #[serde(rename = "4. close", deserialize_with = "de_string_to_f64")]
    close: f64,
    #[serde(rename = "5. volume", deserialize_with = "de_string_to_u64")]
    volume: u64, 
}

#[derive(Serialize, Deserialize)]
pub struct Query{
    pub ticker: String,
}

fn de_string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}


fn de_string_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<u64>().map_err(serde::de::Error::custom)
}
