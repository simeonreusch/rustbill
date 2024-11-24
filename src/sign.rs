use x509_certificate::{CapturedX509Certificate, InMemorySigningKeyPair};
use cryptographic_message_syntax::SignerBuilder;
use std::{fs::File, io::Write};
use pdf_signing::{PDFSigningDocument, UserSignatureInfo};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SignError {
    #[error("io Error")]
    IoError(#[from] std::io::Error),
    #[error("Certifcate error")]
    X509CertificateError(#[from] x509_certificate::X509CertificateError),
}

pub fn sign_pdf(pdf_data: Vec<u8>) -> Result<(), SignError> {
    let cert_str = std::fs::read_to_string("./certs/cert.pem")?;
    let cert: CapturedX509Certificate = CapturedX509Certificate::from_pem(cert_str)?;
    let privkey_str = std::fs::read_to_string("./certs/key.pem")?;
    let privkey = InMemorySigningKeyPair::from_pkcs8_pem(&privkey_str)?;
    let signer = SignerBuilder::new(&privkey, cert);
    let users_signature_info = vec![
        UserSignatureInfo {
            user_id: "1".to_owned(),
            user_name: "Simeon Reusch".to_owned(),
            user_email: "simeon.reusch@waytoosoon.de".to_owned(),
            user_signature: std::fs::read("./certs/asdf.png")?,
            user_signing_keys: signer.clone(),
        },
    ];

    let pdf_filename = "output_signed.pdf";
    let mut pdf_signing_document =
        PDFSigningDocument::read_from(&*pdf_data, pdf_filename.to_owned()).unwrap();
    let pdf_file_data = pdf_signing_document
        .sign_document(users_signature_info)
        .unwrap();
    println!("Signed pdf");

    let mut pdf_file = File::create(pdf_filename)?;
    let _ = pdf_file.write_all(&pdf_file_data);
    Ok(())
}