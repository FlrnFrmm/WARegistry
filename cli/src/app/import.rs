use clap::{App, Arg, ArgMatches};
use tonic::Request;
use futures_util::stream;

use super::push_client::PushClient;
use super::{PushRequest, push_request::Data, WaMetadata, };

pub fn create<'a>() -> App<'a, 'a> {
    App::new("import")
        .about("Import a WA into the local registry")
        .arg(create_source_argument())
        .arg(create_destination_argument())
}

fn create_source_argument<'a>() -> Arg<'a, 'a> {
    Arg::with_name("source")
        .required(true)
        .help("Path to the WA file")
}

fn create_destination_argument<'a>() -> Arg<'a, 'a> {
    Arg::with_name("destination")
        .required(true)
        .help("A destination for the new WA in the form <namespace>/<WA-Name>:<SemVer>")
}

pub async fn process_matches<'a>(matches: &ArgMatches<'a>) -> anyhow::Result<()> {
    let source = matches.value_of("source").unwrap();
    let destination = matches.value_of("destination").unwrap();

    match std::fs::read_to_string(source) {
        Ok(file_content) => {
            println!("About to write ...\n\n{}\n\n ... to {}", file_content, destination);
        }
        Err(error) => {
            println!("A error occured while trying to read '{}': {}", source, error);
        }
    }

    let wasm_bytes = std::fs::read(source)?;

    let channel = tonic::transport::Channel::from_static("http://[::1]:10815")
        .connect()
        .await?;
    
    let mut client = PushClient::new(channel);

    let messages: Vec<PushRequest> = vec![
        PushRequest {
           data: Some(Data::Metadata(WaMetadata {
               name: "MyCompany/MyProject".into(),
               namespace: "SuperAwesomeApp".into(),
               semver: "0.1.0".into()
           }))
        },
        PushRequest {
           data: Some(Data::ChunkData(wasm_bytes))
        },
    ];

    let response = client.push(stream::iter(messages)).await?.into_inner();
    println!("RESPONSE={:?}", response);

    Ok(())
}