use toml;

pub fn update_ai_config(
    config: &mut toml::Value,
    model: &Option<String>,
    apikey: &Option<String>,
    url: &Option<String>,
    prompt: &Option<String>,
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
    if let Some(prompt_value) = prompt {
        ai_table.insert(
            "prompt".to_string(),
            toml::Value::String(prompt_value.clone()),
        );
    }
    if let Some(language_value) = language {
        ai_table.insert(
            "language".to_string(),
            toml::Value::String(language_value.clone()),
        );
    }

    crate::config::save_config(config);
}
