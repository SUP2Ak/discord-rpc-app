# sup_common

Crate commun pour les utilitaires partagés entre les crates `sup_*`.

## Contenu

### Macros de debug

Ce crate contient des macros de debug conditionnelles qui s'affichent uniquement en mode debug :

- `debug_println!` - Équivalent à `println!` mais uniquement en debug
- `debug_eprintln!` - Équivalent à `eprintln!` mais uniquement en debug
- `debug_info!` - Messages d'information avec emoji ℹ️
- `debug_success!` - Messages de succès avec emoji ✅
- `debug_error!` - Messages d'erreur avec emoji ❌
- `debug_warn!` - Messages d'avertissement avec emoji ⚠️

### Script de vérification des dépendances

Le binaire `deps-checker` permet de vérifier et mettre à jour les dépendances Rust dans tous les `Cargo.toml` du workspace.

#### Utilisation

**Méthode recommandée (depuis la racine du projet) :**

```bash
# Vérifier les dépendances (lecture seule)
bun run deps:check

# Mettre à jour automatiquement toutes les dépendances
bun run deps:update

# Mode interactif - demande confirmation pour chaque mise à jour
bun run deps:interactive

# Mode silencieux (sans couleurs/emojis)
bun run deps:quiet
```

**Méthode alternative (via Cargo directement) :**

```bash
cd vendors/sup_common

# Vérifier les dépendances (lecture seule)
cargo run --bin deps-checker check

# Mettre à jour automatiquement toutes les dépendances
cargo run --bin deps-checker check --update

# Mode interactif - demande confirmation pour chaque mise à jour
cargo run --bin deps-checker check --update --interactive

# Mode silencieux (sans couleurs/emojis)
cargo run --bin deps-checker check --update --quiet
```

> 💡 **Recommandation** : Utilise la méthode `bun run` depuis la racine du projet pour une meilleure expérience utilisateur.

#### Fonctionnalités

- 🔍 **Détection automatique** : Trouve tous les `Cargo.toml` dans le workspace
- 🌐 **Vérification en ligne** : Interroge l'API de crates.io pour les dernières versions
- 🎯 **Mode interactif** : Demande confirmation pour chaque mise à jour
- 📊 **Rapports détaillés** : Affiche un résumé complet des mises à jour
- 🚀 **Interface colorée** : Utilise des emojis et couleurs pour une meilleure UX
- ⚡ **Rate limiting** : Respecte les limites de l'API crates.io
- 🔧 **Mise à jour automatique** : Modifie directement les fichiers `Cargo.toml`

#### Options disponibles

- `--update` : Active la mise à jour des dépendances
- `--interactive` : Mode interactif (demande confirmation)
- `--quiet` : Mode silencieux (pas de couleurs/emojis)

#### Exemple de sortie

```
🚀 Vérification des dépendances Rust
================================================================================
ℹ️  Recherche des fichiers Cargo.toml...
ℹ️  Fichiers trouvés: 4
ℹ️    - ../../sup_discord_app/src-tauri\Cargo.toml
ℹ️    - ../../vendors/sup_drpc\Cargo.toml
ℹ️    - ../../vendors/sup_mtracker\Cargo.toml
ℹ️    - ../../vendors/sup_common\Cargo.toml

🚀 Résultats de la vérification
================================================================================
✅ serde: 1.0.219 (à jour)
🔄 tauri: 2 → 2.8.5 (../../sup_discord_app/src-tauri\Cargo.toml)
🔄 clap: 4.0 → 4.5.47 (../../vendors/sup_common\Cargo.toml)

ℹ️  Résumé: 16 à jour, 2 mises à jour disponibles, 0 versions non trouvées

🚀 Mise à jour des dépendances
================================================================================
❓ Mettre à jour tauri de 2 vers 2.8.5 ?
Votre choix (y/N/a pour tout accepter): y
✅ tauri mis à jour vers 2.8.5
```

## Installation

Ce crate fait partie du workspace et est automatiquement disponible pour les autres crates via :

```toml
[dependencies]
sup_common = { path = "../sup_common" }
```

## Dépendances

- `clap` - Interface en ligne de commande
- `serde` - Sérialisation/désérialisation
- `serde_json` - Support JSON
- `tokio` - Runtime asynchrone
- `reqwest` - Client HTTP
- `toml` - Parser TOML