extern crate minigrep_rust;

use minigrep_rust::Config;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problema ao passar os argumentos: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep_rust::run(config) {
        println!("Erro na aplicação: {}", e);
        process::exit(1);
    };
}
