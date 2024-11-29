use zugferd::{InvoiceBuilder,InvoiceTypeCode,CountryCode,CurrencyCode,SpecificationLevel};
use chrono::NaiveDate;
use crate::config_reader::BillConfig;
use crate::calculate::Amounts;


pub fn create_xml(amounts: Amounts, bill_date: NaiveDate, bill_config: &BillConfig) -> String {
    
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