use std::env;
use dotenv::dotenv;

pub struct Config {
    pub alpha_vantage_api_key: String,
}

impl Config {
    pub fn new() -> Self {

        dotenv().ok();
        let alpha_vantage_api_key = env::var("alpha_vantage_api_key").expect("alpha_vantage_api_key");
        
        Config { alpha_vantage_api_key }
    }
}

