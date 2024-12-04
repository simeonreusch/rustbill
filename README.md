# rustbill
`rustbill` does a few things a Python tool I wrote did; and I wanted to learn Rust. 

## The idea
The idea is to have a web frontend where you track hours worked, and a list of companies that you regularly bill in your config. If you execute rustbill, it creates invoices for all logged activity (stored as csv tables), renders these to pdf. A ZUGFeRD-compliant electronic bill xml is created and attached to the pdf. Finally, it creates mail drafts with your invoices and logs the bill to an SQL database for future accounting.

## What it does
- Read a csv per customer (hours worked) ✅
- Read a config (YAML) ✅
- Create invoice PDF with [typst](https://typst.app) ✅
- Create e-bill-XML and attach to pdf ✅ ([EN16931](https://de.wikipedia.org/wiki/ZUGFeRD) compliant)
- sign PDF 🟡 (X509 crypt works, but adding a field to a PDF does not yet work)
- Log to database ✅ (needed to switch from a json-db to a SQL db due to paucity of packages)
- Upload mail draft via IMAP ✅

## Usage
- Move `./sample/config.yaml` to `./config.yaml` and modify to your needs.
- Compile with `cargo build -r`
- Copy the binary from `target/release/rustbill` to `.`
- Use the CLI with `./rustbill --help`. Without further parameters, all companies defined in your config are iterated over, and the date defaults to the last day of the current month.

The data is expected to lie in `./data/YYYY-MM/company_name.csv` within a `csv` table with three columns (`Date`, `Minutes`, `Description`), separated by `;`. A sample table can be found at `./sample/SampleCompany.csv`

## Speedup vs Python:
- Everything except IMAP: 50x
- IMAP: 6x