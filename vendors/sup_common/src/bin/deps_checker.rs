//! Script CLI pour vérifier et mettre à jour les dépendances dans tous les Cargo.toml du workspace

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;
use toml::Value;

#[derive(Parser)]
#[command(name = "deps-checker")]
#[command(about = "Vérifie et met à jour les dépendances dans tous les Cargo.toml du workspace")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Vérifie les versions des dépendances
    Check {
        /// Mettre à jour automatiquement les versions
        #[arg(short, long)]
        update: bool,
        /// Mode interactif (demande confirmation pour chaque mise à jour)
        #[arg(short, long)]
        interactive: bool,
        /// Mode silencieux (pas de couleurs/emojis)
        #[arg(short, long)]
        quiet: bool,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct CrateInfo {
    name: String,
    newest_version: String,
    current_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CrateResponse {
    #[serde(rename = "crate")]
    crate_info: CrateInfo,
}

struct DependencyInfo {
    name: String,
    current_version: String,
    newest_version: Option<String>,
    file_path: PathBuf,
}

// Fonctions d'affichage coloré
fn print_success(msg: &str, quiet: bool) {
    if !quiet {
        println!("✅ {}", msg);
    } else {
        println!("SUCCESS: {}", msg);
    }
}

fn print_info(msg: &str, quiet: bool) {
    if !quiet {
        println!("ℹ️  {}", msg);
    } else {
        println!("INFO: {}", msg);
    }
}

fn print_warning(msg: &str, quiet: bool) {
    if !quiet {
        println!("⚠️  {}", msg);
    } else {
        println!("WARNING: {}", msg);
    }
}

fn print_error(msg: &str, quiet: bool) {
    if !quiet {
        println!("❌ {}", msg);
    } else {
        println!("ERROR: {}", msg);
    }
}

fn print_update(msg: &str, quiet: bool) {
    if !quiet {
        println!("🔄 {}", msg);
    } else {
        println!("UPDATE: {}", msg);
    }
}

fn print_question(msg: &str, quiet: bool) {
    if !quiet {
        println!("❓ {}", msg);
    } else {
        println!("QUESTION: {}", msg);
    }
}

fn print_header(msg: &str, quiet: bool) {
    if !quiet {
        println!("\n🚀 {}", msg);
        println!("{}", "=".repeat(80));
    } else {
        println!("\nHEADER: {}", msg);
        println!("{}", "=".repeat(80));
    }
}

// Fonction pour demander confirmation à l'utilisateur
fn ask_user_confirmation(question: &str, quiet: bool) -> bool {
    if quiet {
        return true; // En mode silencieux, on accepte tout
    }
    
    print_question(question, quiet);
    print!("Votre choix (y/N/a pour tout accepter): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_lowercase();
    
    match input.as_str() {
        "y" | "yes" | "oui" => true,
        "a" | "all" | "tout" => {
            print_info("Toutes les mises à jour suivantes seront acceptées automatiquement", quiet);
            true
        }
        _ => false,
    }
}

async fn get_latest_version(crate_name: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let url = format!("https://crates.io/api/v1/crates/{}", crate_name);
    let client = reqwest::Client::builder()
        .user_agent("deps-checker/1.0")
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    
    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let json: serde_json::Value = response.json().await?;
                
                if let Some(crate_info) = json.get("crate") {
                    if let Some(newest_version) = crate_info.get("newest_version") {
                        if let Some(version) = newest_version.as_str() {
                            return Ok(Some(version.to_string()));
                        }
                    }
                }
                
                // Essayer une autre structure possible
                if let Some(newest_version) = json.get("newest_version") {
                    if let Some(version) = newest_version.as_str() {
                        return Ok(Some(version.to_string()));
                    }
                }
                
                Ok(None)
            } else {
                // Gérer les erreurs HTTP
                if response.status() == 429 {
                    // Rate limit - attendre un peu plus
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                }
                Ok(None)
            }
        }
        Err(_e) => {
            // Erreur de connexion - ne pas spammer les logs
            Ok(None)
        }
    }
}

fn find_cargo_toml_files() -> Vec<PathBuf> {
    let mut files = Vec::new();
    
    // Chercher dans les sous-répertoires (ignorer le workspace root)
    // Utiliser des chemins relatifs au workspace root (../../)
    let subdirs = ["../../sup_discord_app/src-tauri", "../../vendors/sup_drpc", "../../vendors/sup_mtracker", "../../vendors/sup_common"];
    for subdir in &subdirs {
        let path = Path::new(subdir).join("Cargo.toml");
        if path.exists() {
            files.push(path);
        }
    }
    
    files
}

fn parse_dependencies_from_cargo_toml(file_path: &Path) -> Result<Vec<DependencyInfo>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut dependencies = Vec::new();
    
    // Parser le TOML
    let toml_value: Value = toml::from_str(&content)?;
    
    if let Some(deps) = toml_value.get("dependencies").and_then(|d| d.as_table()) {
        for (name, dep_value) in deps {
            if let Some(version) = extract_version_from_dep(dep_value) {
                dependencies.push(DependencyInfo {
                    name: name.clone(),
                    current_version: version,
                    newest_version: None,
                    file_path: file_path.to_path_buf(),
                });
            }
        }
    }
    
    Ok(dependencies)
}

fn extract_version_from_dep(dep_value: &Value) -> Option<String> {
    match dep_value {
        Value::String(version) => Some(version.clone()),
        Value::Table(table) => {
            if let Some(version) = table.get("version").and_then(|v| v.as_str()) {
                Some(version.to_string())
            } else {
                None
            }
        }
        _ => None,
    }
}

