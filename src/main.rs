use std::{env, process};
use pngtojpeg::{Config};

// TODO: Should we use this? https://docs.rs/bytes/latest/bytes/
// https://en.wikipedia.org/wiki/PNG#PNG_Working_Group
// https://www.w3.org/TR/png/#5PNG-file-signature

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = pngtojpeg::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}
