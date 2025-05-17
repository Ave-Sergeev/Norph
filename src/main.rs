use crate::controllers::embedding_controller::TextEmbeddingController;
use crate::pb::inference_pb;
use crate::setting::settings::Settings;
use env_logger::Builder;
use inference_pb::embedding_server::EmbeddingServer;
use log::LevelFilter;
use std::error::Error;
use std::str::FromStr;
use tonic::transport::Server;

mod controllers;
mod error;
mod pb;
mod service;
mod setting;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = Settings::new("config.yaml").map_err(|err| format!("Failed to load setting: {err}"))?;

    Builder::new()
        .filter_level(LevelFilter::from_str(settings.logging.log_level.as_str()).unwrap_or(LevelFilter::Info))
        .init();

    log::info!("Settings:\n{}", settings.json_pretty());

    let text_embedding_controller = TextEmbeddingController::new(&settings.embeddings);

    let address = format!("{}:{}", settings.server.host, settings.server.port)
        .parse()
        .map_err(|err| format!("Invalid server address: {err}"))?;

    log::info!("Server listening on: {address}");

    let text_embedding_server = EmbeddingServer::new(text_embedding_controller).max_decoding_message_size(
        settings
            .embeddings
            .max_text_size
            .as_bytes()
            .try_into()
            .expect("Value max_text_size too large for usize"),
    );

    Server::builder()
        .add_service(text_embedding_server)
        .serve(address)
        .await
        .map_err(|err| format!("GRPC server returned error: {err}"))?;

    Ok(())
}
