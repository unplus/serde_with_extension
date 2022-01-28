use rust_decimal::{serde::float, Decimal};
use serde::{Deserializer, Serializer};
use serde_with::{DeserializeAs, SerializeAs};

pub struct DecimalFromNumber;

impl SerializeAs<Decimal> for DecimalFromNumber {
    fn serialize_as<S>(source: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        float::serialize(source, serializer)
    }
}

impl<'de> DeserializeAs<'de, Decimal> for DecimalFromNumber {
    fn deserialize_as<D>(deserializer: D) -> Result<Decimal, D::Error>
    where
        D: Deserializer<'de>,
    {
        float::deserialize(deserializer)
    }
}
