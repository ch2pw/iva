use std::{collections::HashMap, time::Duration};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeRange {
    pub start: u64,
    pub end: u64,
}

impl TimeRange {
    pub fn new(start: u64, end: u64) -> Self {
        TimeRange { start, end }
    }

    pub fn duration(&self) -> Duration {
        Duration::from_millis(self.end - self.start)
    }
}

#[derive(Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Number(value as f64)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            Value::Number(value) => serializer.serialize_f64(*value),
            Value::String(value) => serializer.serialize_str(value),
            Value::Bool(value) => serializer.serialize_bool(*value),
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::Number(value) => Ok(Value::Number(value.as_f64().unwrap())),
            serde_json::Value::String(value) => Ok(Value::String(value)),
            serde_json::Value::Bool(value) => Ok(Value::Bool(value)),
            _ => panic!("unexpected value"),
        }
    }
}

impl Value {
    pub fn as_int(&self) -> i64 {
        match self {
            Value::Number(value) => *value as i64,
            _ => panic!("expected int"),
        }
    }

    pub fn as_float(&self) -> f64 {
        match self {
            Value::Number(value) => *value,
            _ => panic!("expected float"),
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Value::String(value) => value.to_string(),
            _ => panic!("expected string"),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(value) => *value,
            _ => panic!("expected bool"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Filter {
    pub kind: String,
    pub name: String,
    pub props: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: Uuid,
    pub kind: String,
    pub name: String,
    pub time: TimeRange,
    pub filters: Vec<Filter>,
    pub props: HashMap<String, Value>,
}
