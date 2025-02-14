

# Crypto Market Analysis AI Agent

An intelligent Rust-based analysis system that provides investment insights across different cryptocurrency categories (AI, DeFi, L2) using data from CoinGecko and DeFiLlama APIs.

## 🚀 Features

### 📊 Multi-Category Analysis

#### AI Projects
- Real-time tracking of AI-focused tokens (FETCH, OCEAN, AGIX)
- Github development activity monitoring
- Social sentiment analysis
- Technology adoption metrics

#### DeFi Protocols
- TVL (Total Value Locked) tracking
- APY/APR calculations
- Protocol revenue analysis
- Risk assessment metrics

#### Layer 2 Solutions
- Transaction volume analysis
- Gas efficiency metrics
- User adoption tracking
- Cross-chain comparisons

### 🔍 Data Sources
- CoinGecko API for market data
- DeFiLlama API for TVL metrics
- Github API for development activity
- Social media APIs for sentiment analysis

## 🛠 Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/social-ai-agent
cd social-ai-agent
```

2. Set up environment variables:
```bash
cp .env.example .env
# Add your API keys to .env
```

3. Install dependencies and build:
```bash
cargo build --release
```

## 💻 Usage

### Basic Market Analysis
```bash
cargo run --example crypto_analysis
```

### Custom Analysis
```rust
use social_ai_agent::{MarketAnalyzer, DefiAnalyzer, AiProjectAnalyzer};

#[tokio::main]
async fn main() {
    let market_analyzer = MarketAnalyzer::new();
    let analysis = market_analyzer.analyze_category("ai").await;
    println!("{:?}", analysis);
}
```

## ⚙️ Configuration

### Required API Keys:
```env
COINGECKO_API_KEY=your_api_key_here
DEFILAMA_API_KEY=your_api_key_here
GITHUB_API_KEY=your_github_api_key_here
```

### Analyzed Categories:
- AI Tokens: FETCH, OCEAN, AGIX
- DeFi: AAVE, UNI, COMP
- L2: ARB, OP, MATIC

## 📁 Project Structure
```
social-ai-agent/
├── src/
│   ├── lib.rs           # Core functionality
│   ├── defi_analyzer.rs # DeFi-specific analysis
│   └── ai_analyzer.rs   # AI project analysis
├── examples/
│   └── crypto_analysis.rs
└── .env
```

## 🔮 Roadmap

- [ ] Machine Learning prediction models
- [ ] Real-time market alerts
- [ ] Custom strategy builder
- [ ] Portfolio optimization
- [ ] Risk assessment metrics
- [ ] Automated trading signals

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](licenses/Apache-2.0) file for details.

## ⚠️ Disclaimer

This tool is for informational purposes only. Not financial advice. Always DYOR (Do Your Own Research) before making investment decisions.

## 🔗 Dependencies

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dotenv = "0.15"
async-trait = "0.1"
chrono = "0.4"
```

## 🤔 Support

For support, please open an issue in the repository or contact the maintainers.
