use comfy_table::Table;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize, Serialize, Debug)]
enum TaskStatus {
    Idle,
    InProgress,
    Done,
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

#[derive(Deserialize, Serialize, Debug)]
struct Task {
    id: u32,
    title: String,
    description: String,
    status: TaskStatus,
}

#[derive(Deserialize, Serialize, Debug)]
struct DB {
    data: Vec<Task>,
}

pub fn ls() -> Result<(), Box<dyn Error>> {
    let db = read_db()?;

    let mut table = Table::new();

    table.set_header(vec!["ID", "Title", "Description", "Status"]);

    db.data.iter().for_each(|task| {
        table.add_row(vec![
            task.id.to_string(),
            format_for_table_view(&task.title, 4),
            format_for_table_view(&task.description, 4),
            task.status.to_string(),
        ]);

        ()
    });

    println!("{}", table);

    Ok(())
}

fn read_db() -> Result<DB, Box<dyn Error>> {
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

fn format_for_table_view(subject: &str, split_at: usize) -> String {
    let subject = subject.to_string();
    let mut result = String::new();

    for (index, word) in subject.split(" ").enumerate() {
        if index % split_at == 0 {
            result.push_str("\n");
        }

        let word = format!("{} ", &word);

        result.push_str(&word);
    }

    result.trim().to_string()
}
