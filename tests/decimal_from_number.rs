use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with_extension::DecimalFromNumber;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct S1 {
    #[serde_as(as = "DecimalFromNumber")]
    value: Decimal,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct S2 {
    #[serde_as(as = "Option<DecimalFromNumber>")]
    value: Option<Decimal>,
}

#[test]
fn test_serialize() {
    let s = S1 {
        value: Decimal::new(101, 1),
    };
    let actual = serde_json::to_string(&s).unwrap();
    assert_eq!(actual, r#"{"value":10.1}"#.to_string());

    let s = S2 {
        value: Some(Decimal::new(101, 1)),
    };
    let actual = serde_json::to_string(&s).unwrap();
    assert_eq!(actual, r#"{"value":10.1}"#.to_string());

    let s = S2 { value: None };
    let actual = serde_json::to_string(&s).unwrap();
    assert_eq!(actual, r#"{"value":null}"#.to_string());
}

#[test]
fn test_deserialize() {
    let value = serde_json::json!({"value": "10.1"});
    let actual = serde_json::from_value::<S1>(value).unwrap();
    assert_eq!(
        actual,
        S1 {
            value: Decimal::new(101, 1)
        }
    );

    let value = serde_json::json!({"value": "10.1a"});
    let res = serde_json::from_value::<S1>(value);
    assert!(res.is_err());

    let value = serde_json::json!({"value": 10.1});
    let actual = serde_json::from_value::<S2>(value).unwrap();
    assert_eq!(
        actual,
        S2 {
            value: Some(Decimal::new(101, 1))
        }
    );

    let value = serde_json::json!({ "value": null });
    let actual = serde_json::from_value::<S2>(value).unwrap();
    assert_eq!(actual, S2 { value: None });
}
