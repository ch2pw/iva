#![allow(dead_code)]
use std::{collections::HashMap, time::Duration};

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
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

    pub fn contains(&self, time: u64) -> bool {
        self.start <= time && time <= self.end
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub kind: String,
    pub name: String,
    pub props: HashMap<String, serde_json::Value>,
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub layer: i64,
    pub kind: String,
    pub name: String,
    pub time: TimeRange,
    pub filters: Vec<Filter>,
    pub props: HashMap<String, serde_json::Value>,
}
