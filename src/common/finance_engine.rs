use serde::{Deserialize, Deserializer, Serialize, de};
use std::collections::HashMap;
use std::fmt;

use super::metrics::{increment_requests, increment_errors, RequestTimer};

use rmcp::{
    ServerHandler,
    handler::server::router::tool::ToolRouter,
    handler::server::wrapper::Parameters,
    model::{ServerCapabilities, ServerInfo, CallToolResult, Content},
    ErrorData as McpError,
    schemars, tool, tool_handler, tool_router
};

// =================== PARSING UTILITIES ===================

/// Sanitize user input for safe inclusion in error messages
fn sanitize_for_error_message(input: &str) -> String {
    let truncated = if input.len() > 50 { 
        format!("{}...", &input[..47])
    } else { 
        input.to_string() 
    };
    
    truncated
        .chars()
        .map(|c| match c {
            '\n' | '\r' | '\t' => ' ',
            '"' | '\'' | '`' => '?',
            '\\' => '?',
            '<' | '>' => '?',
            c if c.is_ascii_graphic() || c == ' ' => c,
            _ => '?'
        })
        .collect()
}

/// Validate input length and format for security
fn validate_input_security(input: &str, field_name: &str) -> Result<(), String> {
    if input.len() > 100 {
        return Err(format!("Invalid {}: input too long (max 100 characters)", field_name));
    }
    
    if input.contains('\0') {
        return Err(format!("Invalid {}: input contains null bytes", field_name));
    }
    
    let control_char_count = input.chars().filter(|c| c.is_control()).count();
    if control_char_count > 2 {
        return Err(format!("Invalid {}: input contains too many control characters", field_name));
    }
    
    Ok(())
}

/// Parse a string to f64
fn parse_f64_from_string(s: &str) -> Result<f64, String> {
    let trimmed = s.trim();
    
    if let Err(e) = validate_input_security(trimmed, "number") {
        return Err(e);
    }
    
    if trimmed.is_empty() {
        return Err("Empty string cannot be parsed as number".to_string());
    }
    
    let sanitized = sanitize_for_error_message(trimmed);
    
    let cleaned = trimmed
        .replace(',', "")
        .replace('$', "")
        .replace('€', "")
        .replace('£', "")
        .replace('¥', "")
        .replace('%', "");
    
    match cleaned.parse::<f64>() {
        Ok(value) => {
            if value.is_infinite() || value.is_nan() {
                Err(format!("Invalid number: '{}'", sanitized))
            } else {
                Ok(value)
            }
        },
        Err(_) => Err(format!("Cannot parse '{}' as a number", sanitized))
    }
}

// =================== CUSTOM DESERIALIZERS ===================

/// Custom deserializer that accepts both f64 numbers and strings
fn deserialize_flexible_f64<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct FlexibleF64Visitor;

    impl<'de> de::Visitor<'de> for FlexibleF64Visitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number or a string representing a number")
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value)
        }
    }

    deserializer.deserialize_any(FlexibleF64Visitor)
}

// =================== DATA STRUCTURES ===================

// Function: calculate_company_health_score
#[derive(Debug, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub struct CompanyHealthScoreParams {
    #[serde(deserialize_with = "deserialize_flexible_f64")]
    #[schemars(description = "Year-over-year revenue growth rate as decimal (e.g., 0.09 for 9%)")]
    pub revenue_growth: String,
    #[serde(deserialize_with = "deserialize_flexible_f64")]
    #[schemars(description = "Service Level Agreement compliance rate as decimal (e.g., 0.985 for 98.5%)")]
    pub sla_compliance: String,
    #[serde(deserialize_with = "deserialize_flexible_f64")]
    #[schemars(description = "Percentage of revenue from subscription/recurring revenue streams as decimal (e.g., 0.377 for 37.7%)")]
    pub modern_revenue_pct: String,
    #[serde(deserialize_with = "deserialize_flexible_f64")]
    #[schemars(description = "Customer satisfaction score on 0-100 scale")]
    pub customer_satisfaction: String,
    #[serde(deserialize_with = "deserialize_flexible_f64")]
    #[schemars(description = "Ratio of active sales pipeline value to annual revenue as decimal (e.g., 0.849 for 84.9%)")]
    pub pipeline_coverage: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub struct CompanyHealthScoreResponse {
    #[schemars(description = "Composite health score 0-100")]
    pub overall_score: f64,
    #[schemars(description = "Individual dimension scores before weighting")]
    pub components: HashMap<String, f64>,
    #[schemars(description = "Point contribution of each dimension to final score")]
    pub weighted_contributions: HashMap<String, f64>,
    #[schemars(description = "Risk level: LOW, MEDIUM, HIGH, or CRITICAL")]
    pub risk_level: String,
    #[schemars(description = "Human-readable assessment of health status")]
    pub interpretation: String,
}

// Function: calculate_revenue_quality_score
#[derive(Debug, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub struct RevenueQualityScoreParams {
    #[serde(deserialize_with = "deserialize_flexible_f64")]
    #[schemars(description = "Dollar amount of revenue growing above 15% year-over-year")]
    pub high_growth_revenue: String,
    #[serde(deserialize_with = "deserialize_flexible_f64")]
    #[schemars(description = "Dollar amount of revenue growing 0-15% year-over-year")]
    pub stable_revenue: String,
    #[serde(deserialize_with = "deserialize_flexible_f64")]
    #[schemars(description = "Dollar amount of revenue with negative year-over-year growth")]
    pub declining_revenue: String,
    #[serde(deserialize_with = "deserialize_flexible_f64")]
    #[schemars(description = "Total company revenue for normalization")]
    pub total_revenue: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub struct RevenueQualityScoreResponse {
    #[schemars(description = "Composite quality score 0.0-1.0 scale where 1.0 is perfect")]
    pub quality_score: f64,
    #[schemars(description = "Percentage breakdown of revenue by growth category")]
    pub distribution: HashMap<String, f64>,
    #[schemars(description = "Letter grade A through F based on quality score")]
    pub grade: String,
    #[schemars(description = "Actionable strategic guidance based on score")]
    pub recommendation: String,
    #[schemars(description = "Industry benchmark for comparison")]
    pub target_score: f64,
    #[schemars(description = "Distance from benchmark, negative means exceeding target")]
    pub gap_to_target: f64,
}

