use dirs;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub fn ensure_config_file_exists() -> std::io::Result<PathBuf> {
    let config_dir = dirs::home_dir();
    if config_dir.is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Home directory not found",
        ));
    }

    let config_dir = config_dir.unwrap().join(".config/gim/");
    let config_file = config_dir.join("config.toml");
    if !config_file.exists() {
        if let Some(parent) = config_file.parent() {
            fs::create_dir_all(parent)?;
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "config directory not found",
            ));
        }
        let mut settings = toml::map::Map::new();
        settings.insert(
            "example".to_string(),
            toml::Value::String("value".to_string()),
        );
        let mut default_content = toml::map::Map::new();
        default_content.insert("settings".to_string(), toml::Value::Table(settings));
        let default_content = toml::to_string(&toml::Value::Table(default_content))
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let mut file = fs::File::create(&config_file)?;
        file.write_all(default_content.as_bytes())?;
    }
    println!("Config file is {}", config_file.display());
    Ok(config_file)
}