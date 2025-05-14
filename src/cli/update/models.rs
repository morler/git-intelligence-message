use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BrewInfo {
    pub versions: BrewVersions,
}

#[derive(Debug, Deserialize)]
pub struct BrewVersions {
    pub stable: String,
}

#[derive(Debug, Deserialize)]
pub struct BrewFormulae {
    pub formulae: Vec<BrewInfo>
}