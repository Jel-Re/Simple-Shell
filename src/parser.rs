/// Represents a parsed user intent with extracted arguments
#[derive(Debug, Clone, PartialEq)]
pub enum Intent {
    ListFiles { hidden: bool },
    ChangeDir { path: String },
    MakeDir { name: String },
    Remove { path: String },
    Copy { from: String, to: String },
    Move { from: String, to: String },
    ShowFile { path: String },
    Search { pattern: String, path: String },
    WhereAmI,
    Clear,
    Exit,
    Help,
    RunScript { path: String },
    /// Falls through to the OS as-is
    Raw { command: String },
}

/// Parse a natural-language (German or English) line into an Intent.
pub fn parse(input: &str) -> Intent {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Intent::Raw { command: String::new() };
    }

    let lower = trimmed.to_lowercase();
    let tokens: Vec<&str> = trimmed.split_whitespace().collect();

    // EXIT
    if matches_any(
        &lower,
        &[
            "exit", "quit", "bye", "verlasse", "beende", "tschüss", "tschuess",
            "auf wiedersehen", "ciao", "raus",
        ],
    ) {
        return Intent::Exit;
    }

    // HELP
    if matches_any(
        &lower,
        &["help", "hilfe", "?", "befehle", "commands", "was kann ich", "was kannst du"],
    ) {
        return Intent::Help;
    }

    // CLEAR
    if matches_any(&lower, &["clear", "cls", "leere bildschirm", "bildschirm leeren", "leeren"]) {
        return Intent::Clear;
    }

    // WHERE AM I
    if matches_any(
        &lower,
        &[
            "pwd",
            "wo bin ich",
            "wo bin ich?",
            "where am i",
            "where am i?",
            "aktueller pfad",
            "current path",
            "aktuelles verzeichnis",
            "current directory",
            "current dir",
        ],
    ) {
        return Intent::WhereAmI;
    }

    // LIST FILES — check for "versteckt"/"hidden"/"all"/"-a" variants
    let list_hidden = lower.contains("versteckt")
        || lower.contains("hidden")
        || lower.contains(" -a")
        || lower.contains(" alle ")
        || lower.ends_with(" alle")
        || lower.contains(" all ");

    if matches_any(
        &lower,
        &[
            "ls",
            "ls -la",
            "ls -a",
            "ls -l",
            "dir",
            "zeige dateien",
            "liste dateien",
            "zeige alle dateien",
            "zeige versteckte dateien",
            "list files",
            "show files",
            "show all files",
            "list all files",
            "dateien anzeigen",
            "dateien zeigen",
            "inhalt anzeigen",
            "ordnerinhalt",
            "verzeichnisinhalt",
            "was ist hier",
            "was ist da",
            "what's here",
            "what is here",
        ],
    ) || starts_with_any(&lower, &["ls ", "ls\t", "zeige dateien", "liste dateien", "list files"])
    {
        return Intent::ListFiles { hidden: list_hidden };
    }

    // CHANGE DIR
    if let Some(path) = extract_arg(
        &lower,
        trimmed,
        &[
            "cd ",
            "gehe zu ",
            "gehe nach ",
            "wechsle zu ",
            "wechsle nach ",
            "wechsel zu ",
            "wechsel nach ",
            "go to ",
            "change to ",
            "change dir to ",
            "navigate to ",
            "navigiere zu ",
        ],
    ) {
        return Intent::ChangeDir { path };
    }

    // MAKE DIR
    if let Some(name) = extract_arg(
        &lower,
        trimmed,
        &[
            "mkdir ",
            "erstelle ordner ",
            "erstelle verzeichnis ",
            "neuer ordner ",
            "neues verzeichnis ",
            "create folder ",
            "create directory ",
            "create dir ",
            "make dir ",
            "make directory ",
            "new folder ",
            "new directory ",
        ],
    ) {
        return Intent::MakeDir { name };
    }

    // REMOVE
    if let Some(path) = extract_arg(
        &lower,
        trimmed,
        &[
            "rm ",
            "lösche ",
            "loesche ",
            "entferne ",
            "delete ",
            "remove ",
            "datei löschen ",
            "datei loeschen ",
        ],
    ) {
        return Intent::Remove { path };
    }

    // COPY — needs two args: from & to
    if let Some((from, to)) = extract_two_args(
        &lower,
        trimmed,
        &["cp ", "kopiere ", "copy "],
        &[" nach ", " to ", " in ", " into "],
    ) {
        return Intent::Copy { from, to };
    }

    // MOVE / RENAME
    if let Some((from, to)) = extract_two_args(
        &lower,
        trimmed,
        &["mv ", "verschiebe ", "benenne um ", "move ", "rename "],
        &[" nach ", " to ", " in ", " into ", " zu "],
    ) {
        return Intent::Move { from, to };
    }

    // SHOW FILE (cat / read / print)
    if let Some(path) = extract_arg(
        &lower,
        trimmed,
        &[
            "cat ",
            "lese ",
            "lies ",
            "zeige inhalt ",
            "zeige datei ",
            "drucke ",
            "read ",
            "show ",
            "print ",
            "open ",
            "öffne ",
            "oeffne ",
            "ausgabe von ",
        ],
    ) {
        return Intent::ShowFile { path };
    }

    // SEARCH — needs pattern + file
    if let Some((pattern, path)) = extract_two_args(
        &lower,
        trimmed,
        &["grep ", "suche ", "search ", "finde ", "find "],
        &[" in ", " in der datei ", " within "],
    ) {
        return Intent::Search { pattern, path };
    }

    // RUN SCRIPT
    if let Some(path) = extract_arg(
        &lower,
        trimmed,
        &[
            "run ",
            "execute ",
            "führe aus ",
            "fuehre aus ",
            "starte ",
            "start ",
            "ausführen ",
        ],
    ) {
        return Intent::RunScript { path };
    }
    // bare .nsh file
    if lower.ends_with(".nsh") && tokens.len() == 1 {
        return Intent::RunScript { path: tokens[0].to_string() };
    }

    // Fall through to raw OS command
    Intent::Raw { command: trimmed.to_string() }
}

