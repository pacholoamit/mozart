use prost_types::{value, Value};
use serde_json::{Map, Number, Value as JsonValue};

pub fn prost_to_serde(value: &Value) -> JsonValue {
    match value.kind {
        Some(value::Kind::NullValue(_)) => JsonValue::Null,
        Some(value::Kind::NumberValue(n)) => JsonValue::Number(Number::from_f64(n).unwrap()),
        Some(value::Kind::StringValue(ref s)) => JsonValue::String(s.to_string()),
        Some(value::Kind::BoolValue(b)) => JsonValue::Bool(b),
        Some(value::Kind::StructValue(ref s)) => {
            let mut map = Map::new();
            for (k, v) in s.fields.iter() {
                map.insert(k.to_string(), prost_to_serde(v));
            }
            JsonValue::Object(map)
        }
        Some(value::Kind::ListValue(ref l)) => {
            let mut vec = Vec::new();
            for v in l.values.iter() {
                vec.push(prost_to_serde(v));
            }
            JsonValue::Array(vec)
        }
        None => JsonValue::Null,
    }
}

pub fn serde_to_prost(value: &JsonValue) -> Value {
    match value {
        JsonValue::Null => Value::default(),
        JsonValue::Number(n) => Value {
            kind: Some(value::Kind::NumberValue(n.as_f64().unwrap())),
        },
        JsonValue::String(s) => Value {
            kind: Some(value::Kind::StringValue(s.to_string())),
        },
        JsonValue::Bool(b) => Value {
            kind: Some(value::Kind::BoolValue(*b)),
        },
        JsonValue::Object(o) => {
            let mut prost_struct = prost_types::Struct {
                fields: o
                    .iter()
                    .map(|(k, v)| (k.to_string(), serde_to_prost(v)))
                    .collect(),
            };

            Value {
                kind: Some(value::Kind::StructValue(prost_struct)),
            }
        }
        JsonValue::Array(a) => {
            let mut vec = Vec::new();
            for v in a.iter() {
                vec.push(serde_to_prost(v));
            }
            Value {
                kind: Some(value::Kind::ListValue(prost_types::ListValue {
                    values: vec,
                })),
            }
        }
    }
}
