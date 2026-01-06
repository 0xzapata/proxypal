use tauri::State;

use crate::config::load_config, save_config_to_file, state::AppState, types::HeliconeConfig;

/// Fetch the current Helicone configuration
#[tauri::command]
pub fn get_helicone_config(state: State<'_, AppState>) -> Result<HeliconeConfig, String> {
    let config = state.config.lock().unwrap();
    Ok(config.helicone.clone())
}

/// Update Helicone configuration
#[tauri::command]
pub async fn set_helicone_config(
    config: HeliconeConfig,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut app_config = state.config.lock().unwrap();
    app_config.helicone = config;
    drop(app_config);

    // Save to file
    let config_to_save = state.config.lock().unwrap().clone();
    save_config_to_file(&config_to_save)?;

    Ok(())
}

/// Test Helicone connectivity with current configuration
#[tauri::command]
pub async fn test_helicone_connection(
    state: State<'_, AppState>,
) -> Result<TestResult, String> {
    let config = state.config.lock().unwrap();

    if !config.helicone.enabled {
        return Ok(TestResult {
            success: false,
            message: "Helicone is not enabled".to_string(),
        });
    }

    if config.helicone.api_key.is_empty() {
        return Ok(TestResult {
            success: false,
            message: "Helicone API key is required".to_string(),
        });
    }

    let base_url = config.helicone.get_base_url();
    let api_key = &config.helicone.api_key;

    // Test connection to Helicone API
    let url = if config.helicone.use_self_hosted {
        format!("{}/v1/models", base_url)
    } else {
        format!("https://api.helicone.ai/v1/models")
    };

    let client = reqwest::Client::new();
    let response = match client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .timeout(tokio::time::Duration::from_secs(10))
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            return Ok(TestResult {
                success: false,
                message: format!("Connection failed: {}", e),
            });
        }
    };

    let status = response.status();

    if status.is_success() {
        Ok(TestResult {
            success: true,
            message: "Successfully connected to Helicone".to_string(),
        })
    } else {
        Ok(TestResult {
            success: false,
            message: format!("Failed to connect: HTTP {}", status),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TestResult {
    pub success: bool,
    pub message: String,
}
