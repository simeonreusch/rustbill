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
pub struct CompanyAddress {
    pub name: String,
    pub city: String,
    pub postcode: String,
    pub addressline: String,

}

#[derive(Debug, Deserialize)]
pub struct CompanyConfig {
    pub email: String,
    pub subject: String,
    pub greeting_to: String,
    pub greeting_from: String,
    pub hourly_fee: f64,
    pub address: CompanyAddress,
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
  pub telephone: String,
  pub bill_item: String,
  pub bill_item_description: String,
  pub vat_id: String,
  pub tax_id: String,
  pub color: String,
}

#[derive(Debug, Deserialize)]
pub struct MailConfig {
    pub email: String,
    pub imap_server: String,
    pub imap_port: u16,
    pub email_text: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub bank_config: BankConfig,
    pub bill_config: BillConfig,
    pub mailconfig: MailConfig,
}

fn read_config_yaml(path: &str) -> Result<Value, ExtractError> {
    let yaml_content = fs::read_to_string(path)?;

    // Parse the YAML string into a Config
    let config: Value = serde_yaml::from_str(&yaml_content)?;

    Ok(config)
}

pub fn read_config(path: &str) -> Result<Config, ExtractError> {

    println!("Reading config from {:?}", path);
    
    let yaml_content = fs::read_to_string(path)?;

    let config: Config = serde_yaml::from_str(&yaml_content)?;

    Ok(config)
}

pub fn get_company_config(path: &str, company_str: &str) -> Result<CompanyConfig, ExtractError> {
    let config = read_config_yaml(path)?;

    let Some(companies) = config.get("companies") else {return Err(ExtractError::MissingData)};
    let Some(company) = companies.get(company_str) else {return Err(ExtractError::MissingData)};

    let company_config: CompanyConfig = serde_yaml::from_value(company.clone())?;

    Ok(company_config)
}