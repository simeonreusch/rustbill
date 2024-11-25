use std::{path::Path};
use clap::Parser;
use rust_decimal::Decimal;
use rusty_money::{iso::{self}, Round, Money};
// use std::error::Error;
use config_reader::{ExtractError, read_config};
use thiserror::Error;
mod pdf_gen;
mod date_utils;
mod csv_reader;
mod config_reader;
mod qrcode;
mod sign;
mod ebill;
mod calculate;

#[derive(Debug, Error)]
pub enum AmountCalcs{
    #[error("No total amount could be computed")]
    CalculationError,
    #[error("Extraction error")]
    ExtractError(#[from] ExtractError),
}
type CurrencyResult<T> = Result<T, AmountCalcs>;

#[derive(Parser, Debug)]
#[command(name = "cli_parser")]
#[command(about = "A parser with a default argument (the company) and an optional date flag")]
struct Args {
    /// A default argument for general input
    input: String,

    /// Optional date in YYYY-MM-DD format
    #[arg(long)]
    date: Option<String>,
}


fn to_euro_string(currencyfloat: &f64) -> CurrencyResult<String> {

    let decimal_amount = Decimal::from_f64_retain(*currencyfloat).unwrap();
    let eur_amount = Money::from_decimal(decimal_amount, iso::EUR);
    let eur_amount_rounded = eur_amount.round(2, Round::HalfEven);
    let eur_amount_rounded_value = *eur_amount_rounded.amount();
    let raw_string = eur_amount_rounded_value.to_string();
            // let formatted_str = raw_string.replace('.', ",");

    Ok(raw_string)
}


fn main() {
    let args = Args::parse();

    let date = args.date.unwrap_or(String::from(""));

    let billdate = date_utils::parse_date_or_default(&date);
    let billdate_formatted = billdate.format("%d.%m.%Y").to_string();

    let duedate = date_utils::calculate_due_date(billdate);
    let duedate_formatted = duedate.format("%d.%m.%Y").to_string();

    let company_str = args.input;

    let config_path = "config.yaml";
    let basedir = Path::new("/Users/simeon/rustbill/data");
    let mut file_str = company_str.clone();

    if !file_str.ends_with(".csv") {
        file_str.push_str(".csv");
    }

    let csv_path = basedir.join(file_str);
    let company = String::from(company_str);
    let billnr: i32 = 4;

    let minutes_res = csv_reader::read_csv(&csv_path);
    let minutes_total: i32;

    match minutes_res {
        Ok(v) => minutes_total = v,
        Err(e) => {
            eprintln!("An error occured while extracting the minutes total: {}", e);
            std::process::exit(1);
        }
    }

    let amounts_res = calculate::calculate_amounts(&config_path, &company, &minutes_total);
    let amount_net: f64;
    let amount_vat: f64;
    let amount_total: f64;

    match amounts_res {
        Ok(v) => {amount_net = v.0; amount_vat = v.1; amount_total = v.2},
        Err(e) => {
            eprintln!("An error occured while calculating the amounts: {}", e);
            std::process::exit(1);
        }
    }

    let decimal_amount_str = to_euro_string(&amount_total).unwrap();

    let config = read_config(&config_path).unwrap();
    let bank_config = config.bank_config;
    let bill_config = config.bill_config;
    

    let qrcode = qrcode::create_qrcode(bank_config, &decimal_amount_str, &company, &billdate).unwrap();

    let pdf_content = pdf_gen::Content {
        company: company.to_string(),
        billnr,
        vat: amount_vat.clone(),
        date: billdate_formatted,
        due: duedate_formatted,
        qrcode,
    };

    let pdf_data = pdf_gen::generate_pdf(pdf_content);
    println!("Generated pdf");

    let _ = sign::sign_pdf(pdf_data);

    let xml = ebill::create_xml(amount_net, amount_vat, amount_total, billdate, bill_config);
    

}