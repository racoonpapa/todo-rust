extern crate uuid;

use std::fmt::{Display, Formatter};
use chrono::{NaiveDate};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Priority {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub content: String,
    pub priority: Priority,
    pub due_date: Option<NaiveDate>,
    pub done: bool
}

impl Display for Priority {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let display_str = match self {
            Priority::None => "None",
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
        };
        write!(f, "{}", display_str)
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}{}{}",
               if self.done { 'x' } else { ' ' },
               self.content,
               match self.priority {
                   Priority::None => String::new(),
                   _ => format!(" [{}]", self.priority),
               },
               self.due_date.map_or(String::new(), |dt| format!(" ({})", dt.format("%Y-%m-%d")))
        )
    }
}

impl Item {
    pub fn new(content: &str, priority: Priority, date_str: Option<&str>) -> Result<Item, Box<dyn std::error::Error>> {
        let due_date = match date_str {
            Some(date_str) => {
                Some(NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?)
            },
            None => None,
        };

        Ok(Item {
            id: Uuid::new_v4().to_string(),
            content: content.to_string(),
            priority,
            due_date,
            done: false
        })
    }
}
