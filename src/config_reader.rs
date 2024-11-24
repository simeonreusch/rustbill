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
    #[error("Expected a float")]
    ExpectedFloat,
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


pub fn get_hourly_fee(path: &str, company: &str) -> Result<f64, ExtractError> {
    let config = read_config_yaml(path);

    match config {
        Ok(config) => {

        if let Some(companies) = config.get("companies") {
            if let Some(company) = companies.get(company) {
                if let Some(hourly_fee_val) = company.get("hourly_fee") {
                    if let Some(hourly_fee) = hourly_fee_val.as_f64() {
                        Ok(hourly_fee as f64)
                    }
                    else {
                        Ok(75.0)
                        }
                }
                else {
                    Err(ExtractError::MissingData)
                }
            }
            else {
                Err(ExtractError::MissingData)
            }
        }
        else {
            Err(ExtractError::MissingData)
        }
    },
    Err(_) => Err(ExtractError::MissingData)
}

}