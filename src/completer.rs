use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::Context;
use rustyline_derive::{Helper, Highlighter, Hinter, Validator};

/// All natural-language command prefixes the shell recognises.
/// Sorted so longer / more-specific phrases come before shorter ones
/// (avoids "ls" swallowing "ls -la" suggestions, etc.).
static NL_COMMANDS: &[&str] = &[
    // German – multi-word first
    "zeige alle dateien",
    "zeige versteckte dateien",
    "zeige inhalt ",
    "zeige datei ",
    "zeige dateien",
    "liste dateien",
    "gehe zu ",
    "gehe nach ",
    "wechsle zu ",
    "wechsle nach ",
    "erstelle ordner ",
    "erstelle verzeichnis ",
    "lösche ",
    "loesche ",
    "kopiere ",
    "verschiebe ",
    "suche ",
    "lese ",
    "lies ",
    "führe aus ",
    "fuehre aus ",
    "starte ",
    "leere bildschirm",
    "wo bin ich",
    "aktueller pfad",
    "befehle",
    "hilfe",
    "beende",
    "verlasse",
    // English – multi-word first
    "show all files",
    "show files",
    "list all files",
    "list files",
    "create folder ",
    "create directory ",
    "create dir ",
    "change to ",
    "go to ",
    "navigate to ",
    "make dir ",
    "make directory ",
    "delete ",
    "remove ",
    "copy ",
    "move ",
    "rename ",
    "find ",
    "search ",
    "read ",
    "show ",
    "print ",
    "open ",
    "run ",
    "execute ",
    "where am i",
    "current path",
    "current directory",
    "clear screen",
    "commands",
    "help",
    "quit",
    "exit",
    "bye",
    // Short Unix aliases
    "ls",
    "ls -la",
    "ls -a",
    "cd ",
    "mkdir ",
    "rm ",
    "cp ",
    "mv ",
    "cat ",
    "grep ",
    "pwd",
    "clear",
    "cls",
];

/// Custom helper that completes both natural-language commands and filenames.
#[derive(Helper, Hinter, Validator, Highlighter)]
pub struct NshHelper {
    #[rustyline(Completer)]
    file_completer: FilenameCompleter,
}

impl NshHelper {
    pub fn new() -> Self {
        NshHelper { file_completer: FilenameCompleter::new() }
    }
}

impl Completer for NshHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let lower = line[..pos].to_lowercase();

        // 1. Try NL command prefix completion on the whole line
        let nl_matches: Vec<Pair> = NL_COMMANDS
            .iter()
            .filter(|cmd| cmd.starts_with(&lower as &str) && **cmd != lower.trim())
            .map(|cmd| Pair {
                display: cmd.to_string(),
                replacement: cmd.to_string(),
            })
            .collect();

        if !nl_matches.is_empty() {
            return Ok((0, nl_matches));
        }

        // 2. If the line already has a command prefix, try file completion on
        //    the last "word" (the argument part).
        self.file_completer.complete(line, pos, ctx)
    }
}
