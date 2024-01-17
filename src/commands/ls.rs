use crate::db;
use crate::utils::TableFormat;
use comfy_table::Table;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let db = db::read()?;

    let mut table = Table::new();

    table.set_header(vec!["ID", "Title", "Description", "Status"]);

    db.data.iter().for_each(|task| {
        table.add_row(vec![
            task.id.to_string(),
            task.title.break_lines_by_word(4),
            task.description.break_lines_by_word(4),
            task.status.to_string(),
        ]);

        ()
    });

    println!("{}", table);

    Ok(())
}
