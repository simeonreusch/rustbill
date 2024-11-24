use chrono::NaiveDate;
use qrcode::QrCode;
use qrcode::render::svg;
use thiserror::Error;

use crate::config_reader::BankConfig;

#[derive(Debug, Error)]
pub enum QRGenErrors{
    #[error("QrCode could not be created")]
    QrError(#[from] qrcode::types::QrError)
}
type QrResult<T> = Result<T, QRGenErrors>;

pub fn create_qrcode(bank_config: BankConfig, amount_total: &str, company: &str, billdate: &NaiveDate) -> QrResult<String> {
    
    let amount_formatted = String::from("EUR") + &amount_total;

    let billdate_formatted = billdate.format("%d.%m.%Y").to_string();

    let subject = String::from("")+ &company + &String::from(" IT RE vom ") + &billdate_formatted;
    
    let raw_string = format!(
        "BCD\n001\n2\nSCT\n{bic}\n{issuer}\n{iban}\n{amount}\nSCVE\n{subject}\n",
        bic = bank_config.bic,
        issuer = bank_config.name,
        iban= bank_config.iban,
        amount = amount_formatted,
        subject = subject,
    );
    let byte_slice: &[u8] = raw_string.as_bytes();

    let code = QrCode::new(byte_slice);

    let image = code?.render::<svg::Color>().build();

    Ok(image)

    // let res = fs::write("/Users/simeon/Desktop/test.svg", image);

    // match image {
    //     Ok(image) => Ok(image), 
    //     Err(_)  => Err(QRGenErrors::SVGSaveError)
    // }
}