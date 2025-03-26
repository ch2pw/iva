#![allow(dead_code)]
use std::{collections::HashMap, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub type UniqueProps = HashMap<String, serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonItemProps {
    pub time: TimeRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemProps {
    #[serde(flatten)]
    pub common: CommonItemProps,
    #[serde(flatten)]
    pub others: UniqueProps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedItemProps<T: From<UniqueProps>> {
    #[serde(flatten)]
    pub common: CommonItemProps,
    #[serde(flatten)]
    pub others: T,
}

impl<T: From<UniqueProps>> From<ItemProps> for ParsedItemProps<T> {
    fn from(props: ItemProps) -> Self {
        ParsedItemProps {
            common: props.common,
            others: props.others.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonFilterProps {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterProps {
    #[serde(flatten)]
    pub common: CommonFilterProps,
    #[serde(flatten)]
    pub others: UniqueProps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedFilterProps<T: From<UniqueProps>> {
    #[serde(flatten)]
    pub common: CommonFilterProps,
    #[serde(flatten)]
    pub others: T,
}

impl<T: From<UniqueProps>> From<FilterProps> for ParsedFilterProps<T> {
    fn from(props: FilterProps) -> Self {
        ParsedFilterProps {
            common: props.common,
            others: props.others.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub kind: String,
    pub props: FilterProps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub layer: i64,
    pub kind: String,
    pub name: String,
    pub filters: Vec<Filter>,
    pub props: ItemProps,
}
