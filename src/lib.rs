use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::error::Error;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenData {
    pub price: f64,
    pub market_cap: f64,
    pub volume_24h: f64,
    pub price_change_24h: f64,
}

#[derive(Debug)]
pub struct CategoryAnalysis {
    pub category: String,
    pub average_return_24h: f64,
    pub total_market_cap: f64,
    pub total_volume: f64,
    pub top_performers: Vec<String>,
    pub sentiment_score: f64,
}

pub struct MarketAnalyzer {
    client: Client,
    categories: HashMap<String, Vec<String>>,
}

impl MarketAnalyzer {
    pub fn new() -> Self {
        let mut categories = HashMap::new();
        categories.insert("ai".to_string(), vec![
            "fetch-ai".to_string(),
            "singularitynet".to_string(),
            "ocean-protocol".to_string(),
        ]);
        categories.insert("defi".to_string(), vec![
            "aave".to_string(),
            "uniswap".to_string(),
            "compound-governance-token".to_string(),
        ]);
        categories.insert("l2".to_string(), vec![
            "arbitrum".to_string(),
            "optimism".to_string(),
            "polygon".to_string(),
        ]);

        Self {
            client: Client::new(),
            categories,
        }
    }

    pub async fn analyze_category(&self, category: &str) -> Result<CategoryAnalysis, Box<dyn Error>> {
        let tokens = self.categories.get(category)
            .ok_or("Category not found")?;

        let mut total_return = 0.0;
        let mut total_market_cap = 0.0;
        let mut total_volume = 0.0;
        let mut top_performers = Vec::new();

        for token in tokens {
            match self.fetch_token_data(token).await {
                Ok(data) => {
                    total_return += data.price_change_24h;
                    total_market_cap += data.market_cap;
                    total_volume += data.volume_24h;
                    
                    if data.price_change_24h > 5.0 {
                        top_performers.push(token.clone());
                    }
                }
                Err(e) => println!("Error fetching data for {}: {}", token, e),
            }
        }

        let token_count = tokens.len() as f64;
        Ok(CategoryAnalysis {
            category: category.to_string(),
            average_return_24h: total_return / token_count,
            total_market_cap,
            total_volume,
            top_performers,
            sentiment_score: self.calculate_sentiment(category).await?,
        })
    }

    async fn fetch_token_data(&self, token_id: &str) -> Result<TokenData, Box<dyn Error>> {
        // Using CoinGecko API v3 without authentication for simplicity
        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd&include_market_cap=true&include_24hr_vol=true&include_24hr_change=true",
            token_id
        );

        let response = self.client.get(&url)
            .send()
            .await?
            .json::<HashMap<String, HashMap<String, f64>>>()
            .await?;

        let data = response.get(token_id)
            .ok_or("Token data not found")?;

        Ok(TokenData {
            price: *data.get("usd").unwrap_or(&0.0),
            market_cap: *data.get("usd_market_cap").unwrap_or(&0.0),
            volume_24h: *data.get("usd_24h_vol").unwrap_or(&0.0),
            price_change_24h: *data.get("usd_24h_change").unwrap_or(&0.0),
        })
    }

    async fn calculate_sentiment(&self, _category: &str) -> Result<f64, Box<dyn Error>> {
        // Simplified sentiment calculation
        Ok(0.75)
    }
}