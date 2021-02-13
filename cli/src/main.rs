use clap::{App, Arg};

fn main() {
    let matches = App::new("war-cli")
        .version("1.0")
        .author("Florian Fromm <flrnfrmm@gmail.com>")
        .about("Interact with a WARegistry")
        .arg(
            Arg::with_name("version")
                .short("v")
                .help("Show the current version of the cli.")
        )
        .subcommand(
     App::new("push")
                .about("Push a WASM from a WARegistry to a WARegistry.")
                .arg(
                    Arg::with_name("force")
                        .short("f")
                        .help("Force the push operation.")
                )
        )
        .get_matches();
    
    match matches {
        _ => println!("It just works!")
    }
}
