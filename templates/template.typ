// Author: Simeon Reusch (mail@simeonreusch.com)
// License: BSD-3-Clause

#let letter-formats = (
  "DIN-5008-A": (
    folding-mark-1-pos: 87mm,
    folding-mark-2-pos: 87mm + 105mm,
    header-size: 27mm,
  ),
  
  "DIN-5008-B": (
    folding-mark-1-pos: 105mm,
    folding-mark-2-pos: 105mm + 105mm,
    header-size: 45mm,
  ),
)

#let letter-generic(
  format: "DIN-5008-B",
  
  header: none,
  footer: none,
  
  folding-marks: false,
  hole-mark: true,
  
  address-box: none,
  information-box: none,
  logo-box: none,
  
  reference-signs: none,
  
  page-numbering: (current-page, page-count) => {
    "Page " + str(current-page) + " of " + str(page-count)
  },

  margin: (
    left:   25mm,
    right:  20mm,
    top:    20mm,
    bottom: 20mm,
  ),
  
  body,
) = {
  if not letter-formats.keys().contains(format) {
    panic("Invalid letter format! Options: " + letter-formats.keys().join(", "))
  }
  
  margin = (
    left:   margin.at("left",   default: 25mm),
    right:  margin.at("right",  default: 20mm),
    top:    margin.at("top",    default: 20mm),
    bottom: margin.at("bottom", default: 20mm),
  )
  
  set page(
    paper: "a4",
    flipped: false,
    
    margin: margin,
    
    background: {
      if folding-marks {
        // folding mark 1
        place(top + left, dx: 5mm, dy: letter-formats.at(format).folding-mark-1-pos, line(
            length: 2.5mm,
            stroke: 0.25pt + black
        ))
        
        // folding mark 2
        place(top + left, dx: 5mm, dy: letter-formats.at(format).folding-mark-2-pos, line(
            length: 2.5mm,
            stroke: 0.25pt + black
        ))
      }
      
      if hole-mark {
        // hole mark
        place(left + top, dx: 5mm, dy: 148.5mm, line(
          length: 4mm,
          stroke: 0.25pt + black
        ))
      }

      

    },
 
    footer: context {
      show: pad.with(top: -30pt, bottom: 12pt)
      grid(
        columns: (1fr),
        rows: (auto, auto),
        row-gutter: 6pt,

        let current-page = counter(page).get().first(),
        let page-count = counter(page).final().first(),

        if page-count > 1 and current-page > 0 {
          align(right, numbering(page-numbering, current-page, page-count))
        },
        if current-page == 1 {
          footer
        }
      )
    },

  )
  
  place(top+left, dx: -16mm, dy: -5mm, logo-box)
  pad(top: -margin.top, left: -margin.left, right: -margin.right, {
    grid(
      columns: 100%,
      rows: (letter-formats.at(format).header-size, 45mm),


      header,
      
      pad(left: 20mm, right: 10mm, {
        grid(
          columns: (85mm, 75mm),
          rows: 45mm,
          column-gutter: 20mm,
          
          address-box,
          
          pad(top: 5mm, information-box)
        )
      }),
    )
  })

  v(12pt)

  if (reference-signs != none) and (reference-signs.len() > 0) {
    grid(
      
      columns: (1fr,1fr,1fr),
      align: (left, center, right),
      rows: 12pt * 3,
      gutter: 82pt,
      
      ..reference-signs.map(sign => {
        let (key, value) = sign
        
        text(size: 10pt, key)
        linebreak()
        text(size: 10pt, value)
      })
    )
  }
  
  body
}


#let header-simple(name, address, extra: none) = {
  set text(size: 10pt)

  if name != none {
    strong(name)
    linebreak()
  }
  
  if address != none {
    address
    linebreak()
  }

  if extra != none {
    extra
  }
}

#let sender-box(name: none, company: none, address) = rect(width: 85mm, height: 5mm, stroke: none, inset: 0pt, {
  set text(size: 7pt)
  set align(horizon)
  
  pad(left: 5mm, underline(offset: 2pt, {
    if company != none {
      company
    }

    if (name != none) and (address != none) {
      ", "
    }

    if address != none {
      address
    }
  }))
})


#let annotations-box(content) = {
  set text(size: 7pt)
  set align(bottom)
  
  pad(left: 5mm, bottom: 2mm, content)
}

