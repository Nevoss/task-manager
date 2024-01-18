use std::error::Error;

use crate::db::{self, Task, TaskStatus};

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut db = db::read()?;
    let id = db.meta.last_id + 1;

    let new_task = Task {
        id,
        title: "This is new title".to_string(),
        description: "This is new description".to_string(),
        status: TaskStatus::Idle,
    };

    db.meta.last_id = id;
    db.data.push(new_task);

    db::write(db)
}
