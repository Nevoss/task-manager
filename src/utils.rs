pub trait TableFormat {
    fn break_lines_by_word(&self, word_count: usize) -> String;
}

impl TableFormat for String {
    fn break_lines_by_word(&self, break_at: usize) -> String {
        let mut result = String::new();

        for (index, word) in self.split(" ").enumerate() {
            if index % break_at == 0 {
                result.push_str("\n");
            }

            let word = format!("{} ", &word);

            result.push_str(&word);
        }

        result.trim().to_string()
    }
}