// Function: calculate_hhi_and_diversification
#[derive(Debug, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub struct HHIParams {
    #[schemars(description = "Revenue values for each business segment")]
    pub revenues: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub struct HHIResponse {
    #[schemars(description = "Herfindahl-Hirschman Index value 0.0-1.0")]
    pub hhi: f64,
    #[schemars(description = "Inverse of HHI, where higher means more diversified")]
    pub diversification_score: f64,
    #[schemars(description = "Effective number of equal-sized segments")]
    pub effective_n: f64,
    #[schemars(description = "Risk level: LOW, MEDIUM, or HIGH")]
    pub risk_level: String,
    #[schemars(description = "Risk interpretation in plain language")]
    pub assessment: String,
    #[schemars(description = "Individual segment shares as decimals")]
    pub market_shares: Vec<f64>,
    #[schemars(description = "Highest individual segment share")]
    pub largest_share: f64,
    #[schemars(description = "Specific warnings about concentration risks")]
    pub concentration_issues: Vec<String>,
}

// Function: calculate_operating_leverage
#[derive(Debug, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub struct OperatingLeverageParams {
    #[serde(deserialize_with = "deserialize_flexible_f64")]
    #[schemars(description = "Year-over-year revenue growth rate as decimal (e.g., 0.09 for 9%)")]
    pub revenue_growth_rate: String,
    #[serde(deserialize_with = "deserialize_flexible_f64")]
    #[schemars(description = "Year-over-year operating cost growth rate as decimal (e.g., 0.06 for 6%)")]
    pub cost_growth_rate: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub struct OperatingLeverageResponse {
    #[schemars(description = "Operating leverage ratio (revenue growth / cost growth)")]
    pub operating_leverage: f64,
    #[schemars(description = "Revenue growth rate as percentage")]
    pub revenue_growth_pct: f64,
    #[schemars(description = "Cost growth rate as percentage")]
    pub cost_growth_pct: f64,
    #[schemars(description = "Margin expansion in basis points")]
    pub margin_expansion_bps: f64,
    #[schemars(description = "Efficiency rating: Excellent, Good, Adequate, or Poor")]
    pub efficiency_rating: String,
    #[schemars(description = "Plain language interpretation of the leverage")]
    pub interpretation: String,
}

// Function: calculate_portfolio_momentum
#[derive(Debug, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub struct PortfolioSegmentData {
    #[schemars(description = "Segment revenue in millions")]
    pub revenue: f64,
    #[schemars(description = "Year-over-year growth rate as decimal (e.g., 0.20 for 20%)")]
    pub growth_rate: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub struct PortfolioMomentumParams {
    #[schemars(description = "Dictionary of segment names to revenue and growth rate data")]
    pub segments: HashMap<String, PortfolioSegmentData>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub struct SegmentMomentumContribution {
    #[schemars(description = "Segment revenue")]
    pub revenue: f64,
    #[schemars(description = "Segment revenue as percentage of total")]
    pub revenue_pct: f64,
    #[schemars(description = "Segment growth rate as percentage")]
    pub growth_rate: f64,
    #[schemars(description = "Contribution to overall momentum as percentage")]
    pub contribution_to_momentum: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub struct PortfolioMomentumResponse {
    #[schemars(description = "Portfolio momentum as decimal")]
    pub portfolio_momentum: f64,
    #[schemars(description = "Portfolio momentum as percentage")]
    pub portfolio_momentum_pct: f64,
    #[schemars(description = "Total revenue across all segments")]
    pub total_revenue: f64,
    #[schemars(description = "Individual segment contributions to momentum")]
    pub segment_contributions: HashMap<String, SegmentMomentumContribution>,
    #[schemars(description = "Name of segment contributing most to momentum")]
    pub top_contributor: String,
    #[schemars(description = "Momentum rating: Strong, Moderate, Weak, or Declining")]
    pub momentum_rating: String,
}

// Function: calculate_gini_coefficient
#[derive(Debug, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub struct GiniCoefficientParams {
    #[schemars(description = "List of revenue values by segment (any order)")]
    pub revenues: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub struct GiniCoefficientResponse {
    #[schemars(description = "Gini coefficient (0-1 scale, higher = more concentrated)")]
    pub gini_coefficient: f64,
    #[schemars(description = "Diversification score (1 - Gini, higher = more diversified)")]
    pub diversification_score: f64,
    #[schemars(description = "Concentration level: Low, Moderate, or High")]
    pub concentration_level: String,
    #[schemars(description = "Largest segment share as percentage")]
    pub largest_segment_share: f64,
    #[schemars(description = "Smallest segment share as percentage")]
    pub smallest_segment_share: f64,
    #[schemars(description = "Effective number of equal-sized segments")]
    pub effective_segments: f64,
    #[schemars(description = "Revenue values sorted in ascending order")]
    pub sorted_revenues: Vec<f64>,
}

// Function 11: calculate_organic_growth
#[derive(Debug, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub struct OrganicGrowthParams {
    #[serde(deserialize_with = "deserialize_flexible_f64")]
    #[schemars(description = "Revenue from prior period")]
    pub revenue_prior: String,
    #[serde(deserialize_with = "deserialize_flexible_f64")]
    #[schemars(description = "Revenue from current period")]
    pub revenue_current: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, schemars::JsonSchema)]
pub struct OrganicGrowthResponse {
    #[schemars(description = "Organic growth rate as decimal")]
    pub organic_growth_rate: f64,
    #[schemars(description = "Organic growth rate as percentage")]
    pub organic_growth_pct: f64,
    #[schemars(description = "Absolute dollar growth")]
    pub absolute_growth: f64,
    #[schemars(description = "Prior period revenue")]
    pub revenue_prior: f64,
    #[schemars(description = "Current period revenue")]
    pub revenue_current: f64,
    #[schemars(description = "Growth rating: Exceptional, Strong, Moderate, Weak, or Declining")]
    pub growth_rating: String,
    #[schemars(description = "Annualized CAGR as percentage")]
    pub annualized_cagr: f64,
}

// =================== FINANCE ENGINE ===================

#[derive(Debug, Clone)]
pub struct FinanceEngine {
    tool_router: ToolRouter<Self>,
}

