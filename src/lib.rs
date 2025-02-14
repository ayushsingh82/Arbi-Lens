use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::error::Error;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenData {
    price: f64,
    market_cap: f64,
    volume_24h: f64,
    price_change_24h: f64,
    price_change_7d: Option<f64>,
    category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryAnalysis {
    pub category: String,
    pub average_return_24h: f64,
    pub total_market_cap: f64,
    pub total_volume: f64,
    pub top_performers: Vec<String>,
    pub sentiment_score: f64,
}

#[derive(Debug)]
pub struct InvestmentStrategy {
    pub recommendation: String,
    pub confidence: f64,
    pub reasoning: String,
    pub risk_level: String,
    pub time_horizon: String,
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
            "compound".to_string(),
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
            if let Ok(data) = self.fetch_token_data(token).await {
                total_return += data.price_change_24h;
                total_market_cap += data.market_cap;
                total_volume += data.volume_24h;
                
                if data.price_change_24h > 5.0 {
                    top_performers.push(token.clone());
                }
            }
        }

        Ok(CategoryAnalysis {
            category: category.to_string(),
            average_return_24h: total_return / tokens.len() as f64,
            total_market_cap,
            total_volume,
            top_performers,
            sentiment_score: self.calculate_sentiment(category).await?,
        })
    }

    async fn fetch_token_data(&self, token_id: &str) -> Result<TokenData, Box<dyn Error>> {
        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd&include_24hr_vol=true&include_24hr_change=true&include_market_cap=true",
            token_id
        );

        let data = self.client
            .get(&url)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        // Parse response into TokenData
        // Add proper error handling here
        Ok(TokenData {
            price: 0.0, // Parse from response
            market_cap: 0.0,
            volume_24h: 0.0,
            price_change_24h: 0.0,
            price_change_7d: None,
            category: "".to_string(),
        })
    }

    async fn calculate_sentiment(&self, category: &str) -> Result<f64, Box<dyn Error>> {
        // Implement sentiment analysis logic
        Ok(0.7) // Placeholder
    }
} 