mod completer;
mod executor;
mod parser;
mod scripts;

use std::env;

use colored::Colorize;
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use rustyline::Editor;

use completer::NshHelper;

fn main() {
    // If a script path is passed as argument, run it and exit
    let args: Vec<String> = env::args().skip(1).collect();
    if !args.is_empty() {
        for script in &args {
            scripts::run_script(script);
        }
        return;
    }

    print_banner();

    // Build readline editor with history + completion
    let mut rl: Editor<NshHelper, DefaultHistory> =
        Editor::new().expect("Konnte Editor nicht erstellen");
    rl.set_helper(Some(NshHelper::new()));
    rl.set_auto_add_history(true);
    rl.set_completion_type(rustyline::CompletionType::List);

    // Load persistent history
    let history_path = history_file();
    if let Some(ref p) = history_path {
        let _ = rl.load_history(p);
    }

    loop {
        let prompt = build_prompt();

        match rl.readline(&prompt) {
            Ok(line) => {
                let input = line.trim().to_string();
                if input.is_empty() {
                    continue;
                }

                let intent = parser::parse(&input);
                let keep_running = executor::execute(intent);
                if !keep_running {
                    break;
                }
            }

            Err(ReadlineError::Interrupted) => {
                // Ctrl-C: clear the current line, don't exit
                println!("(Ctrl-C — tippe 'exit' zum Beenden)");
            }

            Err(ReadlineError::Eof) => {
                // Ctrl-D: exit gracefully
                println!("{}", "Auf Wiedersehen! / Goodbye!".cyan());
                break;
            }

            Err(e) => {
                eprintln!("{}", format!("Fehler: {}", e).red());
                break;
            }
        }
    }

    if let Some(ref p) = history_path {
        let _ = rl.save_history(p);
    }
}

// ── prompt ───────────────────────────────────────────────────────────────────

fn build_prompt() -> String {
    let cwd = env::current_dir()
        .map(|p| {
            // Shorten home dir to ~
            if let Some(home) = dirs::home_dir() {
                let s = p.to_string_lossy().into_owned();
                let h = home.to_string_lossy().into_owned();
                if s.starts_with(&h) {
                    return s.replacen(&h, "~", 1);
                }
                s
            } else {
                p.to_string_lossy().into_owned()
            }
        })
        .unwrap_or_else(|_| "?".to_string());

    format!("{} {} ", cwd.cyan(), "›".green().bold())
}

// ── startup banner ────────────────────────────────────────────────────────────

fn print_banner() {
    println!();
    println!("{}", "  ███╗   ██╗███████╗██╗  ██╗".cyan());
    println!("{}", "  ████╗  ██║██╔════╝██║  ██║".cyan());
    println!("{}", "  ██╔██╗ ██║███████╗███████║".cyan());
    println!("{}", "  ██║╚██╗██║╚════██║██╔══██║".cyan());
    println!("{}", "  ██║ ╚████║███████║██║  ██║".cyan());
    println!("{}", "  ╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝".cyan());
    println!();
    println!(
        "  {}  {}",
        "Simple Shell".white().bold(),
        "— sprich einfach / just speak naturally".dimmed()
    );
    println!(
        "  {}",
        "Tippe 'hilfe' oder 'help' für Befehle  •  Tab für Vorschläge".dimmed()
    );
    println!();
}

// ── history file path ─────────────────────────────────────────────────────────

fn history_file() -> Option<std::path::PathBuf> {
    dirs::home_dir().map(|mut p| {
        p.push(".nsh_history");
        p
    })
}
