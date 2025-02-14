use social_ai_agent::MarketAnalyzer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let analyzer = MarketAnalyzer::new();
    
    // Analyze multiple categories
    let categories = vec!["ai", "defi", "l2"];
    
    for category in categories {
        match analyzer.analyze_category(category).await {
            Ok(analysis) => {
                println!("\nCategory Analysis: {}", category);
                println!("Average 24h Return: {:.2}%", analysis.average_return_24h);
                println!("Total Market Cap: ${:.2}B", analysis.total_market_cap / 1_000_000_000.0);
                println!("Top Performers: {:?}", analysis.top_performers);
                println!("Sentiment Score: {:.2}", analysis.sentiment_score);
                println!("-------------------");
            }
            Err(e) => println!("Error analyzing {}: {}", category, e),
        }
    }

    Ok(())
} 