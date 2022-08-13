use crate::error::InsulinLookupError;
use chrono::{NaiveDateTime, TimeZone};
use chrono_tz::US::Pacific;
use rocket::serde::Serialize;
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

pub fn read_file() -> Result<String, Error> {
    // Read lines from file
    let path = Path::new("/tmp/insulin");
    if path.exists() {
        let mut file = File::open("/tmp/insulin")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        contents.truncate(contents.len() - 1);
        Ok(contents)
    } else {
        Ok(String::new())
    }
}

trait Insulin {
    fn timestamp(&self) -> String;
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Dose {
    time: i64,
    units: u8,
}

pub fn parse_dose() -> Result<Dose, InsulinLookupError> {
    let rawdata = read_file()?;
    if rawdata.is_empty() {
        return Err(InsulinLookupError {
            message: "No data found".to_string(),
        });
    }
    let mut parts = rawdata.split(" ");
    let time = parts
        .next()
        .ok_or(InsulinLookupError {
            message: "Parse failed".to_string(),
        })?
        .parse::<i64>()?;
    let units = parts
        .next()
        .ok_or(InsulinLookupError {
            message: "Parse failed".to_string(),
        })?
        .parse::<u8>()
        .unwrap();
    let dose = Dose { time, units };
    println!("{:?}", dose.timestamp());
    Ok(dose)
}

impl Insulin for Dose {
    fn timestamp(&self) -> String {
        let dt = NaiveDateTime::from_timestamp(self.time, 0);
        let aware = Pacific.from_utc_datetime(&dt);
        aware.to_string()
    }
}
