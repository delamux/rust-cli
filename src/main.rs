use std::env;

use cli_project::{run, Params};


fn main() {
    // it save in an array of strings
    let args: Vec<String> = env::args().collect();

    // do a destructuring of parse_config
    let params: Params = Params::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });


    run(params).unwrap_or_else(|err| {
        println!("Application error: {}", err);
        std::process::exit(1);
    });

}