impl FinanceEngine {
    /// Calculate company health score checked [√]
    fn calculate_company_health_score_internal(
        revenue_growth: f64,
        sla_compliance: f64,
        modern_revenue_pct: f64,
        customer_satisfaction: f64,
        pipeline_coverage: f64,
    ) -> Result<CompanyHealthScoreResponse, String> {
        // Validation
        if sla_compliance < 0.0 || sla_compliance > 1.0 {
            return Err("SLA compliance must be between 0.0 and 1.0".to_string());
        }
        if modern_revenue_pct < 0.0 || modern_revenue_pct > 1.0 {
            return Err("Modern revenue percentage must be between 0.0 and 1.0".to_string());
        }
        if customer_satisfaction < 0.0 || customer_satisfaction > 100.0 {
            return Err("Customer satisfaction must be between 0.0 and 100.0".to_string());
        }
        if pipeline_coverage < 0.0 {
            return Err("Pipeline coverage must be >= 0.0".to_string());
        }

        // Convert to 0-100 scale
        // Revenue Growth: 0% growth = 0 points, 15%+ growth = 100 points
        let revenue_score = ((revenue_growth / 0.15) * 100.0).min(100.0).max(0.0);
        
        // Service Level Agreement Compliance: Direct percentage conversion
        let sla_score = sla_compliance * 100.0;
        
        // Modern Revenue Percentage: Direct percentage conversion
        let innovation_score = modern_revenue_pct * 100.0;
        
        // Customer Satisfaction: Already 0-100, use as-is
        let satisfaction_score = customer_satisfaction;
        
        // Pipeline Coverage: 0% coverage = 0 points, 100%+ coverage = 100 points
        let pipeline_score = (pipeline_coverage * 100.0).min(100.0);

        let mut components = HashMap::new();
        components.insert("revenue".to_string(), revenue_score);
        components.insert("sla".to_string(), sla_score);
        components.insert("innovation".to_string(), innovation_score);
        components.insert("satisfaction".to_string(), satisfaction_score);
        components.insert("pipeline".to_string(), pipeline_score);

        // Apply weights
        let weights = [
            ("revenue", 0.30),
            ("sla", 0.25),
            ("innovation", 0.20),
            ("satisfaction", 0.15),
            ("pipeline", 0.10),
        ];

        let mut weighted_contributions = HashMap::new();
        let mut overall_score = 0.0;

        for (name, weight) in weights.iter() {
            let contribution = components[*name] * weight;
            weighted_contributions.insert(name.to_string(), contribution);
            overall_score += contribution;
        }

        // Classify risk
        let (risk_level, interpretation) = if overall_score >= 80.0 {
            ("LOW", "Company health is excellent across all dimensions.")
        } else if overall_score >= 65.0 {
            ("MEDIUM", "Company health is good but some areas need attention for optimal performance.")
        } else if overall_score >= 50.0 {
            ("HIGH", "Company faces significant challenges in multiple areas requiring strategic intervention.")
        } else {
            ("CRITICAL", "Company health is critical with severe issues across key performance indicators.")
        };

        Ok(CompanyHealthScoreResponse {
            overall_score,
            components,
            weighted_contributions,
            risk_level: risk_level.to_string(),
            interpretation: interpretation.to_string(),
        })
    }

    /// Calculate revenue quality score
    fn calculate_revenue_quality_score_internal(
        high_growth_revenue: f64,
        stable_revenue: f64,
        declining_revenue: f64,
        total_revenue: f64,
    ) -> Result<RevenueQualityScoreResponse, String> {
        // Validation
        if high_growth_revenue < 0.0 || stable_revenue < 0.0 || declining_revenue < 0.0 || total_revenue <= 0.0 {
            return Err("All revenue amounts must be non-negative and total must be positive".to_string());
        }

        let sum = high_growth_revenue + stable_revenue + declining_revenue;
        if (sum - total_revenue).abs() > 0.01 * total_revenue {
            return Err("Revenue categories must sum to total revenue".to_string());
        }

        // Calculate distribution
        let high_growth_pct = high_growth_revenue / total_revenue;
        let stable_pct = stable_revenue / total_revenue;
        let declining_pct = declining_revenue / total_revenue;

        let mut distribution = HashMap::new();
        distribution.insert("high_growth".to_string(), high_growth_pct * 100.0);
        distribution.insert("stable".to_string(), stable_pct * 100.0);
        distribution.insert("declining".to_string(), declining_pct * 100.0);

        // Calculate quality score with weights
        let quality_score = (high_growth_pct * 1.0) + (stable_pct * 0.7) + (declining_pct * 0.0);

        // Assign grade
        let grade = if quality_score >= 0.80 {
            "A"
        } else if quality_score >= 0.65 {
            "B"
        } else if quality_score >= 0.50 {
            "C"
        } else if quality_score >= 0.35 {
            "D"
        } else {
            "F"
        };

        // Generate recommendation
        let recommendation = match grade {
            "A" => "Excellent revenue quality. Continue investing in high-growth segments and maintain momentum.",
            "B" => "Good revenue quality with room for improvement. Focus on accelerating growth in stable segments.",
            "C" => "Moderate revenue quality. Strategic pivot needed to increase high-growth revenue proportion.",
            "D" => "Poor revenue quality. Urgent action required to address declining revenue and stimulate growth.",
            _ => "Critical revenue quality issues. Immediate restructuring needed to reverse declining trends.",
        };

        let target_score = 0.75;
        let gap_to_target = quality_score - target_score;

        Ok(RevenueQualityScoreResponse {
            quality_score,
            distribution,
            grade: grade.to_string(),
            recommendation: recommendation.to_string(),
            target_score,
            gap_to_target,
        })
    }