#let logo-box(content) = {
  set image(width: 100%)
  set align(top)
  pad(left: 15mm, content)
}

#let recipient-box(content) = {
  set text(size: 10pt)
  set align(top)
  
  pad(left: 5mm, content)
}

#let address-duobox(sender, recipient) = {
  grid(
    columns: 1,
    rows: (17.7mm, 27.3mm),
      
    sender,
    recipient,
  )
}

#let address-tribox(sender, annotations, recipient, stamp: false) = {
  if stamp {
    grid(
      columns: 1,
      rows: (5mm, 12.7mm + (4.23mm * 2), 27.3mm - (4.23mm * 2)),
      
      sender,
      annotations,
      recipient,
    )
  } else {
    grid(
      columns: 1,
      rows: (5mm, 12.7mm, 27.3mm),
      
      sender,
      annotations,
      recipient,
    )
  }
}

#let format_currency(number, locale: "de") = {
    let precision = 2
    assert(precision > 0)
    let s = str(calc.round(number, digits: precision))
    let after_dot = s.find(regex("\..*"))
    if after_dot == none {
      s = s + "."
      after_dot = "."
    }
    for i in range(precision - after_dot.len() + 1){
      s = s + "0"
    }
    // fake de locale
    if locale == "de" {
      s.replace(".", ",")
    } else {
      s
    }
  }

#let sum_minutes(array, idx) = {
  for entry in array {
    let minutes = entry.at(idx)
    (int(minutes),)
  }.sum()
}

#let sum_amounts(array, idx, hourly_fee) = {
  for entry in array {
    let minutes = entry.at(idx)
    let amount = float(minutes) / 60 * hourly_fee
    (amount,)
  }.sum()
}

#let minutes_to_hours(minutes) = {
  let minutes_float = float(minutes)
  let hours_float = minutes_float/60
  hours_float
}

#let letter-simple(
  format: "DIN-5008-B",
  
  header: none,
  footer: none,

  folding-marks: true,
  hole-mark: true,
  
  sender: (
    company: none,
    name: none,
    address: none,
    extra: none,
  ),
  
  recipient: none,
  logo: none,

  stamp: false,
  annotations: none,
  
  information-box: none,
  reference-signs: none,
  
  date: none,
  subject: none,

  page-numbering: (current-page, page-count) => {
    "Seite " + str(current-page) + " von " + str(page-count)
  },

  margin: (
    left:   25mm,
    right:  25mm,
    top:    20mm,
    bottom: 30mm,
  ),

  font: "Akrobat",

  body,
) = {
  margin = (
    left:   margin.at("left",   default: 25mm),
    right:  margin.at("right",  default: 25mm),
    top:    margin.at("top",    default: 20mm),
    bottom: margin.at("bottom", default: 20mm),
  )
  
  // Configure page and text properties.
  set document(
    title: subject,
    author: sender.company,
  )

  set text(font: font, hyphenate: false)

  if header == none {
    header = pad(
      left: margin.left,
      right: margin.right,
      top: margin.top,
      bottom: 5mm,

      align(bottom + right, header-simple(
        sender.company,
        if sender.address != none {
          sender.address.split(", ").join(linebreak())
        } else {
          "lul?"
        },
        extra: sender.at("extra", default: none),
      ))
    )
  }

  let sender-box      = sender-box(name: sender.name, company: sender.company, sender.address)
  let annotations-box = annotations-box(annotations)
  let recipient-box   = recipient-box(recipient)
  let logo-box = logo-box(logo)

  let address-box     = address-tribox(sender-box, annotations-box, recipient-box, stamp: stamp)
  if annotations == none and stamp == false {
    address-box = address-duobox(align(bottom, pad(bottom: 0.65em, sender-box)), recipient-box)
  }
  
  letter-generic(
    format: format,
    
    header: header,
    footer: footer,
    logo-box: logo-box,

    folding-marks: folding-marks,
    hole-mark: hole-mark,
    
    address-box:     address-box,
    information-box: information-box,

    reference-signs: reference-signs,

    page-numbering: page-numbering,
    
    {
      // Add the date line, if any.
      if date != none {
        align(right, date)
        v(0.65em)
      }
      
      // Add the subject line, if any.
      if subject != none {
        pad(right: 10%, strong(subject))
        v(0.65em)
      }
      
      set par(justify: true)
      body
    },

    margin: margin,
  )
}

