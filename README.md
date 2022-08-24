# Zahlenraten
Zahlenraten ist ein kleines, in Rust geschriebenes Konsolenspiel. Das Spiel generiert automatisch eine Zahl zwischen 1 und 125, die der Spieler dann erraten muss. Dabei werden seine Versuche gezählt.

## Erstellen des Projekts
Klone dieses Git-Repository mit `git clone`. Installiere dann, wenn noch nicht geschehen, Rust mit Cargo von [rust-lang.org](https://www.rust-lang.org/). Navigiere nun in das geklonte Repository und erstelle das Projekt mit `cargo build` (hiermit wird das Projekt nur erstellt und nicht automatisch ausgeführt) oder mit `cargo run` (hiermit wird das Projekt nicht nur erstellt sondern auch direkt gestartet).
>**Beachte:** Um die Version des Projekts zu erstellen, die du später weitergeben wirst, also die, die für den Release vorgesehen ist, solltest du das Projekt mit `cargo build --release` erstellen, da der Build nun noch Optimierungen enthält.