    /// Calculate HHI and diversification checked [√]
    fn calculate_hhi_and_diversification_internal(revenues: Vec<f64>) -> Result<HHIResponse, String> {
        if revenues.len() < 2 {
            return Err("Must contain at least 2 segments".to_string());
        }

        for (i, &rev) in revenues.iter().enumerate() {
            if rev < 0.0 {
                return Err(format!("Revenue at index {} cannot be negative", i));
            }
        }

        let total: f64 = revenues.iter().sum();
        if total <= 0.0 {
            return Err("Total revenue must be positive".to_string());
        }

        // Calculate market shares
        let market_shares: Vec<f64> = revenues.iter().map(|r| r / total).collect();
        
        // Calculate HHI
        let hhi: f64 = market_shares.iter().map(|s| s * s).sum();
        
        let diversification_score = 1.0 - hhi;
        let effective_n = 1.0 / hhi;
        let largest_share = market_shares.iter().cloned().fold(0.0, f64::max);

        // Determine risk level
        let risk_level = if hhi < 0.15 {
            "LOW"
        } else if hhi <= 0.25 {
            "MEDIUM"
        } else {
            "HIGH"
        };

        let assessment = format!(
            "Revenue concentration is {} with HHI of {:.3}. The portfolio behaves like {:.1} equal-sized segments.",
            risk_level.to_lowercase(), hhi, effective_n
        );

        // Identify concentration issues
        let mut concentration_issues = Vec::new();
        if largest_share > 0.50 {
            concentration_issues.push(format!("Single segment dominance: {:.1}% of revenue", largest_share * 100.0));
        }
        if hhi > 0.35 {
            concentration_issues.push("HHI exceeds 0.35 indicating severe concentration".to_string());
        }
        if effective_n < 3.0 {
            concentration_issues.push(format!("Effective segment count ({:.1}) is below recommended minimum of 3", effective_n));
        }

        Ok(HHIResponse {
            hhi,
            diversification_score,
            effective_n,
            risk_level: risk_level.to_string(),
            assessment,
            market_shares,
            largest_share,
            concentration_issues,
        })
    }

    /// Calculate operating leverage ratio checked [√]
    fn calculate_operating_leverage_internal(
        revenue_growth_rate: f64,
        cost_growth_rate: f64,
    ) -> Result<OperatingLeverageResponse, String> {
        // Validation
        if cost_growth_rate == 0.0 {
            return Err("Cost growth rate cannot be zero".to_string());
        }

        let operating_leverage = revenue_growth_rate / cost_growth_rate;
        let margin_expansion_bps = (revenue_growth_rate - cost_growth_rate) * 10000.0;

        let efficiency_rating = if operating_leverage >= 1.5 {
            "Excellent"
        } else if operating_leverage >= 1.2 {
            "Good"
        } else if operating_leverage >= 1.0 {
            "Adequate"
        } else {
            "Poor"
        };

        let interpretation = format!("Revenue growing {:.1}x faster than costs", operating_leverage);

        Ok(OperatingLeverageResponse {
            operating_leverage: (operating_leverage * 100.0).round() / 100.0,
            revenue_growth_pct: (revenue_growth_rate * 1000.0).round() / 10.0,
            cost_growth_pct: (cost_growth_rate * 1000.0).round() / 10.0,
            margin_expansion_bps: margin_expansion_bps.round(),
            efficiency_rating: efficiency_rating.to_string(),
            interpretation,
        })
    }

    /// Calculate portfolio momentum index checked [√]
    fn calculate_portfolio_momentum_internal(
        segments: HashMap<String, PortfolioSegmentData>,
    ) -> Result<PortfolioMomentumResponse, String> {
        if segments.is_empty() {
            return Err("Segments cannot be empty".to_string());
        }

        let total_revenue: f64 = segments.values().map(|s| s.revenue).sum();

        if total_revenue == 0.0 {
            return Err("Total revenue cannot be zero".to_string());
        }

        let mut momentum = 0.0;
        let mut segment_contributions = HashMap::new();
        let mut max_contribution = 0.0;
        let mut top_contributor = String::new();

        for (name, data) in segments.iter() {
            let weight = data.revenue / total_revenue;
            let contribution = weight * data.growth_rate;
            momentum += contribution;

            let contrib_pct = contribution * 100.0;
            if contrib_pct > max_contribution {
                max_contribution = contrib_pct;
                top_contributor = name.clone();
            }

            segment_contributions.insert(
                name.clone(),
                SegmentMomentumContribution {
                    revenue: (data.revenue * 100.0).round() / 100.0,
                    revenue_pct: (weight * 1000.0).round() / 10.0,
                    growth_rate: (data.growth_rate * 1000.0).round() / 10.0,
                    contribution_to_momentum: (contrib_pct * 100.0).round() / 100.0,
                },
            );
        }

        let momentum_rating = if momentum > 0.10 {
            "Strong"
        } else if momentum > 0.05 {
            "Moderate"
        } else if momentum > 0.0 {
            "Weak"
        } else {
            "Declining"
        };

        Ok(PortfolioMomentumResponse {
            portfolio_momentum: (momentum * 10000.0).round() / 10000.0,
            portfolio_momentum_pct: (momentum * 10000.0).round() / 100.0,
            total_revenue: (total_revenue * 100.0).round() / 100.0,
            segment_contributions,
            top_contributor,
            momentum_rating: momentum_rating.to_string(),
        })
    }

    /// Calculate Gini coefficient for revenue concentration checked [√]
    fn calculate_gini_coefficient_internal(revenues: Vec<f64>) -> Result<GiniCoefficientResponse, String> {
        if revenues.is_empty() {
            return Err("Revenue list cannot be empty".to_string());
        }

        for rev in revenues.iter() {
            if *rev < 0.0 {
                return Err("Revenues cannot be negative".to_string());
            }
        }

        let total_revenue: f64 = revenues.iter().sum();
        if total_revenue == 0.0 {
            return Err("Total revenue cannot be zero".to_string());
        }

        let mut sorted_revenues = revenues.clone();
        sorted_revenues.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let n = sorted_revenues.len() as f64;
        let cumsum: f64 = sorted_revenues
            .iter()
            .enumerate()
            .map(|(i, &rev)| (i as f64 + 1.0) * rev)
            .sum();

        let gini = (2.0 * cumsum) / (n * total_revenue) - (n + 1.0) / n;
        let diversification_score = 1.0 - gini;

        let largest_share = revenues.iter().cloned().fold(0.0, f64::max) / total_revenue * 100.0;
        let smallest_share = revenues.iter().cloned().fold(f64::INFINITY, f64::min) / total_revenue * 100.0;

        let effective_segments = if gini > 0.0 {
            1.0 / (gini + 0.0001)
        } else {
            n
        };

        let concentration_level = if gini < 0.25 {
            "Low"
        } else if gini < 0.40 {
            "Moderate"
        } else {
            "High"
        };

        let sorted_revenues_rounded: Vec<f64> = sorted_revenues
            .iter()
            .map(|r| (r * 100.0).round() / 100.0)
            .collect();

        Ok(GiniCoefficientResponse {
            gini_coefficient: (gini * 1000.0).round() / 1000.0,
            diversification_score: (diversification_score * 1000.0).round() / 1000.0,
            concentration_level: concentration_level.to_string(),
            largest_segment_share: (largest_share * 10.0).round() / 10.0,
            smallest_segment_share: (smallest_share * 10.0).round() / 10.0,
            effective_segments: (effective_segments * 100.0).round() / 100.0,
            sorted_revenues: sorted_revenues_rounded,
        })
    }

