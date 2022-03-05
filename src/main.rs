// import all modules into root for sibling access to other modules
mod constants;
mod options;
mod i18n;

fn main() {
    i18n::init(Some("en-US"));
    i18n::translate("test", None);

    // Parse CLI options and arguments
    let matches = options::get_matches();
}
