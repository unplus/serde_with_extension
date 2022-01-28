use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with_extension::BoolFromNumber;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct S1 {
    #[serde_as(as = "BoolFromNumber")]
    value: bool,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct S2 {
    #[serde_as(as = "Option<BoolFromNumber>")]
    value: Option<bool>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct S3 {
    #[serde_as(as = "BoolFromNumber")]
    #[serde(default)]
    value: bool,
}

#[test]
fn test_serialize() {
    let s = S1 { value: true };
    let actual = serde_json::to_string(&s).unwrap();
    assert_eq!(actual, r#"{"value":1}"#.to_string());

    let s = S1 { value: false };
    let actual = serde_json::to_string(&s).unwrap();
    assert_eq!(actual, r#"{"value":0}"#.to_string());

    let s = S2 { value: Some(true) };
    let actual = serde_json::to_string(&s).unwrap();
    assert_eq!(actual, r#"{"value":1}"#.to_string());

    let s = S2 { value: None };
    let actual = serde_json::to_string(&s).unwrap();
    assert_eq!(actual, r#"{"value":null}"#.to_string());

    let s = S3 { value: true };
    let actual = serde_json::to_string(&s).unwrap();
    assert_eq!(actual, r#"{"value":1}"#.to_string());
}

#[test]
fn test_deserialize() {
    let value = serde_json::json!({"value": 1});
    let actual = serde_json::from_value::<S1>(value).unwrap();
    assert_eq!(actual, S1 { value: true });

    let value = serde_json::json!({"value": 0});
    let actual = serde_json::from_value::<S1>(value).unwrap();
    assert_eq!(actual, S1 { value: false });

    let value = serde_json::json!({"value": 2});
    let res = serde_json::from_value::<S1>(value);
    assert!(res.is_err());

    let value = serde_json::json!({ "value": null });
    let actual = serde_json::from_value::<S2>(value).unwrap();
    assert_eq!(actual, S2 { value: None });

    let value = serde_json::json!({ "value": 1 });
    let actual = serde_json::from_value::<S2>(value).unwrap();
    assert_eq!(actual, S2 { value: Some(true) });

    let value = serde_json::json!({});
    let actual = serde_json::from_value::<S3>(value).unwrap();
    assert_eq!(actual, S3 { value: false });
}