// ── helpers ─────────────────────────────────────────────────────────────────

fn matches_any(lower: &str, patterns: &[&str]) -> bool {
    patterns.iter().any(|p| lower == *p)
}

fn starts_with_any(lower: &str, prefixes: &[&str]) -> bool {
    prefixes.iter().any(|p| lower.starts_with(p))
}

/// Given a prefix list (all lowercase), strip the first matching prefix from
/// `lower` and return the **original-case** tail from `original`.
fn extract_arg(lower: &str, original: &str, prefixes: &[&str]) -> Option<String> {
    for prefix in prefixes {
        if lower.starts_with(prefix) {
            let tail = original[prefix.len()..].trim().to_string();
            if !tail.is_empty() {
                return Some(tail);
            }
        }
    }
    None
}

/// Extract two arguments separated by a mid-separator, e.g.
/// "kopiere foo nach bar" → ("foo", "bar")
fn extract_two_args(
    lower: &str,
    original: &str,
    prefixes: &[&str],
    separators: &[&str],
) -> Option<(String, String)> {
    for prefix in prefixes {
        if lower.starts_with(prefix) {
            let rest_lower = &lower[prefix.len()..];
            let rest_original = &original[prefix.len()..];
            for sep in separators {
                if let Some(pos) = rest_lower.find(sep) {
                    let from = rest_original[..pos].trim().to_string();
                    let to = rest_original[pos + sep.len()..].trim().to_string();
                    if !from.is_empty() && !to.is_empty() {
                        return Some((from, to));
                    }
                }
            }
        }
    }
    None
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exit() {
        assert_eq!(parse("exit"), Intent::Exit);
        assert_eq!(parse("verlasse"), Intent::Exit);
        assert_eq!(parse("quit"), Intent::Exit);
    }

    #[test]
    fn test_list_files() {
        assert!(matches!(parse("ls"), Intent::ListFiles { hidden: false }));
        assert!(matches!(parse("zeige dateien"), Intent::ListFiles { .. }));
        assert!(matches!(parse("list files"), Intent::ListFiles { .. }));
    }

    #[test]
    fn test_list_files_hidden() {
        assert!(matches!(parse("ls -a"), Intent::ListFiles { hidden: true }));
        assert!(matches!(parse("zeige versteckte dateien"), Intent::ListFiles { hidden: true }));
    }

    #[test]
    fn test_change_dir() {
        assert_eq!(parse("cd /home"), Intent::ChangeDir { path: "/home".into() });
        assert_eq!(parse("gehe zu /tmp"), Intent::ChangeDir { path: "/tmp".into() });
        assert_eq!(parse("go to /usr/bin"), Intent::ChangeDir { path: "/usr/bin".into() });
    }

    #[test]
    fn test_make_dir() {
        assert_eq!(parse("mkdir myfolder"), Intent::MakeDir { name: "myfolder".into() });
        assert_eq!(
            parse("erstelle ordner myfolder"),
            Intent::MakeDir { name: "myfolder".into() }
        );
    }

    #[test]
    fn test_copy() {
        assert_eq!(
            parse("kopiere foo.txt nach bar.txt"),
            Intent::Copy { from: "foo.txt".into(), to: "bar.txt".into() }
        );
        assert_eq!(
            parse("copy foo.txt to bar.txt"),
            Intent::Copy { from: "foo.txt".into(), to: "bar.txt".into() }
        );
    }

    #[test]
    fn test_show_file() {
        assert_eq!(parse("cat README.md"), Intent::ShowFile { path: "README.md".into() });
        assert_eq!(parse("lese README.md"), Intent::ShowFile { path: "README.md".into() });
        assert_eq!(parse("read README.md"), Intent::ShowFile { path: "README.md".into() });
    }

    #[test]
    fn test_raw_fallthrough() {
        assert_eq!(
            parse("git status"),
            Intent::Raw { command: "git status".into() }
        );
    }

    #[test]
    fn test_run_script() {
        assert_eq!(
            parse("deploy.nsh"),
            Intent::RunScript { path: "deploy.nsh".into() }
        );
        assert_eq!(
            parse("run myscript.nsh"),
            Intent::RunScript { path: "myscript.nsh".into() }
        );
    }
}
