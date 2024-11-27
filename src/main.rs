use std::{path::Path};
use clap::Parser;
use config_reader::read_config;
mod pdf_gen;
mod date_utils;
mod csv_reader;
mod config_reader;
mod qrcode;
mod sign;
mod ebill;
mod calculate;


#[derive(Parser, Debug)]
#[command(name = "cli_parser")]
#[command(about = "A parser with a default argument (the company) and an optional date flag")]
struct Args {
    input: String,
    #[arg(short, long, default_value_t = String::from(""))] // Optional date in YYYY-MM-DD format
    date: String,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let date = args.date;
    let company_str = args.input;
    let mut file_str = company_str.clone();
    if !file_str.ends_with(".csv") {
        file_str.push_str(".csv");
    }

    let basedir = Path::new("/Users/simeon/rustbill/data");
    let config_path = "config.yaml";
    let config = read_config(&config_path)?;

    let billdate = date_utils::parse_date_or_default(&date)?;
    let billdate_formatted = billdate.format("%d.%m.%Y").to_string();
    let duedate = date_utils::calculate_due_date(billdate);
    let duedate_formatted = duedate?.format("%d.%m.%Y").to_string();

    let csv_path = basedir.join(file_str);
    let company = String::from(company_str);
    let billnr: i32 = 4;

    let csv_data = csv_reader::read_csv(&csv_path)?;

    let minutes_total = csv_reader::extract_minutes_total(&csv_data)?;

    let amounts = calculate::calculate_amounts(&config_path, &company, &minutes_total)?;

    let decimal_amount_str = calculate::to_euro_string(&amounts.total)?;

    let qrcode = qrcode::create_qrcode(config.bank_config, &decimal_amount_str, &company, &billdate)?;

    let pdf_content = pdf_gen::Content {
        company: company.to_string(),
        billnr,
        vat: amounts.vat.clone(),
        date: billdate_formatted,
        due: duedate_formatted,
        qrcode,
    };

    let pdf_data = pdf_gen::generate_pdf(pdf_content)?;
    println!("Generated pdf");

    let _ = sign::sign_pdf(pdf_data);

    let xml = ebill::create_xml(amounts, billdate, config.bill_config);

    Ok(())
    
}