use x509_certificate::{CapturedX509Certificate, InMemorySigningKeyPair};
use cryptographic_message_syntax::SignerBuilder;
use pdf_signing::{PDFSigningDocument, UserSignatureInfo};
use lopdf::{Document, Object, Dictionary};
use std::io::Cursor;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SignError {
    #[error("io Error")]
    IoError(#[from] std::io::Error),
    #[error("Certifcate error")]
    X509CertificateError(#[from] x509_certificate::X509CertificateError),
    #[error("Error while inserting form")]
    FormError(#[from] FormError),
    #[error("Signing error")]
    SigningCerror(),
}

#[derive(Debug, Error)]
pub enum FormError {
    #[error("Error while adding a signature form to the pdf data")]
    FormCreationError(#[from] lopdf::Error),
    #[error("Error while creating the updated pdf")]
    FormInsertError(#[from] std::io::Error),
}

#[allow(dead_code)]
pub fn sign_pdf(pdf_data: Vec<u8>) -> Result<Vec<u8>, SignError> {

    println!("initial length: {:}", pdf_data.len());
    let pdf_data = add_form_field_to_pdf(&pdf_data)?;
    println!("with field length: {:}", pdf_data.len());

    let cert_str = std::fs::read_to_string("./certs/pdf_cert.crt")?;
    let cert: CapturedX509Certificate = CapturedX509Certificate::from_pem(cert_str)?;
    let privkey_str = std::fs::read_to_string("./certs/pkcs8.pem")?;
    let privkey = InMemorySigningKeyPair::from_pkcs8_pem(&privkey_str)?;
    let signer = SignerBuilder::new(&privkey, cert);

    let users_signature_info = vec![
        UserSignatureInfo {
            user_id: "274".to_owned(),
            user_name: "Simeon Reusc".to_owned(),
            user_email: "simeon.reusch@waytoosoon.de".to_owned(),
            user_signature: std::fs::read("./certs/signature.png").unwrap(),
            user_signing_keys: signer.clone(),
        },
    ];

    let pdf_filename = "output_signed.pdf";
    let mut pdf_signing_document =
        PDFSigningDocument::read_from(&*pdf_data, pdf_filename.to_owned()).unwrap();
    let pdf_data_signed = pdf_signing_document
        .sign_document(users_signature_info);

    match pdf_data_signed {
        Ok(pdf_data_signed) => {
            println!("All smooth");
            if pdf_data.len() == pdf_data_signed.len() {
                eprintln!("The signed and unsigned pdf are identical in size. No fields to sign could be found");
                return Ok(pdf_data_signed)
            }
            Ok(pdf_data_signed)
        }, 
        Err(_) => {
            eprintln!("Something went wrong");
            Err(SignError::SigningCerror())
        },
    }
}

#[allow(dead_code)]
fn add_form_field_to_pdf(input_bytes: &Vec<u8>) -> Result<Vec<u8>, FormError> {
    // Load the PDF document
    let mut doc = Document::load_mem(input_bytes)?;

    // Define a new widget annotation for the text field
    let widget_annot = Dictionary::from_iter(vec![
        ("Type", Object::Name(b"Annot".to_vec())),
        ("Subtype", Object::Name(b"Widget".to_vec())),
        ("Rect", Object::Array(vec![100.into(), 700.into(), 300.into(), 750.into()])), // Adjust the rectangle values as needed
        ("FT", Object::Name(b"Sig".to_vec())), // Field Type: Signature
        ("T", Object::String(b"signature_field".to_vec(), lopdf::StringFormat::Literal)), // Field Name
        ("Ff", Object::Integer(1)), // Field Flags, set to 0 for default
    ]);

    let pages = doc.get_pages();

    if let Some((_page_number, &page_id)) = pages.iter().next() {
        let widget_id = doc.add_object(Object::Dictionary(widget_annot));

        if let Ok(Object::Dictionary(page_dict)) = doc.get_object_mut(page_id) {
            let annots = page_dict.get_mut(b"Annots").unwrap();

            match annots {
                Object::Array(ref mut arr) => {
                    arr.push(Object::Reference(widget_id));
                }
                _ => {
                    page_dict.set("Annots", Object::Array(vec![Object::Reference(widget_id)]));
                }
            }
        }
    }

    let mut buffer = Cursor::new(Vec::new());
    doc.save_to(&mut buffer)?;
    let res_vec = buffer.into_inner();

    Ok(res_vec)
}