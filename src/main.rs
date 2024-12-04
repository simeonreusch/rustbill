use std::path::Path;
use clap::Parser;
use rand::thread_rng;
use rand::seq::SliceRandom;
use chrono::Datelike;
use config_reader::read_config;
mod pdf_gen;
mod date_utils;
mod csv_reader;
mod config_reader;
mod qrcode;
mod sign;
mod ebill;
mod calculate;
mod db;
mod mail;

#[derive(Parser, Debug)]
#[command(name = "cli_parser")]
#[command(about = "A parser with a default argument (the company) and an optional date flag")]
struct Args {
    #[arg(short, long)]
    company: Option<String>,
    #[arg(short, long, default_value_t = String::from(""))] // Optional date in YYYY-MM-DD format
    date: String,
    #[arg(short, long)]
    maildraft: bool,
    #[arg(long, default_value_t = String::from("config.yaml"))] // Optional date in YYYY-MM-DD format
    config: String,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let date = args.date;
    let config_name = &args.config;
    let config = read_config(&config_name)?;

    db::create_db_if_needed()?;

    let billdate = date_utils::parse_date_or_default(&date)?;
    let billdate_formatted = &billdate.format("%d.%m.%Y").to_string();
    let duedate = date_utils::calculate_due_date(billdate)?;
    let duedate_formatted = &duedate.format("%d.%m.%Y").to_string();
    
    

    let basedir_data = Path::new("data");
    let subdir_data_str = format!(
        "{year}-{month:02}",
        year = billdate.year(),
        month=billdate.month(),
    );
    let data_dir = &basedir_data.join(&subdir_data_str);
    let binding = data_dir.to_string_lossy().replace("data", "bills");
    let pdfdir = Path::new(&binding);

    println!("Data dir is {:?}", data_dir);

    let mut all_companies: Vec<String> = Vec::new();

    let company_str = args.company;

    match company_str {
        Some(s) => {
            all_companies.push(s)
        },
        None => {
            all_companies = csv_reader::find_all_companies(data_dir)?;
        }
    }
    println!("Running for the following companies: {:#?}", all_companies);

    // We shuffle the companies to shuffle the bill numbers
    let mut rng = thread_rng();
    all_companies.shuffle(&mut rng);

    // let mut billcounter = 1;

    for company_str in all_companies {
        println!("Processing {:}", &company_str);
        let mut file_str = company_str.clone();
        if !file_str.ends_with(".csv") {
            file_str.push_str(".csv");
        }

        let csv_path = data_dir.join(file_str);
        let company = String::from(&company_str);

        println!("Trying to read csv from {:?}", &csv_path);

        let csv_data = csv_reader::read_csv(&csv_path)?;

        let minutes_total = csv_reader::extract_minutes_total(&csv_data)?;

        if minutes_total == 0 {
            println!("{:?} has no entries. This is expected for some. Skipping\n", &company_str);
            continue;
        }

        let (billnr, billnr_int) = match db::get_billnr_if_exists(&company, &billdate)? {
            Some((billnr, billnr_int)) => (billnr, billnr_int),
            None => {
                let (billnr, billnr_int) = db::get_new_billnr(&billdate, &subdir_data_str)?;
                (billnr, billnr_int)
            }
        };
        println!("The bill number is {:?}", billnr);

        let company_config = config_reader::get_company_config(&config_name, &company_str)?;

        let amounts = calculate::calculate_amounts(&minutes_total, &company_config.hourly_fee)?;

        let decimal_amount_str = calculate::to_euro_string(&amounts.total)?;

        let qrcode = qrcode::create_qrcode(&config.bank_config, &decimal_amount_str, &billdate, &billnr, &config.bill_config.color)?;

        let pdf_content = pdf_gen::Content {
            company: company.to_string(),
            billnr: billnr.clone(),
            vat: amounts.vat.clone(),
            date: billdate_formatted.clone(),
            due: duedate_formatted.clone(),
            qrcode,
            hourly_fee: amounts.hourly_fee,
            data_dir: data_dir.to_string_lossy().into_owned(),
            config_name: config_name.clone(),
        };

        let pdf_data = pdf_gen::generate_pdf(pdf_content)?;
        println!("Generated pdf");

        // let signed_pdf_data = sign::sign_pdf(pdf_data)?;

        let xml = ebill::create_ebill_xml(&billnr, &amounts, billdate, duedate, &config.bill_config, &company_config, &config.bank_config)?;

        let pdf_with_xml = ebill::add_xml_to_pdf(&pdf_data, xml)?;
    
        let saved_pdf_filename = pdf_gen::save_pdf(&pdf_with_xml, pdfdir, billdate, &company_str)?;

        let _ = db::add_to_db(&company, &billdate, &billnr, &amounts.total, &decimal_amount_str, &billnr_int);

        if args.maildraft {
            let _ = mail::create_mail_draft(&config.mailconfig, &company_config, pdf_data, &saved_pdf_filename)?;
        }
        
        println!("{:}: Done\n", &company_str);
    }

    // println!("\nTABLE AFTER EVERYTHING:\n");

    // let _ = db::print_all_db_entries()?;
    Ok(())
}