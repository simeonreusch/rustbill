# rustbill
`rustbill` does a few things a Python tool I wrote did; and I wanted to learn Rust. 

## The idea
The idea is to have a web frontend where you track hours worked, and a list of companies that you regularly bill in your config. If you execute rustbill, it creates invoices for all logged activity (stored as csv tables), renders these to pdf and it creates mail drafts with your invoices, as well as logging the bill to a db for future bookkeeping.

## What it does
- Read a csv per customer (hours worked) ✅
- Read a config (YAML) ✅
- Create invoice PDF with typst ✅
- Create e-bill-XML and attach to pdf 🟡 (standard needed (EN16931 not yet fully - implemented in Rust))
- Log to database ✅ (needed to switch from a json-db to a SQL db due to paucity of packages)
- Upload mail draft via IMAP ✅

## Speedup vs Python:

Everything except IMAP: 50x
IMAP: 6x