    /// Calculate organic growth rate checked [√]
    fn calculate_organic_growth_internal(
        revenue_prior: f64,
        revenue_current: f64,
    ) -> Result<OrganicGrowthResponse, String> {
        if revenue_prior <= 0.0 {
            return Err("Prior period revenue must be positive".to_string());
        }

        let absolute_growth = revenue_current - revenue_prior;
        let growth_rate = absolute_growth / revenue_prior;

        let growth_rating = if growth_rate > 0.15 {
            "Exceptional"
        } else if growth_rate > 0.10 {
            "Strong"
        } else if growth_rate > 0.05 {
            "Moderate"
        } else if growth_rate > 0.0 {
            "Weak"
        } else {
            "Declining"
        };

        Ok(OrganicGrowthResponse {
            organic_growth_rate: (growth_rate * 10000.0).round() / 10000.0,
            organic_growth_pct: (growth_rate * 10000.0).round() / 100.0,
            absolute_growth: (absolute_growth * 100.0).round() / 100.0,
            revenue_prior: (revenue_prior * 100.0).round() / 100.0,
            revenue_current: (revenue_current * 100.0).round() / 100.0,
            growth_rating: growth_rating.to_string(),
            annualized_cagr: (growth_rate * 10000.0).round() / 100.0,
        })
    }
}

#[tool_router]
impl FinanceEngine {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Calculate comprehensive company health score (0-100) by combining five weighted dimensions: revenue growth (30%), Service Level Agreement compliance (25%), modern revenue percentage (20%), customer satisfaction (15%), and pipeline coverage (10%). Returns overall score, individual components, weighted contributions, risk level classification (LOW/MEDIUM/HIGH/CRITICAL), and interpretation.")]
    pub async fn calculate_company_health_score(
        &self,
        Parameters(params): Parameters<CompanyHealthScoreParams>,
    ) -> Result<CallToolResult, McpError> {
        let _timer = RequestTimer::new();
        increment_requests();

        // Parse parameters
        let revenue_growth = match parse_f64_from_string(&params.revenue_growth) {
            Ok(v) => v,
            Err(e) => {
                increment_errors();
                return Ok(CallToolResult::error(vec![Content::text(format!("Invalid revenue_growth: {}", e))]));
            }
        };

        let sla_compliance = match parse_f64_from_string(&params.sla_compliance) {
            Ok(v) => v,
            Err(e) => {
                increment_errors();
                return Ok(CallToolResult::error(vec![Content::text(format!("Invalid sla_compliance: {}", e))]));
            }
        };

        let modern_revenue_pct = match parse_f64_from_string(&params.modern_revenue_pct) {
            Ok(v) => v,
            Err(e) => {
                increment_errors();
                return Ok(CallToolResult::error(vec![Content::text(format!("Invalid modern_revenue_pct: {}", e))]));
            }
        };

        let customer_satisfaction = match parse_f64_from_string(&params.customer_satisfaction) {
            Ok(v) => v,
            Err(e) => {
                increment_errors();
                return Ok(CallToolResult::error(vec![Content::text(format!("Invalid customer_satisfaction: {}", e))]));
            }
        };

        let pipeline_coverage = match parse_f64_from_string(&params.pipeline_coverage) {
            Ok(v) => v,
            Err(e) => {
                increment_errors();
                return Ok(CallToolResult::error(vec![Content::text(format!("Invalid pipeline_coverage: {}", e))]));
            }
        };

        match Self::calculate_company_health_score_internal(
            revenue_growth,
            sla_compliance,
            modern_revenue_pct,
            customer_satisfaction,
            pipeline_coverage,
        ) {
            Ok(result) => match serde_json::to_string_pretty(&result) {
                Ok(json_str) => Ok(CallToolResult::success(vec![Content::text(json_str)])),
                Err(e) => {
                    increment_errors();
                    Ok(CallToolResult::error(vec![Content::text(format!("Serialization error: {}", e))]))
                }
            },
            Err(e) => {
                increment_errors();
                Ok(CallToolResult::error(vec![Content::text(format!("Calculation error: {}", e))]))
            }
        }
    }

    #[tool(description = "Evaluate revenue quality and sustainability by categorizing revenue into high-growth (>15% YoY), stable (0-15% YoY), and declining (<0% YoY) segments. Applies quality weights (1.0, 0.7, 0.0) to calculate composite quality score (0.0-1.0). Returns quality score, distribution breakdown, letter grade (A-F), strategic recommendation, and gap to industry benchmark (0.75).")]
    pub async fn calculate_revenue_quality_score(
        &self,
        Parameters(params): Parameters<RevenueQualityScoreParams>,
    ) -> Result<CallToolResult, McpError> {
        let _timer = RequestTimer::new();
        increment_requests();

        let high_growth_revenue = match parse_f64_from_string(&params.high_growth_revenue) {
            Ok(v) => v,
            Err(e) => {
                increment_errors();
                return Ok(CallToolResult::error(vec![Content::text(format!("Invalid high_growth_revenue: {}", e))]));
            }
        };

        let stable_revenue = match parse_f64_from_string(&params.stable_revenue) {
            Ok(v) => v,
            Err(e) => {
                increment_errors();
                return Ok(CallToolResult::error(vec![Content::text(format!("Invalid stable_revenue: {}", e))]));
            }
        };

        let declining_revenue = match parse_f64_from_string(&params.declining_revenue) {
            Ok(v) => v,
            Err(e) => {
                increment_errors();
                return Ok(CallToolResult::error(vec![Content::text(format!("Invalid declining_revenue: {}", e))]));
            }
        };

        let total_revenue = match parse_f64_from_string(&params.total_revenue) {
            Ok(v) => v,
            Err(e) => {
                increment_errors();
                return Ok(CallToolResult::error(vec![Content::text(format!("Invalid total_revenue: {}", e))]));
            }
        };

        match Self::calculate_revenue_quality_score_internal(
            high_growth_revenue,
            stable_revenue,
            declining_revenue,
            total_revenue,
        ) {
            Ok(result) => match serde_json::to_string_pretty(&result) {
                Ok(json_str) => Ok(CallToolResult::success(vec![Content::text(json_str)])),
                Err(e) => {
                    increment_errors();
                    Ok(CallToolResult::error(vec![Content::text(format!("Serialization error: {}", e))]))
                }
            },
            Err(e) => {
                increment_errors();
                Ok(CallToolResult::error(vec![Content::text(format!("Calculation error: {}", e))]))
            }
        }
    }

