use crate::error::InsulinLookupError;
use crate::insulin::{parse_dose, save_dose, Dose};
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[get("/lastdose")]
fn lastdose() -> Result<Json<Dose>, Json<InsulinLookupError>> {
    let dose = match parse_dose() {
        Ok(d) => d,
        Err(e) => {
            return Err(Json(e));
        }
    };
    println!("{:?}", dose);
    Ok(Json(dose))
}

#[derive(FromForm, Deserialize)]
#[serde(crate = "rocket::serde")]
struct DoseRequest {
    dose: u8,
    key: Option<String>,
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
