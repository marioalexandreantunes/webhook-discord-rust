use chrono::prelude::*;
use colored::Colorize;

struct Log {
    info: String,
    date: String,
    text: String,
}

pub fn add(log: String, debug: bool) {
    if debug {
        let final_text = Log {
            info: "DEBUG".to_string(),
            date: Utc::now().format("%d-%m-%Y %H:%M:%S").to_string(),
            text: log,
        };

        let result = format!(
            "[{}] {} >> {}",
            final_text.info.red(),
            final_text.date.bold(),
            final_text.text.purple().italic()
        );
        println!("{} ", result);
    };
}
