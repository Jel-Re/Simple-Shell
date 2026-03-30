use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process::Command;

use colored::Colorize;

use crate::parser::Intent;

/// Execute a parsed intent. Returns `true` if the shell should keep running.
pub fn execute(intent: Intent) -> bool {
    match intent {
        Intent::Exit => {
            println!("{}", "Auf Wiedersehen! / Goodbye!".cyan());
            return false;
        }

        Intent::Help => {
            print_help();
        }

        Intent::Clear => {
            // ANSI clear
            print!("\x1B[2J\x1B[1;1H");
            let _ = io::stdout().flush();
        }

        Intent::WhereAmI => match env::current_dir() {
            Ok(path) => println!("{}", path.display().to_string().yellow()),
            Err(e) => eprintln!("{}", format!("Fehler / Error: {}", e).red()),
        },

        Intent::ListFiles { hidden } => {
            let path = env::current_dir().unwrap_or_else(|_| ".".into());
            match list_dir(&path, hidden) {
                Ok(()) => {}
                Err(e) => eprintln!("{}", format!("Fehler / Error: {}", e).red()),
            }
        }

        Intent::ChangeDir { path } => {
            let expanded = expand_home(&path);
            let target = Path::new(&expanded);
            if let Err(e) = env::set_current_dir(target) {
                eprintln!(
                    "{}",
                    format!("Kann nicht wechseln nach '{}': {}", path, e).red()
                );
            }
        }

        Intent::MakeDir { name } => {
            if let Err(e) = fs::create_dir_all(&name) {
                eprintln!("{}", format!("Fehler beim Erstellen von '{}': {}", name, e).red());
            } else {
                println!("{}", format!("Ordner '{}' erstellt. / Folder '{}' created.", name, name).green());
            }
        }

        Intent::Remove { path } => {
            let p = Path::new(&path);
            let result = if p.is_dir() {
                fs::remove_dir_all(p)
            } else {
                fs::remove_file(p)
            };
            match result {
                Ok(()) => println!("{}", format!("'{}' gelöscht. / '{}' deleted.", path, path).green()),
                Err(e) => eprintln!("{}", format!("Fehler beim Löschen von '{}': {}", path, e).red()),
            }
        }

        Intent::Copy { from, to } => {
            match fs::copy(&from, &to) {
                Ok(_) => println!(
                    "{}",
                    format!("'{}' → '{}' kopiert. / copied.", from, to).green()
                ),
                Err(e) => eprintln!("{}", format!("Fehler beim Kopieren: {}", e).red()),
            }
        }

        Intent::Move { from, to } => {
            if let Err(e) = fs::rename(&from, &to) {
                eprintln!("{}", format!("Fehler beim Verschieben: {}", e).red());
            } else {
                println!("{}", format!("'{}' → '{}' verschoben. / moved.", from, to).green());
            }
        }

        Intent::ShowFile { path } => match fs::read_to_string(&path) {
            Ok(contents) => print!("{}", contents),
            Err(e) => eprintln!("{}", format!("Kann '{}' nicht lesen: {}", path, e).red()),
        },

        Intent::Search { pattern, path } => {
            match fs::File::open(&path) {
                Ok(file) => {
                    let mut found = false;
                    for (i, line) in io::BufReader::new(file).lines().enumerate() {
                        if let Ok(line) = line {
                            if line.contains(&pattern) {
                                println!("{}: {}", format!("{}", i + 1).dimmed(), line);
                                found = true;
                            }
                        }
                    }
                    if !found {
                        println!("{}", format!("Nichts gefunden. / Nothing found.").dimmed());
                    }
                }
                Err(e) => eprintln!("{}", format!("Kann '{}' nicht öffnen: {}", path, e).red()),
            }
        }

        Intent::RunScript { path } => {
            crate::scripts::run_script(&path);
        }

        Intent::Raw { command } => {
            if command.is_empty() {
                return true;
            }
            run_raw(&command);
        }
    }
    true
}

// ── directory listing ────────────────────────────────────────────────────────