    #[tool(description = "Compute Herfindahl-Hirschman Index (HHI) to measure revenue concentration risk across business segments. HHI is sum of squared market shares (0.0-1.0). Returns HHI, diversification score (1-HHI), effective number of segments (1/HHI), risk classification (LOW <0.15, MEDIUM 0.15-0.25, HIGH >0.25), market shares, largest share, and concentration warnings.")]
    pub async fn calculate_hhi_and_diversification(
        &self,
        Parameters(params): Parameters<HHIParams>,
    ) -> Result<CallToolResult, McpError> {
        let _timer = RequestTimer::new();
        increment_requests();

        match Self::calculate_hhi_and_diversification_internal(params.revenues) {
            Ok(result) => match serde_json::to_string_pretty(&result) {
                Ok(json_str) => Ok(CallToolResult::success(vec![Content::text(json_str)])),
                Err(e) => {
                    increment_errors();
                    Ok(CallToolResult::error(vec![Content::text(format!("Serialization error: {}", e))]))
                }
            },
            Err(e) => {
                increment_errors();
                Ok(CallToolResult::error(vec![Content::text(format!("Calculation error: {}", e))]))
            }
        }
    }

    #[tool(description = "Calculate operating leverage ratio measuring relationship between revenue growth and cost growth to assess operational scalability. Ratio > 1.0 indicates positive operating leverage (revenue growing faster than costs). Returns operating leverage ratio, growth rates, margin expansion in basis points, efficiency rating (Excellent/Good/Adequate/Poor), and interpretation.")]
    pub async fn calculate_operating_leverage(
        &self,
        Parameters(params): Parameters<OperatingLeverageParams>,
    ) -> Result<CallToolResult, McpError> {
        let _timer = RequestTimer::new();
        increment_requests();

        let revenue_growth_rate = match parse_f64_from_string(&params.revenue_growth_rate) {
            Ok(v) => v,
            Err(e) => {
                increment_errors();
                return Ok(CallToolResult::error(vec![Content::text(format!("Invalid revenue_growth_rate: {}", e))]));
            }
        };

        let cost_growth_rate = match parse_f64_from_string(&params.cost_growth_rate) {
            Ok(v) => v,
            Err(e) => {
                increment_errors();
                return Ok(CallToolResult::error(vec![Content::text(format!("Invalid cost_growth_rate: {}", e))]));
            }
        };

        match Self::calculate_operating_leverage_internal(revenue_growth_rate, cost_growth_rate) {
            Ok(result) => match serde_json::to_string_pretty(&result) {
                Ok(json_str) => Ok(CallToolResult::success(vec![Content::text(json_str)])),
                Err(e) => {
                    increment_errors();
                    Ok(CallToolResult::error(vec![Content::text(format!("Serialization error: {}", e))]))
                }
            },
            Err(e) => {
                increment_errors();
                Ok(CallToolResult::error(vec![Content::text(format!("Calculation error: {}", e))]))
            }
        }
    }

    #[tool(description = "Calculate revenue-weighted portfolio momentum index measuring aggregate growth trajectory across business segments. Computes weighted average growth rate where each segment's contribution is proportional to its revenue share. Returns portfolio momentum (decimal and percentage), total revenue, per-segment contributions, top contributor, and momentum rating (Strong >10%, Moderate 5-10%, Weak 0-5%, Declining <0%).")]
    pub async fn calculate_portfolio_momentum(
        &self,
        Parameters(params): Parameters<PortfolioMomentumParams>,
    ) -> Result<CallToolResult, McpError> {
        let _timer = RequestTimer::new();
        increment_requests();

        match Self::calculate_portfolio_momentum_internal(params.segments) {
            Ok(result) => match serde_json::to_string_pretty(&result) {
                Ok(json_str) => Ok(CallToolResult::success(vec![Content::text(json_str)])),
                Err(e) => {
                    increment_errors();
                    Ok(CallToolResult::error(vec![Content::text(format!("Serialization error: {}", e))]))
                }
            },
            Err(e) => {
                increment_errors();
                Ok(CallToolResult::error(vec![Content::text(format!("Calculation error: {}", e))]))
            }
        }
    }

    #[tool(description = "Calculate Gini coefficient measuring revenue distribution inequality across segments for concentration risk assessment. Gini ranges 0-1 (0=perfect equality, 1=complete inequality). Returns Gini coefficient, diversification score (1-Gini), concentration level (Low <0.25, Moderate 0.25-0.40, High >0.40), largest/smallest segment shares, effective number of segments, and sorted revenues.")]
    pub async fn calculate_gini_coefficient(
        &self,
        Parameters(params): Parameters<GiniCoefficientParams>,
    ) -> Result<CallToolResult, McpError> {
        let _timer = RequestTimer::new();
        increment_requests();

        match Self::calculate_gini_coefficient_internal(params.revenues) {
            Ok(result) => match serde_json::to_string_pretty(&result) {
                Ok(json_str) => Ok(CallToolResult::success(vec![Content::text(json_str)])),
                Err(e) => {
                    increment_errors();
                    Ok(CallToolResult::error(vec![Content::text(format!("Serialization error: {}", e))]))
                }
            },
            Err(e) => {
                increment_errors();
                Ok(CallToolResult::error(vec![Content::text(format!("Calculation error: {}", e))]))
            }
        }
    }

