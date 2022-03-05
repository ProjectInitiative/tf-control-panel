use rust_embed::RustEmbed;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use toml::Value;
use crate::constants::*;

/**
 * Why roll my own internationalization (i18n), instad of using existing
 * libraries and crates?
 * 
 *   - This project doesn't need much fancy i18n, so a simple
 *     lightweight and versatile implementation will do.
 *   - Writing a custom i18n implementation is also a good learning
 *     exercise, and gives us additional control and flexibility we can
 *     tweak to the use here.
 *   - No need for format-strings or usability concerns that need to be
 *     architected considering other projects and uses not needed here.
 */

#[derive(RustEmbed)]
#[folder = "src/locales/"]
struct Locale;

/*
thread_local!{
    static I18N_LOCALES: RefCell<HashMap<&'static str, &'static str>> = RefCell::new(HashMap::new());
}*/

// TODO: use system locale through `sys-locale` (https://lib.rs/crates/sys-locale) if possible

/**
 * Load and parse requested locale(s) into memory, defaulting to loading
 * all locales if `None` is specified. Lookup strings are embedded
 * during compilation in production environments.
 * 
 * ```rust
 * init("en-US");  // load just the "en-US" locale
 * init(None);     // load all available locales
 * ```
 */
pub fn init(locale: Option<&'static str>) {
    // TODO: error handling with `Result`, allowing for chaining & conditional logic?
    if let Some(locale_used) = &locale {
        _load_locale(locale_used, &format!("{}.toml", locale_used));
    } else {
        for i18n_filepath in Locale::iter() {
            let i18n_filename = Path::new(i18n_filepath.as_ref()).file_name().unwrap();
            let i18n_basename = Path::new(i18n_filename).file_stem().unwrap();
            _load_locale(i18n_basename.to_str().unwrap(), i18n_filepath.as_ref());
        }
    }
}

/**
 * Load and parse a single locale from the filename specified. Like the
 * `init` function, files are loaded from the embedded virtual
 * filesystem created at compile-time.
 * 
 * ```rust
 * _load_locale("en-US", "en-US.toml");
 * ```
 */
fn _load_locale(locale: &'static str, filename: &str) {
    // TODO: short-circuit early if already parsed & cached
    let i18n_file = Locale::get(filename).unwrap();
    let i18n_filebody = std::str::from_utf8(i18n_file.as_ref()).unwrap();
    let i18n_table = i18n_filebody.parse::<Value>().unwrap();
    // TODO: simplify table hierarchy for further access (testing needed)
    // TODO: load table into cache
    &I18N_LOCALES.as_ptr().insert(locale, "test");
    println!("_load_locale!: {:?}", I18N_LOCALES);
    println!("_load_locale: {:?}", i18n_table);
}

fn _flatten_table() {
    println!("_flatten_table")
}

/**
 * Internationalizes an input string with the specified locale.
 * 
 * ## Defaults
 * 
 * If parameter `locale` is unspecified, defaults to `DEFAULT_LOCALE`.
 * 
 * If no internationalization was found for the specified `msg` and
 * `locale`, tries to lookup using the `DEFAULT_LOCALE`. Finally, if the
 * previous steps fail, returns the input `msg`.
 * 
 * TODO: configuration for fallbacks, update doc
 */
pub fn translate<'a>(msg: &str, locale: Option<&str>) -> &'a str {
    let locale_used = locale.unwrap_or(DEFAULT_LOCALE);
    println!("i18n: {}", locale_used);
    "ff"
}
