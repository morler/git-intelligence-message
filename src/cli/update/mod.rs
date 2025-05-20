use std::process::Command;
use serde_json;

pub mod models;
use models::BrewFormulae;

// 从 Cargo.toml 中读取当前版本
const VERSION: &str = env!("CARGO_PKG_VERSION");

const HOMEBREW_FORMULA: &str = "git-intelligence-message";

pub async fn check_and_install_update(force: bool) -> Result<(), Box<dyn std::error::Error>> {
    let current_version = VERSION;
    println!("Checking for updates via Homebrew...");

    // Get latest version from Homebrew
    let output = Command::new("brew")
        .args(["info", "--json=v2", HOMEBREW_FORMULA])
        .output()?;

    if !output.status.success() {
        return Err("Failed to fetch version information from Homebrew".into());
    }

    let formulae: BrewFormulae = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Failed to parse Homebrew info: {}", e))?;
    let brew_info = formulae.formulae;

    let latest_version = brew_info.first()
        .ok_or("No version information found in Homebrew response")?
        .versions.stable.trim_start_matches('v');

    // Parse versions for comparison
    let current = semver::Version::parse(current_version)
        .map_err(|_| format!("Invalid current version format: {}", current_version))?;
    let latest = semver::Version::parse(latest_version)
        .map_err(|_| format!("Invalid version format in release: {}", latest_version))?;

    // Only proceed if force is true or if latest is actually newer
    if latest <= current {
        if !force {
            println!(
                "You're already on the latest version: {} Run with --force to reinstall anyway.",
                current_version
            );
            return Ok(());
        }
        println!("Forcing reinstall of version: {}", latest_version);
    } else {
        println!(
            "New version available: {} (current: {})",
            latest_version, current_version
        );
    }

    // Use Homebrew to upgrade the package
    println!("Upgrading via Homebrew...");
    
    let status = Command::new("brew")
        .args(["upgrade", HOMEBREW_FORMULA])
        .status()?;

    if !status.success() {
        return Err("Failed to upgrade via Homebrew".into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_update() {
        let updated = check_and_install_update(false).await;
        assert!(updated.is_ok(), "update failed (test)");
    }
}