fn list_dir(path: &Path, show_hidden: bool) -> io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            if show_hidden {
                true
            } else {
                !e.file_name().to_string_lossy().starts_with('.')
            }
        })
        .collect();

    entries.sort_by_key(|e| e.file_name());

    let mut dirs = Vec::new();
    let mut files = Vec::new();

    for entry in &entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        let meta = entry.metadata();
        if meta.map(|m| m.is_dir()).unwrap_or(false) {
            dirs.push(name);
        } else {
            files.push(name);
        }
    }

    for d in &dirs {
        println!("  {}/", d.blue().bold());
    }
    for f in &files {
        println!("  {}", f);
    }

    if dirs.is_empty() && files.is_empty() {
        println!("{}", "(leer / empty)".dimmed());
    }

    Ok(())
}

// ── raw command passthrough ──────────────────────────────────────────────────

fn run_raw(command: &str) {
    let mut parts = command.split_whitespace();
    let prog = match parts.next() {
        Some(p) => p,
        None => return,
    };
    let args: Vec<&str> = parts.collect();

    match Command::new(prog).args(&args).status() {
        Ok(status) => {
            if !status.success() {
                if let Some(code) = status.code() {
                    eprintln!("{}", format!("[exit {}]", code).dimmed());
                }
            }
        }
        Err(e) => eprintln!(
            "{}",
            format!(
                "Befehl '{}' nicht gefunden. / Command '{}' not found. ({})",
                prog, prog, e
            )
            .red()
        ),
    }
}

// ── home dir expansion ───────────────────────────────────────────────────────

fn expand_home(path: &str) -> String {
    if path == "~" || path.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            return path.replacen('~', &home.to_string_lossy(), 1);
        }
    }
    path.to_string()
}

// ── help ─────────────────────────────────────────────────────────────────────

fn print_help() {
    let sections: &[(&str, &[(&str, &str)])] = &[
        (
            "Navigation",
            &[
                ("zeige dateien  / list files",      "Dateien auflisten"),
                ("zeige alle dateien / show all files", "Auch versteckte Dateien"),
                ("gehe zu <pfad> / go to <path>",    "Verzeichnis wechseln"),
                ("wo bin ich?    / where am i?",     "Aktuelles Verzeichnis"),
            ],
        ),
        (
            "Dateien / Files",
            &[
                ("erstelle ordner <n> / create folder <n>", "Ordner erstellen"),
                ("lösche <pfad>       / delete <path>",     "Datei oder Ordner löschen"),
                ("kopiere <a> nach <b> / copy <a> to <b>",  "Datei kopieren"),
                ("verschiebe <a> nach <b> / move <a> to <b>", "Datei verschieben"),
                ("lese <datei>        / read <file>",       "Dateiinhalt anzeigen"),
                ("suche <muster> in <datei> / find <p> in <f>", "In Datei suchen"),
            ],
        ),
        (
            "Skripte / Scripts",
            &[
                ("run <datei.nsh>   / führe aus <datei.nsh>", "NSH-Skript ausführen"),
                ("<datei.nsh>",                               "Direkt ausführen (kürzer)"),
            ],
        ),
        (
            "Sonstiges / Misc",
            &[
                ("help / hilfe / ?", "Diese Hilfe anzeigen"),
                ("clear / leere bildschirm", "Terminal leeren"),
                ("exit / verlasse / quit",   "Shell beenden"),
                ("<beliebiger Befehl>",       "Direkter Systembefehl (Fallback)"),
            ],
        ),
    ];

    println!();
    println!("{}", "╔══════════════════════════════════════════════╗".cyan());
    println!("{}", "║          Simple Shell  –  nsh  Hilfe         ║".cyan());
    println!("{}", "╚══════════════════════════════════════════════╝".cyan());

    for (section, commands) in sections {
        println!();
        println!("  {}", section.yellow().bold());
        for (cmd, desc) in *commands {
            println!("    {:<45}  {}", cmd.green(), desc.dimmed());
        }
    }
    println!();
    println!("{}", "  Tipp: Skripte enden auf .nsh und enthalten je einen Befehl pro Zeile.".dimmed());
    println!();
}
