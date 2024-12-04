use minijinja::{Environment, context};
use chrono::{NaiveDate};
use std::io::Cursor;
use crate::config_reader::{BankConfig, BillConfig, CompanyConfig};
use lopdf::{Document, Object, Dictionary, Stream};
use crate::calculate::Amounts;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum XMLError {
    #[error("io Error")]
    IoError(#[from] std::io::Error),
    #[error("pdf bytestream read error")]
    ByteStreamError(#[from] lopdf::Error),
    #[error("Template error")]
    TemplateError(#[from] minijinja::Error),
}

type XMLResult<T> = Result<T, XMLError>;


pub fn add_xml_to_pdf(input_bytes: &Vec<u8>, xml_content: String) -> XMLResult<Vec<u8>> {
    let mut doc = Document::load_mem(input_bytes)?;
    
    let xml_stream = Stream::new(
        Dictionary::from_iter(vec![
            ("Type", Object::Name(b"EmbeddedFile".to_vec())),
            ("Subtype", Object::Name(b"text/xml".to_vec())),
        ]),
        xml_content.as_bytes().to_vec(),
    );

    let xml_stream_id = doc.add_object(xml_stream);

    // Create a file specification for the embedded XML
    let file_spec = Dictionary::from_iter(vec![
        ("Type", Object::Name(b"Filespec".to_vec())),
        ("F", Object::String(
            b"factur-x.xml".to_vec(),
            lopdf::StringFormat::Literal,
        )),
        ("EF", Dictionary::from_iter(vec![
            ("F", Object::Reference(xml_stream_id)),
        ]).into()),
    ]);

    let file_spec_id = doc.add_object(file_spec);
        
    let catalog_obj = doc.catalog_mut()?;


    let names = Object::Array(vec![
        Object::String(b"factur-x.xml".to_vec(), lopdf::StringFormat::Literal,),
        Object::Reference(file_spec_id)
    ]);

    let embedded_files = Dictionary::from_iter(vec![("Names", names)]);

    let obj = Dictionary::from_iter(vec![("EmbeddedFiles", Object::from(embedded_files))]);

    catalog_obj.set("Names", obj);

    let mut buffer = Vec::new();
    doc.save_to(&mut Cursor::new(&mut buffer))?;
    
    println!("Added Zugferd xml to pdf bytestream");
    Ok(buffer)

}

pub fn create_ebill_xml(billnr:&str, amounts: &Amounts, bill_date: NaiveDate, due_date: NaiveDate, bill_config: &BillConfig, company_config: &CompanyConfig, bank_config: &BankConfig) -> XMLResult<String> {
    let mut env = Environment::new();

    println!("Creating Zugferd xml using template");

    env.add_template("ebill", include_str!("../templates/ebill.xml"))?;

    let amount_net = (amounts.net * 100.0).ceil() / 100.0;
    let amount_vat = (amounts.vat * 100.0).ceil() / 100.0;
    let amount_total = (amounts.total * 100.0).ceil()  / 100.0;

    let bill_date_formatted = &bill_date.format("%Y-%m-%d").to_string();
    let due_date_formatted = &due_date.format("%Y-%m-%d").to_string();


    let template = env.get_template("ebill")?;
    let xml = template.render(context! { 
        issuer_name => bill_config.name, 
        issuer_company => bill_config.company,
        issuer_street => bill_config.street,
        issuer_city => bill_config.city,
        issuer_postcode => bill_config.postcode,
        issuer_vat_id => bill_config.vat_id,
        issuer_tax_id => bill_config.tax_id,
        issuer_mail => bill_config.email,
        issuer_country_code => bill_config.country,
        issuer_phone => bill_config.telephone,
        issuer_iban => bank_config.iban,
        issuer_bic => bank_config.bic,
        issuer_account_holder => bank_config.name,
        receiver_mail => company_config.email,
        receiver_name => company_config.address.name,
        receiver_street => company_config.address.addressline,
        receiver_city => company_config.address.city,
        receiver_postcode => company_config.address.postcode,
        bill_item => bill_config.bill_item,
        bill_item_description => bill_config.bill_item_description,
        quantity => amounts.hours_total,
        hourly_fee => amounts.hourly_fee,
        amount_net => amount_net,
        amount_vat => amount_vat,
        amount_total => amount_total,
        bill_number => billnr,
        bill_date => bill_date_formatted,
        due_date => due_date_formatted,
    })?;


    Ok(xml)
}