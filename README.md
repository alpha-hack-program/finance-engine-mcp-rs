# Finance Engine MCP Server

> **Advanced Model Context Protocol (MCP) Server providing seven sophisticated financial calculation functions for business intelligence and strategic decision-making**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)

A production-ready Model Context Protocol (MCP) server developed in Rust that provides seven strongly-typed financial calculation functions. This project demonstrates how to build enterprise-grade MCP servers with sophisticated multi-step calculations for financial analysis and business intelligence.

## Why This Finance Engine MCP Server?

Enterprises need to comply with regulations that require secure, on-premise data handling while leveraging AI capabilities. Small language models, while powerful, sometimes struggle with complex, multi-step financial logic requiring high reliability in regulated environments.

This Finance Engine provides:
- **Explicit, Verifiable Calculations**: All financial logic is transparent and auditable
- **Multi-Step Analytics**: Complex calculations that combine multiple financial dimensions
- **Enterprise-Ready**: Strong typing, comprehensive validation, and error handling
- **AI-Friendly**: Structured responses perfect for LLM consumption and interpretation

## ⚠️ **DISCLAIMER**

This server provides seven calculation functions that demonstrate sophisticated financial analysis patterns commonly used in business intelligence applications. All calculations are explicit and transparent.

**This is a demonstration/example project only.** The calculations and logic implemented here are for educational and demonstration purposes. This software:

- **Should NOT be used for actual financial or business decisions**
- **Does NOT represent real financial advice or calculations**
- **Is NOT affiliated with any official financial entity**
- **Serves as a technical example of MCP server implementation**

For real financial analysis or business decisions, please consult appropriate professional services.

## Introduction

The Finance Engine MCP Server provides sophisticated financial metrics calculation capabilities to AI agents through the Model Context Protocol. It implements seven critical business intelligence functions for enterprise-grade financial analysis:

- **Critical Business Metrics** - Company health scoring, revenue quality assessment, and concentration risk analysis
- **Operational Metrics** - Operating leverage and scalability assessment
- **Portfolio Analytics** - Revenue-weighted momentum, diversification, and organic growth analysis

## 🎯 Features

- **7 Financial Calculation Functions**: Comprehensive business intelligence metrics
- **Explicit Multi-Step Logic**: All calculations transparent and verifiable
- **Robust Input Validation**: JSON schema validation with detailed error handling
- **Multiple Transport Protocols**: STDIO, SSE, and Streamable HTTP
- **Containerization**: Production-ready Podman/Docker setup
- **Claude Desktop Integration**: MCPB packaging for seamless integration
- **Professional Metrics**: Prometheus metrics for monitoring
- **CI/CD Pipeline**: Comprehensive GitHub Actions workflow

## 📚 Quick Reference

| Task | Command | Description |
|------|---------|-------------|
| **🧪 Test** | `make test` | Run all tests |
| **🧪 Test SSE** | `make test-sse` | Run MCP server with SSE transport |
| **🧪 Test MCP** | `make test-mcp` | Run MCP server with Streamable HTTP transport |
| **🚀 Release** | `make release-patch` | Create new patch release |
| **📦 Package** | `make pack` | Create Claude Desktop package |
| **🐳 Container** | `make image-build` | Build container image |
| **ℹ️ Help** | `make help` | Show all commands |

## 📋 Available Functions

### Critical Business Metrics

| Function | Description | Key Output |
|----------|-------------|------------|
| **calculate_company_health_score** | Comprehensive 0-100 health score (5 dimensions) | Overall score, risk level, component breakdown |
| **calculate_revenue_quality_score** | Revenue sustainability analysis | Quality score (0.0-1.0), letter grade, recommendations |
| **calculate_hhi_and_diversification** | Revenue concentration risk assessment (HHI) | HHI index, diversification score, risk level |

### Operational Metrics

| Function | Description | Key Output |
|----------|-------------|------------|
| **calculate_operating_leverage** | Revenue vs cost growth scalability | Operating leverage ratio, margin expansion, efficiency rating |

### Portfolio Analytics