fn compare_versions(current: &str, latest: &str) -> bool {
    // Comparaison simple des versions (peut être améliorée avec semver)
    current != latest
}

async fn check_dependencies(update: bool, interactive: bool, quiet: bool) -> Result<(), Box<dyn std::error::Error>> {
    print_header("Vérification des dépendances Rust", quiet);
    
    print_info("Recherche des fichiers Cargo.toml...", quiet);
    
    let cargo_files = find_cargo_toml_files();
    if cargo_files.is_empty() {
        print_error("Aucun fichier Cargo.toml trouvé", quiet);
        return Ok(());
    }
    
    print_info(&format!("Fichiers trouvés: {}", cargo_files.len()), quiet);
    for file in &cargo_files {
        print_info(&format!("  - {}", file.display()), quiet);
    }
    println!();
    
    let mut all_dependencies = Vec::new();
    
    // Parser tous les fichiers
    for file_path in &cargo_files {
        print_info(&format!("Analyse de {}...", file_path.display()), quiet);
        let deps = parse_dependencies_from_cargo_toml(file_path)?;
        all_dependencies.extend(deps);
    }
    
    print_info("Vérification des versions sur crates.io...", quiet);
    
    // Vérifier les versions pour chaque dépendance
    let total_deps = all_dependencies.len();
    for (processed, dep) in all_dependencies.iter_mut().enumerate() {
        let current = processed + 1;
        if !quiet {
            print!("\r  Progression: {}/{}", current, total_deps);
            io::stdout().flush().unwrap();
        }
        
        if let Ok(Some(latest_version)) = get_latest_version(&dep.name).await {
            dep.newest_version = Some(latest_version);
        }
        
        // Pause pour éviter de surcharger l'API (rate limiting)
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }
    
    if !quiet {
        println!(); // Nouvelle ligne après la barre de progression
    }
    
    // Afficher les résultats
    print_header("Résultats de la vérification", quiet);
    
    let mut has_updates = false;
    let mut updates_to_apply = Vec::new();
    let mut up_to_date_count = 0;
    let mut not_found_count = 0;
    
    for dep in &all_dependencies {
        if let Some(ref latest) = dep.newest_version {
            if compare_versions(&dep.current_version, latest) {
                has_updates = true;
                print_update(&format!("{}: {} → {} ({})", 
                    dep.name, 
                    dep.current_version, 
                    latest, 
                    dep.file_path.display()
                ), quiet);
                
                updates_to_apply.push(dep);
            } else {
                up_to_date_count += 1;
                if !quiet {
                    print_success(&format!("{}: {} (à jour)", dep.name, dep.current_version), quiet);
                }
            }
        } else {
            not_found_count += 1;
            print_warning(&format!("{}: {} (version non trouvée)", dep.name, dep.current_version), quiet);
        }
    }
    
    // Résumé
    println!();
    print_info(&format!("Résumé: {} à jour, {} mises à jour disponibles, {} versions non trouvées", 
        up_to_date_count, updates_to_apply.len(), not_found_count), quiet);
    
    if !has_updates {
        print_success("Toutes les dépendances sont à jour !", quiet);
        return Ok(());
    }
    
    if !update {
        print_info("Utilisez --update pour mettre à jour les dépendances", quiet);
        return Ok(());
    }
    
    // Appliquer les mises à jour
    print_header("Mise à jour des dépendances", quiet);
    
    let mut updated_count = 0;
    let mut skipped_count = 0;
    let mut error_count = 0;
    
    for dep in &updates_to_apply {
        let should_update = if interactive {
            let question = format!("Mettre à jour {} de {} vers {} ?", 
                dep.name, dep.current_version, dep.newest_version.as_ref().unwrap());
            ask_user_confirmation(&question, quiet)
        } else {
            true
        };
        
        if should_update {
            if let Err(e) = update_dependency_in_file(&dep.file_path, &dep.name, dep.newest_version.as_ref().unwrap()) {
                print_error(&format!("Erreur lors de la mise à jour de {}: {}", dep.name, e), quiet);
                error_count += 1;
            } else {
                print_success(&format!("{} mis à jour vers {}", dep.name, dep.newest_version.as_ref().unwrap()), quiet);
                updated_count += 1;
            }
        } else {
            print_info(&format!("{} ignoré", dep.name), quiet);
            skipped_count += 1;
        }
    }
    
    // Résumé final
    println!();
    print_info(&format!("Résumé des mises à jour: {} mises à jour, {} ignorées, {} erreurs", 
        updated_count, skipped_count, error_count), quiet);
    
    print_success("Vérification terminée !", quiet);
    Ok(())
}

fn update_dependency_in_file(file_path: &Path, dep_name: &str, new_version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    
    for (_i, line) in lines.iter_mut().enumerate() {
        if line.contains(&format!("{} = ", dep_name)) {
            // Remplacer la version dans la ligne
            if let Some(version_start) = line.find("version = \"") {
                if let Some(version_end) = line[version_start + 11..].find('"') {
                    let start = version_start + 11;
                    let end = start + version_end;
                    line.replace_range(start..end, new_version);
                    break;
                }
            }
        }
    }
    
    fs::write(file_path, lines.join("\n"))?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Check { update, interactive, quiet } => {
            if let Err(e) = check_dependencies(update, interactive, quiet).await {
                if quiet {
                    eprintln!("ERROR: {}", e);
                } else {
                    eprintln!("❌ Erreur: {}", e);
                }
                process::exit(1);
            }
        }
    }
}
