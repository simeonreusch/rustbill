use serde::Deserialize;
use std::fs::{self};
use serde_yaml::{self, Value};
use serde_yaml::Error as YamlError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExtractError {
    #[error("YAML parsing error")]
    YamlParseError(#[from] YamlError),
    #[error("Missing data")]
    MissingData,
    #[error("Error reading yaml")]
    Error(#[from] std::io::Error)
}

#[derive(Debug, Deserialize)]
pub struct BankConfig {
    pub bic: String,
    pub iban: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct BillConfig {
  pub company: String,
  pub name: String,
  pub street: String,
  pub city: String,
  pub country: String,
  pub postcode: String,
  pub email: String,
  pub vat_id: String,
  pub tax_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub bank_config: BankConfig,
    pub bill_config: BillConfig,
}

fn read_config_yaml(path: &str) -> Result<Value, ExtractError> {
    let yaml_content = fs::read_to_string(path)?;

    // Parse the YAML string into a Config
    let config: Value = serde_yaml::from_str(&yaml_content)?;

    Ok(config)
}

pub fn read_config(path: &str) -> Result<Config, ExtractError> {
    let yaml_content = fs::read_to_string(path)?;

    let config: Config = serde_yaml::from_str(&yaml_content)?;

    Ok(config)
}

// FIXME: THIS NEED TO BE BETTER
pub fn get_hourly_fee(path: &str, company: &str) -> Result<f64, ExtractError> {
    let config = read_config_yaml(path)?;

    let Some(companies) = config.get("companies") else {return Err(ExtractError::MissingData)};
    let Some(company) = companies.get(company) else {return Err(ExtractError::MissingData)};
    let Some(hourly_fee_val) = company.get("hourly_fee") else {return Err(ExtractError::MissingData)};
    let Some(hourly_fee) = hourly_fee_val.as_f64() else {return Err(ExtractError::MissingData)};

    println!("hourly fee is {} EUR", hourly_fee);
    Ok(hourly_fee)
}