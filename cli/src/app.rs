mod import;

tonic::include_proto!("waregistry");

use clap::{App, Arg, ArgMatches, crate_version};

pub fn create<'a>() -> App<'a, 'a> {
    App::new("warc")
        .version(current_version())
        .author("Florian Fromm <flrnfrmm@gmail.com>")
        .about("CLI tool to interact with a WARegistry")
        .arg(create_version_argument())
        .subcommand(import::create())
}

fn create_version_argument<'a>() -> Arg<'a, 'a> {
    Arg::with_name("version")
        .long("version")
        .short("v")
        .required(false)
        .takes_value(false)
        .help("Show the current version of the cli")
}

fn current_version<'a>() -> &'a str {
    crate_version!()
}

pub async fn process_matches<'a>(matches: &ArgMatches<'a>) -> anyhow::Result<()> {
    if matches.is_present("version") {
        println!("Current version of warc is {}", current_version())
    }

    if let Some(import_matches) = matches.subcommand_matches("import") {
        import::process_matches(import_matches).await?
    }

    Ok(())
}