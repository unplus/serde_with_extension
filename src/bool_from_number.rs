use serde::{de, Deserialize, Deserializer, Serializer};
use serde_with::{DeserializeAs, SerializeAs};

pub struct BoolFromNumber;

impl SerializeAs<bool> for BoolFromNumber {
    fn serialize_as<S>(source: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if *source {
            serializer.serialize_u8(1)
        } else {
            serializer.serialize_u8(0)
        }
    }
}

impl<'de> DeserializeAs<'de, bool> for BoolFromNumber {
    fn deserialize_as<D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        match u8::deserialize(deserializer) {
            Ok(v) if v == 1 => Ok(true),
            Ok(v) if v == 0 => Ok(false),
            Ok(v) => Err(de::Error::invalid_value(
                de::Unexpected::Unsigned(v.into()),
                &"0 or 1",
            )),
            Err(e) => Err(e),
        }
    }
}
