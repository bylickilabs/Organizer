use std::fs;
use glob::glob;
use dirs;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "fs-organizer", version = "0.1", author = "Meister", about = "Organisiert Dateien automatisch.")]
struct Args {
    /// Quellverzeichnis (Standard: Downloads-Ordner)
    #[arg(short, long)]
    source: Option<String>,

    /// Zielverzeichnis (Standard: gleich wie Quellverzeichnis)
    #[arg(short, long)]
    destination: Option<String>,
}

// Ordner basierend auf Dateiendung bestimmen
fn get_folder(ext: &str) -> &str {
    match ext {
        "jpg" | "jpeg" | "png" | "gif" => "Bilder",
        "pdf" | "docx" | "xlsx" | "pptx" | "doc" => "Dokumente",
        "mp4" | "mov" | "avi" => "Videos",
        "mp3" | "wav" => "Audio",
        "zip" | "rar" | "7z" => "Archive",
        _ => "Sonstiges",
    }
}

fn main() {
    // Argumente parsen
    let args = Args::parse();

    let source_path = match &args.source {
        Some(path) => std::path::PathBuf::from(path),
        None => dirs::download_dir().expect("Standard Download-Verzeichnis nicht gefunden!"),
    };

    let destination_path = match &args.destination {
        Some(path) => std::path::PathBuf::from(path),
        None => source_path.clone(),
    };

    // Dateien im Quellordner erfassen
    let pattern = format!("{}/*.*", source_path.display());

    for entry in glob(&pattern).expect("Fehler beim Lesen des Ordners.") {
        match entry {
            Ok(path) => {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let ext_lower = ext.to_lowercase();
                    let folder_name = get_folder(&ext_lower);

                    let folder_path = destination_path.join(folder_name);

                    if !folder_path.exists() {
                        fs::create_dir_all(&folder_path).expect("Ordner konnte nicht erstellt werden.");
                    }

                    let new_path = folder_path.join(path.file_name().unwrap());
                    fs::rename(&path, &new_path).expect("Datei konnte nicht verschoben werden.");

                    println!("Verschoben: {} → {}", path.display(), new_path.display());
                }
            }
            Err(e) => println!("Fehler beim Lesen der Datei: {:?}", e),
        }
    }

    println!("Organisation abgeschlossen!");
}
