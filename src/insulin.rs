use std::fs::File;
use std::io::Read;
use chrono_tz::US::Pacific;
use chrono::{NaiveDateTime, Local, TimeZone};

pub fn get_lines() -> Vec<String> {
    // Read lines from file
    let mut file = File::open("/tmp/insulin").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content.split("\n").map(|x| x.to_string()).collect()
}

#[derive(Debug)]
struct Dose {
    time: i64,
    units: u8,
}

pub fn parse_dose() {
    for line in get_lines() {
        if line == "" {
            continue;
        }
        let mut parts = line.split(" ");
        let time = parts.next().unwrap().parse::<i64>().unwrap();
        let units = parts.next().unwrap();
        let dose = Dose {
            time: time,
            units: units.parse::<u8>().unwrap(),
        };
        let dt = NaiveDateTime::from_timestamp(dose.time, 0);
        let aware = Pacific.from_utc_datetime(&dt);
        println!("{:?}", aware.to_string());
    }
}
