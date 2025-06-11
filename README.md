# 📌 FS-Organizer CLI – Rust-Datei-Organizer

> **Wichtig:** Um dieses Tool zu nutzen, ersetze die Standarddateien durch die Dateien dieses Repositories!

---

## 🚩 Ziel des Tools

Dieses Rust-Tool organisiert automatisch Dateien nach Typen, ideal für übersichtliche und strukturierte Ordner.

---

## 📋 Voraussetzungen

- Rust & Cargo ([Installation hier](https://rustup.rs/))

Test der Installation:

```yarn
rustc --version
cargo --version
```

<br>

---

<br>

>⚙️ Anleitung zur Einrichtung
  - 1. Neues Rust-Projekt anlegen
   
```yarn
cargo new fs-organizer
cd fs-organizer
```

- 2. Projektabhängigkeiten installieren
 
```yarn
cargo add glob dirs clap --features derive
```

>⚠️ Dateien ersetzen (aus dem Repository)

>🔹 Cargo.toml ersetzen

- Lösche die Originaldatei.
- Lade Cargo.toml aus diesem Repository herunter.
- Platziere sie im Projektordner.

---

>🔹 src/main.rs ersetzen

- Lösche die Originaldatei src/main.rs.
- Lade main.rs aus diesem Repository herunter.
- Platziere sie im Ordner src/.

> Deine Projektstruktur danach:

```yarn
Organizer/
├── Cargo.toml      # aus Repository ersetzt
└── src
    └── main.rs     # aus Repository ersetzt
```

<br>

---

<br>

> 🚀 Kompilieren und starten

```yarn
cargo build
cargo run
```

<br>

---

<br>

> 📌 Nutzung des Tools
- Standard (Downloads-Ordner):

```yarn
cargo run
```

- Eigene Ordner:

```yarn
cargo run -- --source "Pfad/Quelle" --destination "Pfad/Ziel"
```

- Beispiel:

```yarn
cargo run -- --source "C:/Users/Meister/Desktop" --destination "D:/Sortierte Dateien"
```

<br>

---

<br>

> 📂 Dateien im Repository
- Cargo.toml (Abhängigkeiten & Einstellungen)
- src/main.rs (Hauptfunktionalität)
