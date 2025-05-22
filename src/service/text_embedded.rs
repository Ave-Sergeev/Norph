use crate::error::service_error::ServiceError;
use anyhow::{Error, Result};
use fastembed::{InitOptionsUserDefined, TextEmbedding, TokenizerFiles, UserDefinedEmbeddingModel};
use ort::execution_providers::{
    CPUExecutionProvider, CUDAExecutionProvider, CoreMLExecutionProvider, DirectMLExecutionProvider, ExecutionProvider,
    ExecutionProviderDispatch, OpenVINOExecutionProvider, TensorRTExecutionProvider,
};
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

        let execution_providers = Self::get_execution_providers();

        let options = InitOptionsUserDefined::new()
            .with_max_length(1024)
            .with_execution_providers(execution_providers);

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

    /// Returns a vector of execution providers, trying various GPU first and falling back to CPU
    fn get_execution_providers() -> Vec<ExecutionProviderDispatch> {
        let cpu_ep = CPUExecutionProvider::default();
        let cuda_ep = CUDAExecutionProvider::default();
        let core_ml_ep = CoreMLExecutionProvider::default();
        let direct_ml_ep = DirectMLExecutionProvider::default();
        let tensor_rt_ep = TensorRTExecutionProvider::default();
        let open_vino_ep = OpenVINOExecutionProvider::default();

        let mut execution_providers = vec![];

        if let Ok(true) = cuda_ep.is_available() {
            log::info!("Using CUDA execution provider");
            execution_providers.push(fastembed::ExecutionProviderDispatch::from(cuda_ep));
        } else if let Ok(true) = core_ml_ep.is_available() {
            log::info!("Using CoreML execution provider");
            execution_providers.push(fastembed::ExecutionProviderDispatch::from(core_ml_ep));
        } else if let Ok(true) = tensor_rt_ep.is_available() {
            log::info!("Using TensorRT execution provider");
            execution_providers.push(fastembed::ExecutionProviderDispatch::from(tensor_rt_ep));
        } else if let Ok(true) = direct_ml_ep.is_available() {
            log::info!("Using DirectML execution provider");
            execution_providers.push(fastembed::ExecutionProviderDispatch::from(direct_ml_ep));
        } else if let Ok(true) = open_vino_ep.is_available() {
            log::info!("Using OpenVINO execution provider");
            execution_providers.push(fastembed::ExecutionProviderDispatch::from(open_vino_ep));
        } else {
            log::info!("Using CPU execution provider");
            execution_providers.push(fastembed::ExecutionProviderDispatch::from(cpu_ep));
        }

        execution_providers
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
