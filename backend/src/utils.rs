use rocket::data::ToByteUnit;
use rocket::form::{self, DataField, FromFormField};
use serde_json::Value;
use std::error::Error;

pub struct JsonValue(pub Value);

#[derive(Debug)]
struct JsonParseError(String);

impl std::fmt::Display for JsonParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "JSON parse error: {}", self.0)
    }
}

impl Error for JsonParseError {}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for JsonValue {
    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        let contents = field.data.open(2.mebibytes())
            .into_string()
            .await
            .map_err(|e| {
                let error = JsonParseError(e.to_string());
                form::Error::custom(error)
            })?;
        
        let value: Value = serde_json::from_str(&contents).map_err(|e| {
            let error = JsonParseError(e.to_string());
            form::Error::custom(error)
        })?;

        Ok(JsonValue(value))
    }
}
