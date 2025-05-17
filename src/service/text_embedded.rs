use crate::error::service_error::ServiceError;
use anyhow::{Error, Result};
use fastembed::{InitOptionsUserDefined, TextEmbedding, TokenizerFiles, UserDefinedEmbeddingModel};
use std::path::Path;

pub struct TextEmbedded {
    pub model: TextEmbedding,
}

impl TextEmbedded {
    pub fn new<P>(model_path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let model_filepath = model_path.as_ref();

        if !model_filepath.exists() {
            return Err(Error::from(ServiceError::ModelError(format!(
                "Model file not found at: {}",
                model_filepath.display()
            ))));
        }

        let onnx_file_path = model_filepath.join("model.onnx");
        let onnx_model_contents = std::fs::read(onnx_file_path)?;

        let config_file = std::fs::read(model_filepath.join("config.json"))?;
        let tokenizer_file = std::fs::read(model_filepath.join("tokenizer.json"))?;
        let tokenizer_config_file = std::fs::read(model_filepath.join("tokenizer_config.json"))?;
        let special_tokens_map_file = std::fs::read(model_filepath.join("special_tokens_map.json"))?;

        let tokenizer_files = TokenizerFiles {
            tokenizer_file,
            config_file,
            special_tokens_map_file,
            tokenizer_config_file,
        };

        let options = InitOptionsUserDefined::new().with_max_length(1024);
        let udem = UserDefinedEmbeddingModel::new(onnx_model_contents, tokenizer_files);

        let model = TextEmbedding::try_new_from_user_defined(udem, options)
            .map_err(|err| ServiceError::ModelError(format!("Failed to create model: {err}")))?;

        Ok(Self { model })
    }

    /// Make a service for a single string
    pub fn embed_one(&self, document: &str) -> Result<Vec<f32>, Error> {
        let embeddings = self
            .model
            .embed(vec![document], None)
            .map_err(|err| ServiceError::EmbeddingError(format!("Failed to embed text: {err}")))?;

        embeddings
            .into_iter()
            .next()
            .ok_or(Error::from(ServiceError::EmbeddingError("Failed to get service from text".to_string())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_embedded_success() -> Result<(), Error> {
        let model_path = Path::new("model");
        let text_embedded = TextEmbedded::new(model_path)?;

        let documents = vec![
            "Fish, dog, cat",
            "Look at my hat!",
            "Tiger and bear,",
            "Touch your hair.",
        ];

        let embeddings = text_embedded
            .model
            .embed(documents.clone(), None)
            .map_err(|err| ServiceError::EmbeddingError(format!("Failed to embed text: {err}")))?;

        assert_eq!(embeddings.len(), documents.len());

        Ok(())
    }
}
