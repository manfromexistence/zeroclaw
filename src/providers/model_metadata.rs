//! Model metadata from LiteLLM's comprehensive database
//! 
//! This module provides access to pricing, context limits, and capabilities
//! for 2600+ models across 140+ providers.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Model metadata including pricing, limits, and capabilities
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelInfo {
    #[serde(default)]
    pub litellm_provider: String,
    
    #[serde(default)]
    pub mode: String,
    
    #[serde(default)]
    pub max_tokens: Option<u32>,
    
    #[serde(default)]
    pub max_input_tokens: Option<u32>,
    
    #[serde(default)]
    pub max_output_tokens: Option<u32>,
    
    #[serde(default)]
    pub input_cost_per_token: Option<f64>,
    
    #[serde(default)]
    pub output_cost_per_token: Option<f64>,
    
    #[serde(default)]
    pub output_cost_per_reasoning_token: Option<f64>,
    
    #[serde(default)]
    pub supports_vision: Option<bool>,
    
    #[serde(default)]
    pub supports_function_calling: Option<bool>,
    
    #[serde(default)]
    pub supports_parallel_function_calling: Option<bool>,
    
    #[serde(default)]
    pub supports_prompt_caching: Option<bool>,
    
    #[serde(default)]
    pub supports_reasoning: Option<bool>,
    
    #[serde(default)]
    pub supports_response_schema: Option<bool>,
    
    #[serde(default)]
    pub supports_system_messages: Option<bool>,
    
    #[serde(default)]
    pub supports_audio_input: Option<bool>,
    
    #[serde(default)]
    pub supports_audio_output: Option<bool>,
    
    #[serde(default)]
    pub supports_web_search: Option<bool>,
    
    #[serde(default)]
    pub deprecation_date: Option<String>,
    
    #[serde(default)]
    pub supported_regions: Option<Vec<String>>,
}

impl ModelInfo {
    /// Get the effective max input tokens
    pub fn effective_max_input_tokens(&self) -> Option<u32> {
        self.max_input_tokens.or(self.max_tokens)
    }
    
    /// Get the effective max output tokens
    pub fn effective_max_output_tokens(&self) -> Option<u32> {
        self.max_output_tokens.or(self.max_tokens)
    }
    
    /// Check if the model is deprecated
    pub fn is_deprecated(&self) -> bool {
        self.deprecation_date.is_some()
    }
    
    /// Check if the model supports chat mode
    pub fn is_chat_model(&self) -> bool {
        self.mode == "chat"
    }
}

/// Global model metadata database
static MODEL_DATABASE: &str = include_str!("model_prices_and_context_window.json");

/// Lazy-loaded model database
static MODEL_DB: once_cell::sync::Lazy<HashMap<String, ModelInfo>> = 
    once_cell::sync::Lazy::new(|| {
        let raw: HashMap<String, serde_json::Value> = serde_json::from_str(MODEL_DATABASE)
            .unwrap_or_else(|e| {
                eprintln!("Warning: Failed to parse model database: {}", e);
                HashMap::new()
            });
        
        // Filter out sample_spec and parse valid entries
        raw.into_iter()
            .filter(|(k, _)| k != "sample_spec")
            .filter_map(|(k, v)| {
                serde_json::from_value::<ModelInfo>(v)
                    .ok()
                    .map(|info| (k, info))
            })
            .collect()
    });

/// Get model metadata by model name
pub fn get_model_info(model_name: &str) -> Option<&ModelInfo> {
    MODEL_DB.get(model_name)
}

/// Get all models for a specific provider
pub fn get_models_by_provider(provider: &str) -> Vec<(&String, &ModelInfo)> {
    MODEL_DB
        .iter()
        .filter(|(_, info)| info.litellm_provider == provider)
        .collect()
}

/// Get all available providers
pub fn get_all_providers() -> Vec<String> {
    let mut providers: Vec<String> = MODEL_DB
        .values()
        .map(|info| info.litellm_provider.clone())
        .filter(|p| !p.is_empty() && p != "sample_spec")
        .collect();
    providers.sort();
    providers.dedup();
    providers
}

/// Get count of models per provider
pub fn get_provider_model_counts() -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for info in MODEL_DB.values() {
        if !info.litellm_provider.is_empty() && info.litellm_provider != "sample_spec" {
            *counts.entry(info.litellm_provider.clone()).or_insert(0) += 1;
        }
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_database_loads() {
        assert!(!MODEL_DB.is_empty(), "Model database should not be empty");
    }

    #[test]
    fn can_get_providers() {
        let providers = get_all_providers();
        assert!(!providers.is_empty(), "Should have providers");
        assert!(providers.len() > 50, "Should have 50+ providers");
    }

    #[test]
    fn can_get_provider_counts() {
        let counts = get_provider_model_counts();
        assert!(!counts.is_empty(), "Should have provider counts");
    }
}
