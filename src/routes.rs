use crate::error::InsulinLookupError;
use crate::insulin::{parse_dose, save_dose, Dose, Insulin};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(FromForm, Deserialize)]
#[serde(crate = "rocket::serde")]
struct DoseRequest {
    dose: u8,
    key: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct DoseResponse {
    dose: Option<Dose>,
    timestamp: Option<String>,
    error: Option<InsulinLookupError>,
    success: bool,
    insulin_time: bool,
    time_until: Option<i64>,
}

#[get("/lastdose")]
fn lastdose() -> Json<DoseResponse> {
    let dose = match parse_dose() {
        Ok(d) => d,
        Err(e) => {
            return Json(DoseResponse {
                dose: None,
                timestamp: None,
                error: Some(e),
                success: false,
                insulin_time: false,
                time_until: None,
            });
        }
    };
    println!("{:?}", dose);
    Json(DoseResponse {
        dose: Some(dose.clone()),
        timestamp: Some(dose.timestamp()),
        error: None,
        success: true,
        insulin_time: dose.insulin_time(),
        time_until: Some(dose.time_until_insulin()),
    })
}

#[post("/dose", format = "json", data = "<dosereq>")]
fn dose(dosereq: Json<DoseRequest>) -> String {
    let dose = Dose {
        units: dosereq.dose,
        time: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards?")
            .as_secs(),
    };
    match save_dose(dose) {
        Ok(_) => "saved".to_string(),
        Err(e) => e.to_string(),
    }
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![lastdose, dose]
}
