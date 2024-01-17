use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

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
pub struct DB {
    pub data: Vec<Task>,
}

pub fn read() -> Result<DB, Box<dyn Error>> {
    let file = File::open(".db.json")?;
    let reader = BufReader::new(file);

    let db: DB = serde_json::from_reader(reader)?;

    Ok(db)
}
