use crate::prelude::*;
use serde::{Serialize, de::DeserializeOwned};

/// Helper to quickly convert json.
pub trait JsonHelper: Sized + Serialize + DeserializeOwned {
    fn to_json(self) -> Result<JsonValue, serde_json::Error> {
        serde_json::to_value(self)
    }
    fn from_json(v: JsonValue) -> Result<Self, serde_json::Error> {
        serde_json::from_value(v)
    }
}

/// Automatically implement for Serialize + DeserializeOwned.
impl<T> JsonHelper for T where T: Serialize + DeserializeOwned {}
