# sup_common

Crate commun pour les utilitaires partagés entre les crates `sup_*`.

## 🎯 **Objectif**

Ce crate contient uniquement des macros de debug conditionnelles qui ne s'affichent qu'en mode développement.

## 📦 **Contenu**

### Macros de debug disponibles :

- `debug_println!` - Logs normaux
- `debug_eprintln!` - Logs d'erreur  
- `debug_info!` - Logs d'information avec emoji ℹ️
- `debug_success!` - Logs de succès avec emoji ✅
- `debug_error!` - Logs d'erreur avec emoji ❌
- `debug_warn!` - Logs d'avertissement avec emoji ⚠️

## 🚀 **Utilisation**

```rust
use sup_common::{debug_println, debug_eprintln, debug_info, debug_success, debug_error, debug_warn};

// Ces macros ne s'affichent qu'en mode debug (cargo run)
debug_println!("Message de debug");
debug_info!("Information importante");
debug_success!("Opération réussie");
debug_error!("Erreur détectée");
debug_warn!("Avertissement");

// En mode release (cargo run --release), ces macros ne produisent aucun output
```

## ⚙️ **Comportement**

- **Mode debug** (`cargo run`) : Les macros affichent les messages
- **Mode release** (`cargo run --release`) : Les macros sont vides (aucun output)

## 📝 **Note importante**

Les dépendances communes (`serde`, `tokio`, `anyhow`, etc.) sont déclarées directement dans chaque crate qui en a besoin, car les macros `#[derive(Serialize, Deserialize)]` nécessitent que ces crates soient disponibles au niveau du crate.

Ce crate ne contient donc aucune dépendance externe pour éviter les conflits de versions.