    #[tool(description = "Calculate year-over-year organic revenue growth excluding acquisitions, divestitures, and other inorganic factors. This is the purest measure of underlying business performance. Returns organic growth rate (decimal and percentage), absolute dollar growth, prior/current revenue values, growth rating (Exceptional >15%, Strong 10-15%, Moderate 5-10%, Weak 0-5%, Declining <0%), and annualized CAGR.")]
    pub async fn calculate_organic_growth(
        &self,
        Parameters(params): Parameters<OrganicGrowthParams>,
    ) -> Result<CallToolResult, McpError> {
        let _timer = RequestTimer::new();
        increment_requests();

        let revenue_prior = match parse_f64_from_string(&params.revenue_prior) {
            Ok(v) => v,
            Err(e) => {
                increment_errors();
                return Ok(CallToolResult::error(vec![Content::text(format!("Invalid revenue_prior: {}", e))]));
            }
        };

        let revenue_current = match parse_f64_from_string(&params.revenue_current) {
            Ok(v) => v,
            Err(e) => {
                increment_errors();
                return Ok(CallToolResult::error(vec![Content::text(format!("Invalid revenue_current: {}", e))]));
            }
        };

        match Self::calculate_organic_growth_internal(revenue_prior, revenue_current) {
            Ok(result) => match serde_json::to_string_pretty(&result) {
                Ok(json_str) => Ok(CallToolResult::success(vec![Content::text(json_str)])),
                Err(e) => {
                    increment_errors();
                    Ok(CallToolResult::error(vec![Content::text(format!("Serialization error: {}", e))]))
                }
            },
            Err(e) => {
                increment_errors();
                Ok(CallToolResult::error(vec![Content::text(format!("Calculation error: {}", e))]))
            }
        }
    }
}

#[tool_handler]
impl ServerHandler for FinanceEngine {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Finance Engine providing seven calculation functions for financial analysis and business intelligence:\
                 \n\n**Critical Business Metrics**\
                 \n1. calculate_company_health_score - Comprehensive 0-100 health score combining five weighted dimensions: revenue growth (30%), SLA compliance (25%), modern revenue percentage (20%), customer satisfaction (15%), and pipeline coverage (10%)\
                 \n2. calculate_revenue_quality_score - Revenue quality evaluation with high-growth, stable, and declining categorization\
                 \n3. calculate_hhi_and_diversification - Herfindahl-Hirschman Index for revenue concentration risk assessment\
                 \n\n**Operational Metrics**\
                 \n4. calculate_operating_leverage - Operating leverage ratio measuring revenue growth vs cost growth for scalability assessment\
                 \n\n**Portfolio Analytics**\
                 \n5. calculate_portfolio_momentum - Revenue-weighted portfolio momentum index showing aggregate growth trajectory\
                 \n6. calculate_gini_coefficient - Gini coefficient for revenue concentration and diversification risk analysis\
                 \n7. calculate_organic_growth - Year-over-year organic revenue growth excluding inorganic factors\
                 \n\nAll functions perform sophisticated multi-step calculations with comprehensive validation.".into()
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: rmcp::model::Implementation {
                name: "finance-engine".to_string(),
                version: "2.0.0".to_string(),
                title: None,
                icons: None,
                website_url: None,
            },
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_calculate_company_health_score() {
        let engine = FinanceEngine::new();
        let params = CompanyHealthScoreParams {
            revenue_growth: "0.09".to_string(),
            sla_compliance: "0.985".to_string(),
            modern_revenue_pct: "0.377".to_string(),
            customer_satisfaction: "89.0".to_string(),
            pipeline_coverage: "0.849".to_string(),
        };
        
        let result = engine.calculate_company_health_score(Parameters(params)).await;
        assert!(result.is_ok());
        
        let call_result = result.unwrap();
        let content = call_result.content;
        let json_text = content[0].raw.as_text().unwrap().text.as_str();
        let response: CompanyHealthScoreResponse = serde_json::from_str(json_text).unwrap();
        
        // Expected overall score: 72.0 based on spec example
        assert!(response.overall_score > 70.0 && response.overall_score < 74.0);
        assert!(response.overall_score <= 100.0);
        assert_eq!(response.risk_level, "MEDIUM");
        
        // Verify component scores
        assert!((response.components["revenue"] - 60.0).abs() < 0.1);
        assert!((response.components["sla"] - 98.5).abs() < 0.1);
        assert!((response.components["innovation"] - 37.7).abs() < 0.1);
        assert!((response.components["satisfaction"] - 89.0).abs() < 0.1);
        assert!((response.components["pipeline"] - 84.9).abs() < 0.1);
        
        // Verify weighted contributions
        assert!((response.weighted_contributions["revenue"] - 18.0).abs() < 0.1);
        assert!((response.weighted_contributions["sla"] - 24.625).abs() < 0.1);
        assert!((response.weighted_contributions["innovation"] - 7.54).abs() < 0.1);
        assert!((response.weighted_contributions["satisfaction"] - 13.35).abs() < 0.1);
        assert!((response.weighted_contributions["pipeline"] - 8.49).abs() < 0.1);
    }

    #[tokio::test]
    async fn test_calculate_revenue_quality_score() {
        let engine = FinanceEngine::new();
        let params = RevenueQualityScoreParams {
            high_growth_revenue: "15.0".to_string(),
            stable_revenue: "25.0".to_string(),
            declining_revenue: "10.0".to_string(),
            total_revenue: "50.0".to_string(),
        };
        
        let result = engine.calculate_revenue_quality_score(Parameters(params)).await;
        assert!(result.is_ok());
        
        let call_result = result.unwrap();
        let content = call_result.content;
        let json_text = content[0].raw.as_text().unwrap().text.as_str();
        let response: RevenueQualityScoreResponse = serde_json::from_str(json_text).unwrap();
        
        assert!(response.quality_score >= 0.0 && response.quality_score <= 1.0);
        assert!(!response.grade.is_empty());
    }

    #[tokio::test]
    async fn test_calculate_hhi_and_diversification() {
        let engine = FinanceEngine::new();
        let params = HHIParams {
            revenues: vec![15.0, 25.0, 5.0, 8.0],
        };
        
        let result = engine.calculate_hhi_and_diversification(Parameters(params)).await;
        assert!(result.is_ok());
        
        let call_result = result.unwrap();
        let content = call_result.content;
        let json_text = content[0].raw.as_text().unwrap().text.as_str();
        let response: HHIResponse = serde_json::from_str(json_text).unwrap();
        
        assert!(response.hhi >= 0.0 && response.hhi <= 1.0);
        assert!(response.effective_n >= 1.0);
    }

    #[tokio::test]
    async fn test_calculate_operating_leverage() {
        let engine = FinanceEngine::new();
        let params = OperatingLeverageParams {
            revenue_growth_rate: "0.09".to_string(),
            cost_growth_rate: "0.06".to_string(),
        };
        
        let result = engine.calculate_operating_leverage(Parameters(params)).await;
        assert!(result.is_ok());
        
        let call_result = result.unwrap();
        let content = call_result.content;
        let json_text = content[0].raw.as_text().unwrap().text.as_str();
        let response: OperatingLeverageResponse = serde_json::from_str(json_text).unwrap();
        
        assert_eq!(response.operating_leverage, 1.5);
        assert_eq!(response.revenue_growth_pct, 9.0);
        assert_eq!(response.cost_growth_pct, 6.0);
        assert_eq!(response.margin_expansion_bps, 300.0);
        assert_eq!(response.efficiency_rating, "Excellent");
        assert!(!response.interpretation.is_empty());
    }