#let configread(contents, recipient) = (
  header: contents.bill_config.header,
  hourly_fee: contents.companies.at(recipient).hourly_fee,
  sender_name: contents.bill_config.name,
  sender_company: contents.bill_config.company,
  sender_city: contents.bill_config.city,
  sender_postcode: contents.bill_config.postcode,
  sender_street: contents.bill_config.street,
  sender_email: contents.bill_config.email,
  sender_phone: contents.bill_config.telephone,
  sender_phone_concise: contents.bill_config.telephone_concise,
  sender_web: contents.bill_config.company_web,
  recipient_name: contents.companies.at(recipient).address.name,
  recipient_city: contents.companies.at(recipient).address.city,
  recipient_postcode: contents.companies.at(recipient).address.postcode,
  recipient_street: contents.companies.at(recipient).address.addressline,
  bank_name: contents.bank_config.bank_name,
  iban: contents.bank_config.iban,
  vat_id: contents.bill_config.vat_id,
  tax_id: contents.bill_config.tax_id,
  color: rgb(contents.bill_config.color),
  billtext: contents.bill_config.text,
)

#let footerdef(config) =  {
  set align(left)
  set text(size: 9pt)
  grid(
    columns: (1fr,1fr),
    rows: (auto, auto,auto),
    gutter: 8pt,
    grid.hline(y: 0, stroke: 0.5pt + config.color, position:top),
    align: left,
    [],
    [],
    [#config.bank_name],
    grid.cell("Umsatzsteuer-ID: " + config.vat_id, align: right),
    [Kontoinhaber: #config.sender_name],
    grid.cell("Steuernummer: " + config.tax_id, align: right),
    grid.cell(config.iban, align: left),

  ) 
  set align(left)
}

#let overview_short(hours_total, amount_total, vat, amount_with_vat, config) = table(
  align: (left, left, right),
  columns: (auto, auto, auto),
  inset: 5pt,
  table.hline(stroke: config.color + 0.5pt),
  table.vline(stroke: config.color + 0.5pt),
  table.header(
    [*Pos.*], [*Bezeichnung*], [*Betrag*]
  ),
  table.vline(stroke: config.color + 0.5pt),
  [1],
  [Supportdienstleistungen / #hours_total Stunden zu #config.hourly_fee € (Netto)],
  [#format_currency(amount_total) €],
  [2],
  [Umsatzsteuer (19 %)],
  [#format_currency(vat) €],
  table.hline(stroke: config.color + 0.5pt),
  table.cell(colspan: 2)[*Gesamtbetrag*],
  [*#format_currency(amount_with_vat) €*],
  table.hline(stroke: config.color + 0.5pt),
)

#let format_unsplit_str(input) = {
  // input
  str(input + ",00 €")
}

#let format_split_str(input1, input2) = {
  input1 + "," + input2 + " €"
}

#let format_currency(input) = {
  let input = calc.round(float(input), digits: 2)
  let input_str = str(input)
  let split_str = input_str.split(".")
  let len = split_str.len()
  if len == 1 {format_unsplit_str(split_str.at(0))} else {format_split_str(split_str.at(0), split_str.at(1))}
}

#let overview_detailed(data, minutes_total, amount_total, pos, hourly_fee, custom_color) = table(
  align: (left, left, left, right, right),
  columns: (auto,auto,auto, auto, auto),
  table.hline(stroke: custom_color + 0.5pt),
  table.vline(stroke: custom_color + 0.5pt),
  table.header(
    [*Pos.*], [*Datum*],[*Bezeichnung*], [*Minuten*], [*Betrag*]
  ),
  // let pos = 0,
  ..for (date, minutes, description) in data {
    // let date = datetime(day: int(day), month: int(month), year: int(year)).display("[day].[month].[year]")
    let amount = (float(minutes)/60*hourly_fee)
    let amount_formatted = format_currency((amount))
    (str(int(pos)), date, description, minutes, amount_formatted)
    pos = pos + 1
  }, 
  table.vline(stroke: custom_color + 0.5pt),
  table.hline(stroke: custom_color + 0.5pt),
  table.cell(colspan: 3)[*Summe*],
  [*#minutes_total*],[*#format_currency(amount_total)*],
  table.hline(stroke: custom_color + 0.5pt),
)