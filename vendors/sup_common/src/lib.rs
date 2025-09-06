//! Crate commun pour les utilitaires partagés entre les crates sup_*
//! 
//! Ce crate contient uniquement des macros de debug conditionnelles.
//! Les dépendances communes sont déclarées directement dans chaque crate
//! pour permettre l'utilisation des macros #[derive].

/// Macro pour les logs de debug - s'affiche uniquement en mode debug
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        println!($($arg)*);
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {};
}

/// Macro pour les logs d'erreur de debug - s'affiche uniquement en mode debug
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_eprintln {
    ($($arg:tt)*) => {
        eprintln!($($arg)*);
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_eprintln {
    ($($arg:tt)*) => {};
}

/// Macro pour les logs d'info - s'affiche uniquement en mode debug
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_info {
    ($($arg:tt)*) => {
        println!("ℹ️ {}", format!($($arg)*));
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_info {
    ($($arg:tt)*) => {};
}

/// Macro pour les logs de succès - s'affiche uniquement en mode debug
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_success {
    ($($arg:tt)*) => {
        println!("✅ {}", format!($($arg)*));
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_success {
    ($($arg:tt)*) => {};
}

/// Macro pour les logs d'erreur - s'affiche uniquement en mode debug
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_error {
    ($($arg:tt)*) => {
        eprintln!("❌ {}", format!($($arg)*));
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_error {
    ($($arg:tt)*) => {};
}

/// Macro pour les logs d'avertissement - s'affiche uniquement en mode debug
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_warn {
    ($($arg:tt)*) => {
        println!("⚠️ {}", format!($($arg)*));
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_warn {
    ($($arg:tt)*) => {};
}
