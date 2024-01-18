use core::fmt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

const DB_PATH: &str = ".db.json";

#[derive(Deserialize, Serialize, Debug)]
pub enum TaskStatus {
    Idle,
    InProgress,
    Done,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Meta {
    pub last_id: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DB {
    pub data: Vec<Task>,
    pub meta: Meta,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            TaskStatus::Idle => "idle",
            TaskStatus::InProgress => "in-progress",
            TaskStatus::Done => "done",
        };

        write!(f, "{}", text)
    }
}

pub fn read() -> Result<DB, Box<dyn Error>> {
    let file = fs::read_to_string(DB_PATH)?;

    let db: DB = serde_json::from_str(&file)?;

    Ok(db)
}

pub fn write(data: DB) -> Result<(), Box<dyn Error>> {
    let contents = serde_json::to_string(&data)?;

    fs::write(DB_PATH, contents)?;

    Ok(())
}
