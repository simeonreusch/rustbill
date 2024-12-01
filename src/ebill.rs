use typst::foundations::Dict;
use zugferd::{InvoiceBuilder,InvoiceTypeCode,CountryCode,CurrencyCode,SpecificationLevel};
use chrono::NaiveDate;
use std::io::Cursor;
use crate::config_reader::BillConfig;
use lopdf::{Document, Object, Dictionary, Stream};
use crate::calculate::Amounts;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum XMLError {
    #[error("io Error")]
    IoError(#[from] std::io::Error),
    #[error("pdf bytestream read error")]
    ByteStreamError(#[from] lopdf::Error),
}

type XMLResult<T> = Result<T, XMLError>;

pub fn create_xml(amounts: &Amounts, bill_date: NaiveDate, bill_config: &BillConfig) -> String {
    
    let mut invoice_builder = InvoiceBuilder::new();

    invoice_builder.set_business_process("process1")
        .set_invoice_type_code(InvoiceTypeCode::CommercialInvoice)
        .set_invoice_nr("15")
        .set_date_of_issue(bill_date)
        .set_buyer_reference("bla")
        .set_sellers_name(&*bill_config.company)
        .set_sellers_specified_legal_organization("LegalOrg-001")
        .set_sellers_postal_trade_address_country_code(CountryCode::Germany)
        .set_sellers_specified_tax_registration("DE123456789")
        .set_sellers_postal_trade_address_city_name(&*bill_config.city)
        .set_buyers_name("Buyer Inc.")
        .set_buyers_specified_legal_organization("LegalOrg-002")
        .set_buyers_order_specified_document("OD-2024-001")
        .set_invoice_currency_code(CurrencyCode::Euro);

    invoice_builder.set_monetary_summation_tax_basis_total_amount(amounts.net)
    .set_monetary_summation_tax_total_amount(amounts.vat)
    .set_monetary_summation_grand_total_amount(amounts.total)
    .set_monetary_summation_due_payable_amount(amounts.total);

    let mut xml_string: String = String::new();

    match invoice_builder.to_xml_string(SpecificationLevel::Minimum) {
        Ok(string_returned_by_function) => {
            xml_string = string_returned_by_function;
        },
        Err(e) => {
            println!("Something happened at the XML generation: {}",e);
        }
    }

    println!("Generated ZUGFeRD XML");

    xml_string
}


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
    
    println!("Added xml to pdf bytestream");
    Ok(buffer)

}