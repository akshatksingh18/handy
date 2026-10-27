//! Tray menu strings
//!
//! Everything is auto-generated at compile time by build.rs from the
//! frontend locale files (src/i18n/locales/*/translation.json). The app ships
//! English only; any other locale folder added later is picked up the same way.
//!
//! The English translation.json is the single source of truth:
//! - TrayStrings struct fields are derived from the English "tray" keys
//! - All languages are auto-discovered from the locales directory
//!
//! To add a new tray menu item:
//! 1. Add the key to en/translation.json under "tray"
//! 2. Update tray.rs to use the new field (e.g., strings.new_field)

use crate::chinese_script::chinese_script_for_locale;
use crate::settings::ChineseScript;
use once_cell::sync::Lazy;
use std::collections::HashMap;

// Include the auto-generated TrayStrings struct and TRANSLATIONS static
include!(concat!(env!("OUT_DIR"), "/tray_translations.rs"));

/// Get localized tray menu strings based on the system locale.
///
/// Lookup order: exact locale → Chinese script/region fallback → language code → English.
pub fn get_tray_translations(locale: Option<String>) -> TrayStrings {
    let normalized = locale
        .as_deref()
        .unwrap_or("en")
        .to_lowercase()
        .replace('_', "-");
    let language = normalized.split('-').next().unwrap_or("en");

    let exact_match = TRANSLATIONS
        .iter()
        .find_map(|(code, strings)| code.eq_ignore_ascii_case(&normalized).then_some(strings));
    let fallback = match chinese_script_for_locale(&normalized) {
        Some(ChineseScript::TraditionalTw | ChineseScript::TraditionalHk) => "zh-TW",
        Some(_) => "zh",
        None => language,
    };

    exact_match
        .or_else(|| TRANSLATIONS.get(fallback))
        .or_else(|| TRANSLATIONS.get("en"))
        .cloned()
        .expect("English translations must exist")
}

#[cfg(test)]
mod tests {
    use super::{get_tray_translations, TRANSLATIONS};

    #[test]
    fn every_locale_falls_back_to_english() {
        for locale in ["en-US", "zh-Hant-TW", "zh-Hans-CN", "fr-FR", "xx-YY"] {
            assert_eq!(
                format!("{:?}", get_tray_translations(Some(locale.into()))),
                format!("{:?}", TRANSLATIONS["en"]),
                "{locale} should resolve to English"
            );
        }
    }
}
