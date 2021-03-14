use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("argumentos insuficientes");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problema ao passar os argumentos: {}", err);
        process::exit(1);
    });

    let mut file = File::open(config.filename).expect("arquivo não encontrado");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("erro ao ler o arquivo");

    println!("{}", contents);
}
