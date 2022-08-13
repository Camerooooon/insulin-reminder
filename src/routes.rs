use crate::insulin::{Dose, parse_dose, InsulinLookupError};
use rocket::serde::json::Json;

#[get("/lastdose")]
fn lastdose() -> Result<Json<Dose>, Json<InsulinLookupError>> {
    let dose = match parse_dose() {
        Ok(d) => d,
        Err(e) => {
            return Err(Json(e));
        }
    };
    Ok(Json(dose))
}

#[post("/dose")]
fn dose() -> &'static str {
    "Dose"
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![lastdose, dose]
}
