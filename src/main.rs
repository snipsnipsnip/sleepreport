mod sleep_table;
use sleep_table::SleepTable;

use anyhow::Context;
use chrono::Local;
use ical::IcalParser;
use ical::parser::ical::component::IcalCalendar;
use ical_property;
use std::fs::File;
use std::io::BufReader;

fn main() -> anyhow::Result<()> {
    let buf = BufReader::new(File::open("basic.ics")?);
    let reader = IcalParser::new(buf);

    for vcal in reader {
        match vcal {
            Ok(ical) => {
                let table = make_table(ical)?;
                println!("{}", table);
            }
            Err(e) => {
                eprintln!("parse error: {:?}", e);
            }
        }
    }

    Ok(())
}

fn make_table(ical: IcalCalendar) -> anyhow::Result<String> {
    let mut table = SleepTable::new(Local::now()).context("init a sleep table")?;

    for event in ical.events.into_iter() {
        let e: ical_property::Event = event.try_into()?;
        if table.eat(&e).is_none() {
            eprintln!("failed to load a sleep: {:?}", e)
        }
    }

    Ok(table
        .build_csv_lines(6, &Local)
        .collect::<Vec<_>>()
        .join("\n"))
}
