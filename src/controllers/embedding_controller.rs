use crate::pb::inference_pb;
use crate::pb::inference_pb::{TextEmbeddingRequest, TextEmbeddingResponse};
use crate::service::text_embedded::TextEmbedded;
use crate::setting::settings::EmbeddingSettings;
use futures::{Stream, StreamExt};
use inference_pb::embedding_server;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Request, Response, Status, Streaming};

pub struct TextEmbeddingController {
    pub text_embedded: Arc<TextEmbedded>,
}

impl TextEmbeddingController {
    pub fn new(settings: &EmbeddingSettings) -> Self {
        let embedded = TextEmbedded::new(&settings.model_path).expect("Failed to initialize TextEmbed model");

        Self {
            text_embedded: Arc::new(embedded),
        }
    }
}

#[tonic::async_trait]
impl embedding_server::Embedding for TextEmbeddingController {
    async fn embed_text(
        &self,
        request: Request<TextEmbeddingRequest>,
    ) -> Result<Response<TextEmbeddingResponse>, Status> {
        let request = request.into_inner();

        log::info!("Received unary request[id: {}]", request.id);

        let embedding = self
            .text_embedded
            .embed_one(&request.text)
            .map_err(|err| Status::internal(err.to_string()))?;

        Ok(Response::new(TextEmbeddingResponse {
            id: request.id,
            embedding,
        }))
    }

    type EmbedTextStreamStream = Pin<Box<dyn Stream<Item = Result<TextEmbeddingResponse, Status>> + Send>>;

    async fn embed_text_stream(
        &self,
        request: Request<Streaming<TextEmbeddingRequest>>,
    ) -> Result<Response<Self::EmbedTextStreamStream>, Status> {
        let stream = request.into_inner();
        let embedder = self.text_embedded.clone();

        let response = stream.map(move |request| {
            let (id, text) = match request? {
                req if req.text.is_empty() => return Err(Status::invalid_argument("Text is required")),
                req => (req.id, req.text),
            };

            let embedding = embedder
                .embed_one(&text)
                .map_err(|err| Status::internal(err.to_string()))?;

            Ok(TextEmbeddingResponse { id, embedding })
        });

        Ok(Response::new(Box::pin(response) as Self::EmbedTextStreamStream))
    }
}
