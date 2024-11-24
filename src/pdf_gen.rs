use derive_typst_intoval::{IntoDict, IntoValue};
use typst_as_lib::TypstTemplate;
use typst::foundations::{Bytes, Dict, IntoValue};
use typst::foundations::Smart;
use typst::text::Font;
use typst_pdf::{self, PdfOptions, PdfStandard, PdfStandards};

static TEMPLATE_FILE: &str = include_str!("../templates/invoice.typ");
static FONT: &[u8] = include_bytes!("../templates/Akrobat-Regular.otf");
static FONTBOLD: &[u8] = include_bytes!("../templates/Akrobat-Bold.otf");

#[derive(Debug, Clone, IntoValue, IntoDict)]
pub struct Content {
    pub company: String,
    pub billnr: i32,
    pub vat: f64,
    pub date: String,
    pub due: String,
    pub qrcode: String,
}

impl From<Content> for Dict {
    fn from(value: Content) -> Self {
        value.into_dict()
    }
}

pub fn generate_pdf(data: Content) -> Vec<u8> {
    let font = Font::new(Bytes::from(FONT), 0)
        .expect("Could not parse font!");

    let fontbold = Font::new(Bytes::from(FONTBOLD), 0)
        .expect("Could not parse font!");

    let template = TypstTemplate::new(vec![font, fontbold], TEMPLATE_FILE).with_file_system_resolver(".");

    let doc = template
        .compile_with_input(data)
        .output
        .expect("typst::compile() returned an error!");

    let pdf_standard = [PdfStandard::A_2b];
    let pdf_standards = PdfStandards::new(&pdf_standard).unwrap();
    let pdf_options: PdfOptions = PdfOptions {standards: pdf_standards, page_ranges: None, timestamp: None, ident: Smart::Auto};
    let pdf = typst_pdf::pdf(&doc, &pdf_options)
        .expect("Could not generate pdf.");
    pdf
}


