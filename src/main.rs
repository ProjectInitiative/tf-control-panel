// import all modules into root for sibling access to other modules
mod constants;
mod options;

fn main() {
    // Parse CLI options and arguments
    let matches = options::get_matches();
}
