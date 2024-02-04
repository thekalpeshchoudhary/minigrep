use std::process;
use std::env;
use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments : {err}");
        process::exit(1);
    });
    eprintln!(
        "Seacrching for '{}' in the file '{}'",
        config.query, config.file_path
    );
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
