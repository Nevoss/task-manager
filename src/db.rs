use serde::{Deserialize, Serialize};
use std::error::Error;

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
    let json = r#"
        {
            "data": [
                {
                    "id": 1,
                    "title": "Create V2 transformer package.",
                    "description": "This is some description for the transformer package.",
                    "status": "Idle"
                },
                {
                    "id": 2,
                    "title": "Create V2 preview package.",
                    "description": "This is some description for the preview package.",
                    "status": "InProgress"
                }
            ]
        }
    "#;

    let db: DB = serde_json::from_str(json)?;

    Ok(db)
}
