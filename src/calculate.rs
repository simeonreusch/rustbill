use crate::config_reader;
use rust_decimal::Decimal;
use rusty_money::{iso::{self}, Round, Money};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AmountCalcs{
    #[error("No total amount could be computed")]
    CalculationError,
    #[error("Extraction error")]
    ExtractError(#[from] config_reader::ExtractError),
}

type CalculationResult<T> = Result<T, AmountCalcs>;
type CurrencyResult<T> = Result<T, AmountCalcs>;

#[derive(Debug)]
pub struct Amounts {
    pub net: f64,
    pub vat: f64,
    pub total: f64,
    pub hourly_fee: f64,
    pub hours_total: f64,
}

fn calculate_amount_net(minutes_total: &i32, hourly_fee: &f64) -> CalculationResult<f64> {
    let minutes_total_float: f64 = *minutes_total as f64;
    let amount_net: f64 = minutes_total_float / 60.0 * hourly_fee;
    Ok(amount_net)
}

fn calculate_vat(amount_net: &f64) -> CalculationResult<f64> {
    let vat_rate: f64 = 0.19;
    let amount_vat: f64 = amount_net * vat_rate;
    Ok(amount_vat)
}

fn calculate_amount_total(amount_net: &f64, amount_vat: &f64) -> CalculationResult<f64> {
    let amount_total = amount_net + amount_vat;
    Ok(amount_total)
}

pub fn calculate_amounts(minutes_total: &i32, hourly_fee: &f64) -> CalculationResult<Amounts> {
    let amount_net = calculate_amount_net(minutes_total, hourly_fee)?;
    let amount_vat = calculate_vat(&amount_net)?;
    let amount_total = calculate_amount_total(&amount_net, &amount_vat)?;
    let hours_total = *minutes_total as f64 / 60.0;

    let amounts =  Amounts {net: amount_net, vat: amount_vat, total: amount_total, hourly_fee: hourly_fee.clone(), hours_total};

    Ok(amounts)
}

pub fn to_euro_string(currencyfloat: &f64) -> CurrencyResult<String> {

    let decimal_amount = Decimal::from_f64_retain(*currencyfloat).ok_or(AmountCalcs::CalculationError)?;
    let eur_amount = Money::from_decimal(decimal_amount, iso::EUR);
    let eur_amount_rounded = eur_amount.round(2, Round::HalfEven);
    let eur_amount_rounded_value = *eur_amount_rounded.amount();
    let raw_string = eur_amount_rounded_value.to_string();

    Ok(raw_string)
}