use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use sup_mtracker::RealtimeProcessMonitor;

pub type ProcessMonitors = Arc<RwLock<HashMap<String, Arc<Mutex<RealtimeProcessMonitor>>>>>;
pub type RichPresenceManager = Arc<tokio::sync::Mutex<sup_drpc::RichPresenceManager>>;
pub type SavedConfigs = Arc<RwLock<HashMap<String, SavedConfig>>>;
pub type ActiveSessions = Arc<RwLock<HashMap<String, MonitoringSession>>>;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PathType {
    Normal, // pour les chemins normaux
    Conditional, // pour les patterns conditionnels
}

// Chemin sélectionné avec son type et ses métadonnées (unifié)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedPath {
    pub path_type: PathType,
    pub value: String,
    pub pattern_id: Option<String>, // uniquement pour les patterns conditionnels
    pub default_value: Option<String>,
}

// Pattern conditionnel pour la sélection dynamique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalPattern {
    pub id: String,
    pub array_path: String, // ex: "windows"
    pub filter_path: String, // ex: "windows.3.class_name"
    pub filter_value: String, // ex: "WinUIDesktopWin32WindowClass"
    pub target_path: String, // ex: "windows.3.window_title"
    pub pattern: String, // ex: "[windows.?.class_name='WinUIDesktopWin32WindowClass'].window_title"
}

// Configuration Discord Rich Presence avec patterns conditionnels etc...
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichPresenceConfig {
    #[serde(flatten)] // Étendre la configuration de base de sup_drpc
    pub base_config: sup_drpc::RichPresenceConfig,
    pub selected_paths: Vec<SelectedPath>,
    pub conditional_patterns: Option<Vec<ConditionalPattern>>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedConfig {
    pub id: String,
    pub name: String,
    pub executable_name: String,
    pub discord_app_name: String,
    pub config: RichPresenceConfig,
    pub scan_options: sup_mtracker::MetadataOptions,
    pub check_interval: u64,
    pub auto_start_monitoring: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringSession {
    pub id: String,
    pub config_id: String,
    pub executable_name: String,
    pub discord_application_id: String,
    pub discord_app_name: String,
    pub is_active: bool,
    pub last_update: String,
    pub current_metadata: Option<sup_mtracker::ProcessMetadata>,
}
