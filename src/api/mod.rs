pub mod ws;
pub mod http;
pub mod objects;
pub mod request;
pub mod response;
pub mod event;

mod bool_as_u8 {
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(data: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(*data as i32)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        match u8::deserialize(deserializer)? {
            0 => Ok(false),
            1 => Ok(true),
            other => Err(D::Error::invalid_value(serde::de::Unexpected::Unsigned(other as u64), &"zero or one")),
        }
    }
}