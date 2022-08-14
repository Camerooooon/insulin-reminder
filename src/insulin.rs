use crate::error::InsulinLookupError;
use chrono::{NaiveDateTime, TimeZone};
use chrono_tz::US::Pacific;
use rocket::serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Error, Read};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn read_file() -> Result<String, Error> {
    // Read lines from file
    let path = Path::new("/tmp/insulin");
    if path.exists() {
        let mut file = File::open("/tmp/insulin")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    } else {
        Ok(String::new())
    }
}

pub trait Insulin {
    fn timestamp(&self) -> String;
    fn insulin_time(&self) -> bool;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Dose {
    pub time: u64,
    pub units: u8,
}

pub fn save_dose(dose: Dose) -> Result<(), std::io::Error> {
    let path = Path::new("/tmp/insulin");
    fs::write(path, format!("{} {}", dose.time, dose.units))?;
    Ok(())
}

pub fn parse_dose() -> Result<Dose, InsulinLookupError> {
    let rawdata = read_file()?;
    if rawdata.is_empty() {
        return Err(InsulinLookupError {
            message: "No data found".to_string(),
        });
    }
    println!("{}", rawdata);
    let mut parts = rawdata.split(" ");
    let time = parts
        .next()
        .ok_or(InsulinLookupError {
            message: "Parse failed".to_string(),
        })?
        .parse::<u64>()?;
    let units = parts
        .next()
        .ok_or(InsulinLookupError {
            message: "Parse failed".to_string(),
        })?
        .parse::<u8>()?;
    let dose = Dose { time, units };
    println!("{:?}", dose.timestamp());
    Ok(dose)
}

impl Insulin for Dose {
    fn timestamp(&self) -> String {
        let dt = NaiveDateTime::from_timestamp(self.time.try_into().expect("Backwards time"), 0);
        let aware = Pacific.from_utc_datetime(&dt);
        aware.to_string()
    }

    fn insulin_time(&self) -> bool {
        if (get_time() - self.time) > 60 * 60 * 24 {
            return true;
        }
        return false;
    }
}

fn get_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards?")
        .as_secs()
}