    #[tokio::test]
    async fn test_calculate_operating_leverage_zero_cost_growth() {
        let engine = FinanceEngine::new();
        let params = OperatingLeverageParams {
            revenue_growth_rate: "0.09".to_string(),
            cost_growth_rate: "0.0".to_string(),
        };
        
        let result = engine.calculate_operating_leverage(Parameters(params)).await;
        assert!(result.is_ok());
        
        let call_result = result.unwrap();
        let content = call_result.content;
        let json_text = content[0].raw.as_text().unwrap().text.as_str();
        assert!(json_text.contains("Cost growth rate cannot be zero"));
    }

    #[tokio::test]
    async fn test_calculate_portfolio_momentum() {
        let engine = FinanceEngine::new();
        let mut segments = HashMap::new();
        segments.insert("subscription".to_string(), PortfolioSegmentData {
            revenue: 15.0,
            growth_rate: 0.20,
        });
        segments.insert("enterprise".to_string(), PortfolioSegmentData {
            revenue: 25.0,
            growth_rate: 0.14,
        });
        segments.insert("upsell".to_string(), PortfolioSegmentData {
            revenue: 5.0,
            growth_rate: 0.19,
        });
        segments.insert("legacy".to_string(), PortfolioSegmentData {
            revenue: 8.0,
            growth_rate: -0.20,
        });
        
        let params = PortfolioMomentumParams { segments };
        
        let result = engine.calculate_portfolio_momentum(Parameters(params)).await;
        assert!(result.is_ok());
        
        let call_result = result.unwrap();
        let content = call_result.content;
        let json_text = content[0].raw.as_text().unwrap().text.as_str();
        let response: PortfolioMomentumResponse = serde_json::from_str(json_text).unwrap();
        
        assert!(response.portfolio_momentum > 0.0);
        assert_eq!(response.total_revenue, 53.0);
        assert_eq!(response.momentum_rating, "Strong");
        assert!(!response.top_contributor.is_empty());
        assert_eq!(response.segment_contributions.len(), 4);
    }

    #[tokio::test]
    async fn test_calculate_gini_coefficient() {
        let engine = FinanceEngine::new();
        let params = GiniCoefficientParams {
            revenues: vec![15.0, 25.0, 5.0, 8.0],
        };
        
        let result = engine.calculate_gini_coefficient(Parameters(params)).await;
        assert!(result.is_ok());
        
        let call_result = result.unwrap();
        let content = call_result.content;
        let json_text = content[0].raw.as_text().unwrap().text.as_str();
        let response: GiniCoefficientResponse = serde_json::from_str(json_text).unwrap();
        
        assert!(response.gini_coefficient >= 0.0);
        assert!(response.gini_coefficient <= 1.0);
        // Use approximate comparison for floating point
        assert!((response.diversification_score - (1.0 - response.gini_coefficient)).abs() < 0.001);
        assert!(response.largest_segment_share > response.smallest_segment_share);
        assert_eq!(response.sorted_revenues.len(), 4);
        assert!(response.sorted_revenues[0] <= response.sorted_revenues[3]);
    }

    #[tokio::test]
    async fn test_calculate_gini_coefficient_empty_list() {
        let engine = FinanceEngine::new();
        let params = GiniCoefficientParams {
            revenues: vec![],
        };
        
        let result = engine.calculate_gini_coefficient(Parameters(params)).await;
        assert!(result.is_ok());
        
        let call_result = result.unwrap();
        let content = call_result.content;
        let json_text = content[0].raw.as_text().unwrap().text.as_str();
        assert!(json_text.contains("Revenue list cannot be empty"));
    }

    #[tokio::test]
    async fn test_calculate_organic_growth() {
        let engine = FinanceEngine::new();
        let params = OrganicGrowthParams {
            revenue_prior: "48.7".to_string(),
            revenue_current: "53.0".to_string(),
        };
        
        let result = engine.calculate_organic_growth(Parameters(params)).await;
        assert!(result.is_ok());
        
        let call_result = result.unwrap();
        let content = call_result.content;
        let json_text = content[0].raw.as_text().unwrap().text.as_str();
        let response: OrganicGrowthResponse = serde_json::from_str(json_text).unwrap();
        
        assert!(response.organic_growth_rate > 0.0);
        assert_eq!(response.revenue_prior, 48.7);
        assert_eq!(response.revenue_current, 53.0);
        assert_eq!(response.absolute_growth, 4.3);
        // Growth rate is 8.83%, which falls in Moderate range (5-10%)
        assert_eq!(response.growth_rating, "Moderate");
        assert_eq!(response.organic_growth_pct, response.annualized_cagr);
    }

    #[tokio::test]
    async fn test_calculate_organic_growth_negative_prior() {
        let engine = FinanceEngine::new();
        let params = OrganicGrowthParams {
            revenue_prior: "0".to_string(),
            revenue_current: "53.0".to_string(),
        };
        
        let result = engine.calculate_organic_growth(Parameters(params)).await;
        assert!(result.is_ok());
        
        let call_result = result.unwrap();
        let content = call_result.content;
        let json_text = content[0].raw.as_text().unwrap().text.as_str();
        assert!(json_text.contains("Prior period revenue must be positive"));
    }

    #[tokio::test]
    async fn test_calculate_organic_growth_declining() {
        let engine = FinanceEngine::new();
        let params = OrganicGrowthParams {
            revenue_prior: "53.0".to_string(),
            revenue_current: "48.0".to_string(),
        };
        
        let result = engine.calculate_organic_growth(Parameters(params)).await;
        assert!(result.is_ok());
        
        let call_result = result.unwrap();
        let content = call_result.content;
        let json_text = content[0].raw.as_text().unwrap().text.as_str();
        let response: OrganicGrowthResponse = serde_json::from_str(json_text).unwrap();
        
        assert!(response.organic_growth_rate < 0.0);
        assert_eq!(response.growth_rating, "Declining");
    }
}

