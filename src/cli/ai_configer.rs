use gim_config::config;
use std::io::Result;
use toml;

/// Updates the AI configuration in the provided TOML value with the specified model, API key, URL, and language.
///
/// # Arguments
///
/// * `config` - Mutable reference to the TOML configuration value.
/// * `model` - Optional model name to set.
/// * `apikey` - Optional API key to set.
/// * `url` - Optional API URL to set.
/// * `language` - Optional language to set.
pub fn update_ai_config(
    config: &mut toml::Value,
    model: &Option<String>,
    apikey: &Option<String>,
    url: &Option<String>,
    language: &Option<String>,
) {
    let ai_table = config
        .get_mut("ai")
        .expect("Missing ai section")
        .as_table_mut()
        .expect("ai section is not a table");

    if let Some(model_value) = model {
        ai_table.insert(
            "model".to_string(),
            toml::Value::String(model_value.clone()),
        );
    }
    if let Some(apikey_value) = apikey {
        ai_table.insert(
            "apikey".to_string(),
            toml::Value::String(apikey_value.clone()),
        );
    }
    if let Some(url_value) = url {
        ai_table.insert("url".to_string(), toml::Value::String(url_value.clone()));
    }
    if let Some(language_value) = language {
        ai_table.insert(
            "language".to_string(),
            toml::Value::String(language_value.clone()),
        );
    }

    if let Err(e) = config::save_config(config) {
        eprintln!("Failed to save AI info to file: {}", e)
    }
}

/// Retrieves the AI configuration section from the TOML configuration file.
///
/// # Returns
///
/// * `Ok(toml::Value)` containing the AI configuration if successful.
/// * `Err(std::io::Error)` if the AI section is missing or invalid.
pub fn get_ai_config() -> Result<toml::Value> {
    let toml = config::get_config_into_toml(false);
    if toml.is_err() {
        toml
    } else if let Ok(toml) = toml {
        let ai = toml.get("ai");
        if let Some(ai) = ai {
            Ok(ai.clone())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to get ai section",
            ))
        }
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Failed to get ai config",
        ))
    }
}
