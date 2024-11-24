#import sys: inputs
#import "templates/template.typ": letter-simple, sum_minutes, sum_amounts, format_currency, configread, footerdef, minutes_to_hours, overview_short, overview_detailed

#set page(paper: "a4")
#set text(font: "Akrobat", 11pt)

#let params = inputs.v
// #let last_index = content.len() - 1

#let recipient = params.at("company", default: "Allerland")
#let invoice_nr = params.at("billnr", default:  "2024-TEST")
#let vat = float(params.at("vat", default: 1))
#let billdate = params.at("date", default: datetime.today().display("[day].[month].[year]"))
#let due_date = params.at("due", default: datetime(day: 15, month: 11, year: 2030).display("[day].[month].[year]"))


#recipient
#invoice_nr
#vat
#billdate
#due_date