use std::fs;

use colored::Colorize;

use crate::executor::execute;
use crate::parser::parse;

/// Execute every line of a `.nsh` script file.
/// Lines starting with `#` are treated as comments and skipped.
/// Empty lines are also skipped.
pub fn run_script(path: &str) {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", format!("Kann Skript '{}' nicht laden: {}", path, e).red());
            return;
        }
    };

    println!("{}", format!("▶  Führe '{}' aus / Running '{}'", path, path).cyan());

    for (line_no, raw_line) in content.lines().enumerate() {
        let line = raw_line.trim();

        // Skip comments and blank lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        println!("{} {}", format!("[{}]", line_no + 1).dimmed(), line.dimmed());

        let intent = parse(line);
        let keep_running = execute(intent);
        if !keep_running {
            // Script contained an `exit` — stop immediately
            println!("{}", "Skript hat 'exit' aufgerufen. / Script called 'exit'.".yellow());
            std::process::exit(0);
        }
    }

    println!("{}", format!("✓  '{}' abgeschlossen. / finished.", path).green());
}