| Function | Description | Key Output |
|----------|-------------|------------|
| **calculate_portfolio_momentum** | Revenue-weighted portfolio growth | Portfolio momentum %, segment contributions, top contributor |
| **calculate_gini_coefficient** | Revenue concentration risk (Gini coefficient) | Gini coefficient, diversification score, concentration level |
| **calculate_organic_growth** | YoY organic growth (excl. M&A) | Organic growth rate, absolute growth, growth rating |

> **Note**: These functions implement sophisticated multi-step calculations combining multiple business dimensions.

## 📊 Function Details

### Function 1: calculate_company_health_score

**Purpose:** Calculates comprehensive company health by combining five weighted dimensions.

**Weights:**
- Revenue growth: 30%
- SLA compliance: 25%
- Modern revenue percentage (innovation): 20%
- Customer satisfaction: 15%
- Pipeline coverage: 10%

**Example:**
```json
{
  "revenue_growth": 0.09,
  "sla_compliance": 0.985,
  "modern_revenue_pct": 0.377,
  "customer_satisfaction": 89,
  "pipeline_coverage": 0.849
}
```

**Returns:**
- Overall score (0-100)
- Component scores
- Weighted contributions
- Risk level: LOW (80+), MEDIUM (65-79), HIGH (50-64), CRITICAL (<50)
- Interpretation

---

### Function 2: calculate_revenue_quality_score

**Purpose:** Evaluates revenue quality by categorizing into high-growth, stable, and declining segments.

**Quality Weights:**
- High-growth (>15% YoY): 1.0
- Stable (0-15% YoY): 0.7
- Declining (<0% YoY): 0.0

**Example:**
```json
{
  "high_growth_revenue": 15.0,
  "stable_revenue": 25.0,
  "declining_revenue": 10.0,
  "total_revenue": 50.0
}
```

**Returns:**
- Quality score (0.0-1.0)
- Distribution breakdown
- Letter grade (A-F)
- Strategic recommendation
- Gap to target (0.75 benchmark)

---

### Function 3: calculate_hhi_and_diversification

**Purpose:** Computes Herfindahl-Hirschman Index for revenue concentration risk.

**HHI Formula:** Sum of squared market shares

**Risk Thresholds:**
- LOW: HHI < 0.15
- MEDIUM: HHI 0.15-0.25
- HIGH: HHI > 0.25

**Example:**
```json
{
  "revenues": [15.0, 25.0, 5.0, 8.0]
}
```

**Returns:**
- HHI value
- Diversification score (1-HHI)
- Effective number of segments (1/HHI)
- Risk classification
- Market shares
- Concentration warnings

---

### Function 4: calculate_support_efficiency_score

**Purpose:** Measures support operations efficiency combining resolution and SLA metrics.

**Composition:**
- Resolution efficiency (40%): FCR (70%) + Time improvement (30%)
- SLA compliance (60%)

**Example:**
```json
{
  "fcr_current": 0.72,
  "fcr_prior": 0.68,
  "handling_time_current": 3.8,
  "handling_time_prior": 4.2,
  "sla_compliance": 0.985
}
```

**Returns:**
- Overall score (0-100)
- Component scores
- Performance grade (A-F)
- Year-over-year efficiency gains
- Component contributions

---

### Function 5: calculate_growth_attribution

**Purpose:** Attributes total growth to individual segments, identifying drivers and drags.

**Example:**
```json
{
  "segments": {
    "subscription": {"fy2024": 12.5, "fy2025": 15.0},
    "enterprise": {"fy2024": 22.0, "fy2025": 25.0}
  }
}
```

**Returns:**
- Total growth dollars
- Per-segment contributions (dollar, percentage, growth rate)
- Growth drivers (sorted descending)
- Growth drags (sorted ascending)
- Verification (sum = 100%)

---

### Function 6: calculate_segment_growth_analysis

**Purpose:** Compares modern vs traditional product growth to assess portfolio transformation.

**Interpretation:**
- Ratio > 3.0: Transformation successful
- Ratio 1.0-3.0: Transformation progressing
- Ratio < 1.0: Transformation failing

**Example:**
```json
{
  "modern_fy2024": 12.5,
  "modern_fy2025": 15.0,
  "traditional_fy2024": 37.5,
  "traditional_fy2025": 38.0
}
```

