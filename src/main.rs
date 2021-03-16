extern crate minigrep_rust;

use minigrep_rust::Config;
use std::env;
use std::process;
fn main() {
    // let args: Vec<String> = env::args().collect();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problema ao passar os argumentos: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep_rust::run(config) {
        eprintln!("Erro na aplicação: {}", e);
        process::exit(1);
    };
}
