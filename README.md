# Simple-Shell (`nsh`)

Eine Shell, die **natürliche Sprache** versteht — auf Deutsch und Englisch.
A shell that understands **natural language** — in German and English.

Statt kryptischer Unix-Befehle tippst du einfach, was du meinst:

```
zeige alle dateien
gehe zu /home
lese README.md
```

---

## Installation

```bash
git clone https://github.com/Jel-Re/Simple-Shell.git
cd Simple-Shell
cargo build --release
./target/release/nsh
```

Oder direkt starten (Debug-Build):

```bash
cargo run
```

---

## Befehle / Commands

### Navigation

| Deutsch                        | Englisch                   | Unix     |
|-------------------------------|----------------------------|----------|
| `zeige dateien`               | `list files`               | `ls`     |
| `zeige alle dateien`          | `show all files`           | `ls -la` |
| `zeige versteckte dateien`    | `show hidden files`        | `ls -a`  |
| `gehe zu <pfad>`              | `go to <path>`             | `cd`     |
| `wo bin ich`                  | `where am i`               | `pwd`    |

### Dateien / Files

| Deutsch                              | Englisch                        | Unix        |
|-------------------------------------|---------------------------------|-------------|
| `erstelle ordner <name>`            | `create folder <name>`          | `mkdir`     |
| `lösche <datei>`                    | `delete <file>`                 | `rm`        |
| `kopiere <von> nach <nach>`         | `copy <from> to <to>`           | `cp`        |
| `verschiebe <von> nach <nach>`      | `move <from> to <to>`           | `mv`        |
| `lese <datei>`                      | `read <file>`                   | `cat`       |
| `suche <muster> in <datei>`         | `find <pattern> in <file>`      | `grep`      |

### Skripte / Scripts

| Deutsch                        | Englisch            |
|-------------------------------|---------------------|
| `führe <datei.nsh> aus`       | `run <file.nsh>`    |
| `<datei.nsh>`                 | `<file.nsh>`        |

### Sonstiges / Misc

| Deutsch              | Englisch       |
|---------------------|----------------|
| `hilfe`             | `help`         |
| `leere bildschirm`  | `clear screen` |
| `verlasse`          | `exit`         |

---

## Skripte (.nsh)

Skripte sind einfache Textdateien mit der Endung `.nsh`.
Jede Zeile ist ein natürlichsprachiger Befehl. Zeilen mit `#` sind Kommentare.

```bash
# mein-skript.nsh

# Projektordner anlegen
erstelle ordner mein-projekt
gehe zu mein-projekt

# Datei anzeigen
lese README.md
```

Ausführen:
```
führe mein-skript.nsh aus
```

Oder direkt als Argument beim Start:
```bash
./target/release/nsh mein-skript.nsh
```

---

## Features

- **Natürliche Sprache** — Deutsch und Englisch, gemischt nutzbar
- **Tab-Vervollständigung** — für alle Befehle und Dateinamen
- **Befehlshistorie** — mit `↑` / `↓` durch vorherige Befehle navigieren, gespeichert in `~/.nsh_history`
- **Farbige Ausgabe** — Ordner blau, Fehler rot, Erfolg grün
- **Fallback** — unbekannte Eingaben werden direkt als Systembefehl ausgeführt (`git status`, `cargo build`, ...)
- **Skript-Support** — `.nsh`-Dateien mit Kommentaren und NL-Befehlen

---

## Tastenkürzel / Shortcuts

| Taste      | Funktion                          |
|-----------|-----------------------------------|
| `Tab`      | Autovervollständigung             |
| `↑` / `↓` | Befehlshistorie                   |
| `Ctrl+C`  | Aktuelle Eingabe abbrechen        |
| `Ctrl+D`  | Shell beenden                     |

---

## Voraussetzungen / Requirements

- [Rust](https://rustup.rs/) ≥ 1.70

---

## Lizenz / License

MIT
