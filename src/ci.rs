use shadow_rs::shadow;
use std::env;
shadow!(build);
use commands::InstallArgs;

#[allow(unused)]
mod commands;

#[allow(unused)]
mod utils;

/// Simple version of a ci installer
fn main() {
    let args: Vec<String> = env::args().collect();
    let force = args.contains(&"--force".to_string()) || args.contains(&"-f".to_string());
    let install = InstallArgs { url: None, force };
    match commands::install::run(&install) {
        Err(err) => println!("{}", err.to_string()),
        Ok(_) => println!("Everything is good to go!"),
    }
}
