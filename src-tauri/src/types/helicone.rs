use serde::{Deserialize, Serialize};

/// Helicone observability integration configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HeliconeConfig {
    /// Enable/disable Helicone integration
    #[serde(default)]
    pub enabled: bool,

    /// Helicone API key from https://www.helicone.ai/keys
    #[serde(default)]
    pub api_key: String,

    /// Use self-hosted Helicone instance
    #[serde(default)]
    pub use_self_hosted: bool,

    /// Custom Helicone URL for self-hosted deployments (e.g., https://helicone.yourcompany.com)
    #[serde(default)]
    pub self_hosted_url: String,

    /// Get the base URL for Helicone gateway
    /// Returns https://ai-gateway.helicone.ai for cloud, or the custom URL for self-hosted
    pub fn get_base_url(&self) -> String {
        if self.use_self_hosted && !self.self_hosted_url.is_empty() {
            self.self_hosted_url.clone()
        } else {
            "https://ai-gateway.helicone.ai".to_string()
        }
    }
}

impl Default for HeliconeConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            api_key: String::new(),
            use_self_hosted: false,
            self_hosted_url: String::new(),
        }
    }
}