**Returns:**
- Modern growth rate
- Traditional growth rate
- Growth rate ratio
- Dollar gains by segment
- Transformation interpretation

---

### Function 7: calculate_operating_leverage

**Purpose:** Measures relationship between revenue growth and cost growth to assess operational scalability.

**Formula:** Operating Leverage = Revenue Growth Rate / Cost Growth Rate

**Efficiency Ratings:**
- Excellent: ≥ 1.5
- Good: 1.2 - 1.5
- Adequate: 1.0 - 1.2
- Poor: < 1.0

**Example:**
```json
{
  "revenue_growth_rate": 0.09,
  "cost_growth_rate": 0.06
}
```

**Returns:**
- Operating leverage ratio
- Revenue/cost growth percentages
- Margin expansion in basis points
- Efficiency rating
- Interpretation

---

### Function 8: calculate_portfolio_momentum

**Purpose:** Calculates revenue-weighted growth rate across business segments to measure overall portfolio momentum.

**Formula:** Σ(Segment Revenue / Total Revenue × Growth Rate)

**Momentum Ratings:**
- Strong: > 10%
- Moderate: 5% - 10%
- Weak: 0% - 5%
- Declining: < 0%

**Example:**
```json
{
  "segments": {
    "subscription": {"revenue": 15.0, "growth_rate": 0.20},
    "enterprise": {"revenue": 25.0, "growth_rate": 0.14},
    "upsell": {"revenue": 5.0, "growth_rate": 0.19},
    "legacy": {"revenue": 8.0, "growth_rate": -0.20}
  }
}
```

**Returns:**
- Portfolio momentum (decimal and percentage)
- Total revenue
- Per-segment contributions
- Top contributor
- Momentum rating

---

### Function 9: calculate_gini_coefficient

**Purpose:** Measures revenue distribution inequality using Gini coefficient for concentration risk assessment.

**Formula:** Gini = (2 × Σ(i × Revenue_i)) / (n × Σ(Revenue_i)) - (n + 1) / n

**Concentration Levels:**
- Low: Gini < 0.25 (well diversified)
- Moderate: Gini 0.25 - 0.40 (acceptable)
- High: Gini > 0.40 (risky)

**Example:**
```json
{
  "revenues": [15.0, 25.0, 5.0, 8.0]
}
```

**Returns:**
- Gini coefficient (0-1 scale)
- Diversification score (1 - Gini)
- Concentration level
- Largest/smallest segment shares
- Effective number of segments
- Sorted revenues

---

### Function 10: calculate_lifecycle_weighted_growth

**Purpose:** Calculates weighted growth rate by categorizing segments into lifecycle stages.

**Lifecycle Stages:**
- High Growth: growth > 15%
- Mature: growth 0% - 15%
- Declining: growth < 0%

**Portfolio Quality Ratings:**
- Excellent: >40% high growth, <20% declining
- Good: >30% high growth, <25% declining
- Fair: >20% high growth
- Poor: <20% high growth

**Example:**
```json
{
  "segments": {
    "subscription": {"revenue": 15.0, "growth_rate": 0.20},
    "enterprise": {"revenue": 25.0, "growth_rate": 0.14},
    "upsell": {"revenue": 5.0, "growth_rate": 0.19},
    "legacy": {"revenue": 8.0, "growth_rate": -0.20}
  }
}
```

**Returns:**
- Lifecycle-weighted growth (decimal and percentage)
- Stage breakdown (high_growth, mature, declining)
- Portfolio quality rating

---

### Function 11: calculate_organic_growth

**Purpose:** Calculates year-over-year organic revenue growth excluding acquisitions, divestitures, and other inorganic factors.

**Formula:** (Revenue Current - Revenue Prior) / Revenue Prior

**Growth Ratings:**
- Exceptional: > 15%
- Strong: 10% - 15%
- Moderate: 5% - 10%
- Weak: 0% - 5%
- Declining: < 0%

**Example:**
```json
{
  "revenue_prior": 48.7,
  "revenue_current": 53.0
}
```

