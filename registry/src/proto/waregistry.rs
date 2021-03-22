use tonic::{Request, Response, Status, Streaming};
use futures::StreamExt;

use super::{
    say_server::{Say}, SayRequest, SayResponse,
    push_server::{Push}, PushRequest, PushResponse, push_request::Data, WaMetadata};

#[derive(Default)]
pub struct MySay {}

#[tonic::async_trait]
impl Say for MySay {
    async fn send(&self, request: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
        let response = SayResponse { message:format!("hello {}", request.get_ref().name) };
        Ok(Response::new(response))
    }

    async fn take(&self,request: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
        let response = SayResponse { message:format!("hello {}", request.get_ref().name) };
        Ok(Response::new(response))
    }
}

#[derive(Default)]
pub struct PushService {}

#[tonic::async_trait]
impl Push for PushService {
    async fn push(&self, request: Request<Streaming<PushRequest>>) -> Result<Response<PushResponse>, Status> {
        let mut stream = request.into_inner();
        
        let mut metadata: Option<WaMetadata> = None;
        let mut bytes: Vec<u8> = Vec::new();

        while let Some(push_request_result) = stream.next().await {
            let push_request = push_request_result?;

            if let Some(data) = push_request.data {
                match data {
                    Data::Metadata(data) => { metadata = Some(data) },
                    Data::ChunkData(chunk) => { bytes.extend(chunk.iter()) }
                }
            }
        }

        if let Some(WaMetadata { name, namespace, semver}) = metadata {
            let full_path_as_string = format!("./data/{}/{}:{}", namespace, name, semver);
            let full_path = std::path::Path::new(&full_path_as_string);

            std::fs::create_dir_all(full_path.parent().unwrap())?;
            std::fs::write(full_path, bytes)?;
        }

        let response = PushResponse { id: "42".into(), size: 42 };
        Ok(Response::new(response))
    }
}