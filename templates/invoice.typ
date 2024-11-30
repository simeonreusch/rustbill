#import sys: inputs
#import "templates/template.typ": letter-simple, sum_minutes, sum_amounts, format_currency, configread, footerdef, minutes_to_hours, overview_short, overview_detailed


#let recipient = inputs.at("company")
#let invoice_nr = inputs.at("billnr")
#let vat = float(inputs.at("vat"))
#let billdate = inputs.at("date")
#let due_date = inputs.at("due")
#let qrcode = inputs.at("qrcode")
#let hourly_fee = inputs.at("hourly_fee")
#let data_dir = inputs.at("data_dir")

#let data_with_header = csv(data_dir + "/" + recipient + ".csv", delimiter: ";")
#let data = data_with_header.slice(1)

#set text(lang: "de")

#let config = configread(
  yaml("/config.yaml"), recipient
)
#let address = config.sender_street + ", " + config.sender_postcode + " " + config.sender_city

#show: letter-simple.with(
  sender: (
    name: config.sender_name,
    company: config.sender_company,
    address: address,
    extra: [
      #link("tel:" + config.sender_phone_concise)[#config.sender_phone]\
      #link("mailto:" + config.sender_email)[#config.sender_email]\
      #link("https://" + config.sender_web)[#config.sender_web]\
    ],
  ),
  logo: image("templates/logo.svg", width: 25mm),

  footer: footerdef(config),

  folding-marks: false,
  hole-mark: false,

 
  recipient: [
    #config.recipient_name\
    #config.recipient_street\
    #config.recipient_postcode #config.recipient_city\
  ],

  reference-signs: (
    ([Rechnungsnummer], [#invoice_nr]),
    ([Steuernummer], [#config.tax_id]),
    ([Datum], [#billdate]),
  ),
  subject: config.header,
)

Sehr geehrte Damen und Herren,

#config.billtext

#let minutes_total = sum_minutes(data, 1)
#let amount_total = sum_amounts(data, 1, hourly_fee)
#let hours_total = minutes_to_hours(minutes_total)
#let amount_with_vat = vat + amount_total

#set table(
  align: center,
  stroke: none,
  fill: (x, y) =>
    if y == 0 { config.color }
)


#overview_short(hours_total, amount_total, vat, amount_with_vat, config)

Bitte überweisen Sie den Gesamtbetrag von *#format_currency(amount_with_vat) €* innerhalb von 10 Werktagen -- also bis zum *#due_date* -- auf das angeführte Konto.
#v(0.3cm)


Mit freundlichen Grüßen

#v(0.5cm)

#config.sender_name

#v(0.5cm)

#image.decode(qrcode, width: 25mm, format: "svg")

Um die Rechnung zu begleichen, können Sie\
diesen Code mit ihrer Banking-App scannen.

#pagebreak()
= Stundenübersicht

#v(1cm)


#set table(
  stroke: none,
  fill: (x, y) =>
    if y == 0 { config.color },
  inset: (right: 1.5em)
)
#set par(justify: false)

#overview_detailed(data, minutes_total, amount_total, 1, hourly_fee, config.color)

