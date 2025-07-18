use ical::IcalParser;
use ical::parser::ical::component::IcalCalendar;
use std::fs::File;
use std::io::BufReader;

mod sleep_table;
use sleep_table::SleepTable;

fn main() -> anyhow::Result<()> {
    let buf = BufReader::new(File::open("basic.ics")?);
    let reader = IcalParser::new(buf);

    for line in reader {
        if let Ok(ical) = line {
            let table = make_table(ical)?;
            println!("{}", table);
        } else {
            eprintln!("parse error: {:?}", line?);
        }
    }

    Ok(())
}

fn make_table(ical: IcalCalendar) -> anyhow::Result<String> {
    let mut table = SleepTable::new();

    for event in ical.events.into_iter() {
        let e = event.try_into()?;
        table.eat(&e);
    }

    Ok(format!("{:?}", table))
}
