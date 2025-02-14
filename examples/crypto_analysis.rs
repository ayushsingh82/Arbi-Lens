use social_ai_agent::{MarketAnalyzer, DefiAnalyzer, AiProjectAnalyzer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let market_analyzer = MarketAnalyzer::new();
    let defi_analyzer = DefiAnalyzer::new();
    let ai_analyzer = AiProjectAnalyzer::new();
    
    // Analyze categories
    let categories = vec!["ai", "defi", "l2"];
    
    for category in categories {
        match market_analyzer.analyze_category(category).await {
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

    // Analyze specific DeFi protocols
    let defi_protocols = vec!["aave", "uniswap", "compound"];
    for protocol in defi_protocols {
        if let Ok(analysis) = defi_analyzer.analyze_protocol(protocol).await {
            println!("\n{}", analysis);
        }
    }

    // Analyze AI projects
    let ai_projects = vec!["fetch-ai", "ocean-protocol", "singularitynet"];
    for project in ai_projects {
        if let Ok(analysis) = ai_analyzer.analyze_project(project).await {
            println!("\n{}", analysis);
        }
    }

    Ok(())
} 