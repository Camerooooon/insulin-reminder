use core::fmt::{Display, Formatter};
use core::num::ParseIntError;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct InsulinLookupError {
    pub message: String,
}

impl From<std::io::Error> for InsulinLookupError {
    fn from(err: std::io::Error) -> Self {
        InsulinLookupError {
            message: format!("IO Error occured while reading file: {}", err),
        }
    }
}

impl From<ParseIntError> for InsulinLookupError {
    fn from(err: ParseIntError) -> Self {
        InsulinLookupError {
            message: format!("Parse error occured: {}", err),
        }
    }
}

impl Display for InsulinLookupError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.message)
    }
}
