## Norph

---

[Russian version](https://github.com/Ave-Sergeev/Norph/blob/main/README.ru.md)

### Description

An example of gRPC server implementation in Rust, designed to convert text into vector representations (embeddings).
This service can be part of a RAG system, search engine, vector analysis systems, etc.

Preloaded local model is used (without connection to external AI APIs).
Computation (inference) is performed on GPU (if GPU providers are available on your device). Otherwise - on CPU.

### Models

For example, the following models can be used (encoder):

- RU
  - https://huggingface.co/cointegrated/rubert-tiny2 (~ 120 MB)
  - https://huggingface.co/intfloat/multilingual-e5-small (~ 470 MB)
  - https://huggingface.co/sentence-transformers/paraphrase-multilingual-mpnet-base-v2 (~ 1.1 GB)
  - https://huggingface.co/BAAI/bge-m3 (~ 2.2 GB)

- EN
  - https://huggingface.co/sentence-transformers/all-MiniLM-L12-v2 (~ 135 MB)
  - https://huggingface.co/sentence-transformers/all-mpnet-base-v2 (~ 437 MB)
  - https://huggingface.co/BAAI/bge-large-en-v1.5 (~ 1.3 GB)
  - https://huggingface.co/Alibaba-NLP/gte-large-en-v1.5 (~ 1.7 GB)

To use it, you need to download the following files from the repository: `model.onnx`, `config.json`, `tokenizer.json`, `tokenizer_config.json`, `special_tokens_map.json`.
Then they should be placed in the `./model` directory of the project.

### Configuration

In `config.yaml` you can set the value for the fields:

- `server.host` - host to start the gRPC server.
- `server.port` - port to start the gRPC server.
- `logging.log_level` - logging level of detail.
- `embeddings.model_path` - path to the directory with model files.
- `embeddings.max_text_size` - maximum text size.

### Usage

To send a request to the server, take `text_embeddings.proto` (from the `./proto` directory), and use it in your client.
You can check if it works, for example, via `Postman`.

Query structure for rpc `EmbedText`:

```Json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "text": "Hello, how are you?"
}
```

As a result of the recognition, the server will return JSON of the form:

```Json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "embedding": [
    -0.028818072751164436,
    0.04423859715461731,
    -0.004005502909421921,
    "...",
    -0.044924844056367874,
    -0.02066687121987343
  ]
}
```

Query stream structure for rpc `EmbedTextStream`:

```Json
[
  {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "text": "Hello"
  },
  {
    "id": "123e4567-e89b-12d3-a456-426614174001",
    "text": "how are you?"
  },
  "..."
]
```

As a result of the recognition, the server will return a JSON stream of the form:

```Json
[
  {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "embedding": [
      -0.028818072751164436,
      "...",
      -0.02066687121987343
    ]
  },
  {
    "id": "123e4567-e89b-12d3-a456-426614174001",
    "embedding": [
      -0.0586518072751624839,
      "...",
      -0.07025247164987985
    ]
  },
  "..."
]
```

### Local startup

1) To install `Rust` on Unix-like systems (MacOS, Linux, ...) - run the command in the terminal.
   After the download is complete, you will get the latest stable version of Rust for your platform, as well as the latest version of Cargo.

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2) Run the following command in the terminal to verify.

```shell
cargo --version
```

3) Open the project and run the commands.

Check the code to see if it can be compiled (without running it).
```shell
cargo check
```

Build + run the project (in release mode with optimizations).
```shell
cargo run --release
```

UDP: If you have Windows, see [Instructions here](https://forge.rust-lang.org/infra/other-installation-methods.html).

### P.S.

Don't forget to leave a ⭐ if you found [this project](https://github.com/Ave-Sergeev/Tictonix) useful.