**Returns:**
- Organic growth rate (decimal and percentage)
- Absolute dollar growth
- Prior/current revenue values
- Growth rating
- Annualized CAGR

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Cargo (included with Rust)
- `jq` for JSON processing ([Install jq](https://jqlang.github.io/jq/download/))
- `cargo-release` for version management: `cargo install cargo-release`
- NodeJS 19+ if testing with [MCP Inspector](https://modelcontextprotocol.io/docs/tools/inspector)

### 📥 Installation

```bash
# Clone the repository
git clone https://github.com/alpha-hack-program/finance-engine-mcp-rs.git
cd finance-engine-mcp-rs
```

### 🏗️ Build

```bash
# Build all servers
make build-all

# Or build individually
make build-sse      # SSE Server
make build-mcp      # MCP HTTP Server
make build-stdio    # STDIO Server for Claude
```

### 🧪 Unit Testing

```bash
# Run all tests
make test
```

### 🏃‍♂️ Running

> **NOTE:** By default `BIND_ADDRESS=127.0.0.1:8000` for **SSE** and `BIND_ADDRESS=127.0.0.1:8001` for **Streamable HTTP**

```bash
# SSE Server
make test-sse

# MCP Streamable HTTP Server
make test-mcp

# Or directly with custom address
RUST_LOG=info BIND_ADDRESS=127.0.0.1:8002 ./target/release/sse_server
```

### 🧪 Testing With MCP Inspector

Run the MCP server with SSE transport:

```bash
make test-sse
```

In another terminal, run MCP inspector:

```bash
make inspector
```

Open the URL provided in your browser and:
1. Set **Transport Type:** `SSE`
2. Set **URL:** `http://localhost:8002/sse`
3. Click `Connect`
4. Click `List Tools` to see all eleven functions
5. Select any function, fill parameters, and click `Run tool`

## 📦 Claude Desktop Integration

### Packaging

```bash
# Create MCPB package for Claude Desktop
make pack
```

This creates `finance-engine-mcp-server.mcpb` file.

### Installation

1. Open Claude Desktop
2. Go to Settings → Developer → Edit Config
3. Add the server configuration or drag and drop the `finance-engine-mcp-server.mcpb` file
4. Restart Claude Desktop

### Example Queries

Try asking Claude:

**Company Health:**
> "Calculate the company health score for a business with 8.83% revenue growth, 95% retention rate, operational score of 81.1, 37.7% modern revenue, CSAT of 89, and pipeline coverage of 0.849. What's their risk level?"

**Operating Leverage:**
> "Our revenue grew 9% while costs only grew 6%. Calculate our operating leverage and tell me what the margin expansion is in basis points."

**Portfolio Analysis:**
> "Calculate portfolio momentum for these segments: subscription ($15M, 20% growth), enterprise ($25M, 14% growth), upsell ($5M, 19% growth), and legacy ($8M, -20% growth). Which segment contributes most to momentum?"

**Concentration Risk:**
> "We have revenue of $15M, $25M, $5M, and $8M across four segments. Calculate the Gini coefficient and tell me if we have dangerous concentration risk."

**Organic Growth:**
> "Revenue grew from $48.7M to $53M year-over-year with no acquisitions. What's our organic growth rate?"

## 🔧 Configuration

### Environment Variables

```bash
# Logging level
RUST_LOG=info           

# Server bind address
BIND_ADDRESS=127.0.0.1:8000
```

## 🐳 Containerization

### Build and Run

```bash
# Build container image
scripts/image.sh build

# Run locally
scripts/image.sh run

# Run from remote registry
scripts/image.sh push
scripts/image.sh run-remote

# Show container information
scripts/image.sh info
```

### Production Configuration

```bash
podman run -p 8001:8001 \
  -e BIND_ADDRESS=0.0.0.0:8001 \
  -e RUST_LOG=info \
  quay.io/yourorg/finance-engine-mcp-server:latest
```

## 🛠️ Development

### Available Commands

#### 🏗️ Build Commands
```bash
make build-all              # Build all servers
make build-mcp              # Build MCP server
make build-sse              # Build SSE server
make build-stdio            # Build stdio server
make pack                   # Pack for Claude Desktop
```

#### 🚀 Release Commands (cargo-release)
```bash
make release-patch          # Patch release (1.0.0 → 1.0.1)
make release-minor          # Minor release (1.0.0 → 1.1.0)
make release-major          # Major release (1.0.0 → 2.0.0)
make release-dry-run        # Preview release changes
make sync-version           # Manually sync version
```

#### 🧪 Test Commands
```bash
make test                   # Run all tests
make test-sse               # Test SSE server
make test-mcp               # Test MCP server
```

#### 🔧 Development Commands
```bash
make clean                  # Clean build artifacts
make help                   # Show all commands
```

### Project Structure

```
├── src/                                    # Source code
│   ├── common/
│   │   ├── finance_engine.rs              # Core financial logic
│   │   ├── metrics.rs                     # Prometheus metrics
│   │   └── mod.rs
│   ├── sse_server.rs                      # SSE Server
│   ├── mcp_server.rs                      # MCP HTTP Server
│   └── stdio_server.rs                    # STDIO Server
├── scripts/                               # Utility scripts
│   ├── sync-manifest-version.sh           # Version sync
│   └── image.sh                           # Container management
├── mcpb/
│   └── manifest.json                      # Claude Desktop manifest
├── .github/workflows/                     # CI/CD pipelines
├── Containerfile                          # Container definition
├── Cargo.toml                             # Rust package manifest
└── Makefile                               # Build commands
```

## 💡 Usage Tips for LLM Integration

When querying an LLM with this MCP agent:

1. **Be specific with numbers** - Provide exact financial figures
2. **Include context** - Mention fiscal periods, business segments, etc.
3. **Ask for explanations** - Functions provide detailed breakdowns
4. **Combine calculations** - Use multiple functions for comprehensive analysis
5. **Use natural language** - No need to know exact API parameters
6. **Enterprise metrics** - Use Tier 3 functions for portfolio analysis and operational efficiency

## 🔒 Security

- **Input validation**: Strict JSON schemas and range checking
- **Non-root user**: Containers run as user `1001`
- **Security audit**: `cargo audit` in CI/CD
- **Minimal image**: Based on UBI 9 minimal
- **Sanitized errors**: Input sanitization prevents injection attacks

## 🤝 Contributing

### Development Workflow

1. **Fork the project**
2. **Create feature branch**: `git checkout -b feature/new-metric`
3. **Make changes and test**: `make test`
4. **Commit changes**: `git commit -am 'Add new metric'`
5. **Push to branch**: `git push origin feature/new-metric`
6. **Create Pull Request**

### Guidelines

- **Code Quality**: Follow `cargo fmt` and pass `cargo clippy`
- **Testing**: Add tests for new functionality
- **Version Management**: Let cargo-release handle versioning
- **CI/CD**: Ensure all GitHub Actions pass
- **Documentation**: Update README as needed

## ⚙️ Version Management

This project uses **cargo-release** for professional version management with automatic synchronization.

### Release Workflow

```bash
# 1. Make your changes and commit them
git add -A && git commit -m "feat: your changes"

# 2. Create a release
make release-patch     # Bug fixes: 1.0.0 → 1.0.1
make release-minor     # New features: 1.0.0 → 1.1.0  
make release-major     # Breaking changes: 1.0.0 → 2.0.0

# 3. Build and package
make pack
make image-build
make image-push

# 4. Push to repository
git push && git push --tags
```

## 📄 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) for details.

## 🙋 Support

- **Issues**: [GitHub Issues](https://github.com/alpha-hack-program/finance-engine-mcp-rs/issues)
- **Documentation**: [Project Wiki](https://github.com/alpha-hack-program/finance-engine-mcp-rs/wiki)
- **CI/CD**: Automated testing via GitHub Actions

## 🏷️ Tags

`mcp` `model-context-protocol` `rust` `finance-engine` `financial-analysis` `business-intelligence` `explicit-logic` `claude` `multi-step-calculations` `cargo-release` `enterprise-rust` `containerization` `ci-cd`

---

**Developed with ❤️ by [Alpha Hack Group](https://github.com/alpha-hack-program)**
