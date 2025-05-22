## Norph

---

### Описание

Пример реализации gRPC-сервера на Rust, предназначенного для преобразования текста в векторные представления (эмбеддинги). 
Этот сервис может являться частью RAG-системы, поисковой системы, систем векторного анализа, и т.д. 

Используется предварительно загруженная локальная модель (без подключения к внешним AI API).
Вычисление (инференс) производится на GPU (при доступности GPU провайдеров на вашем устройстве). В противном случае - на CPU.

### Модели

Например, можно использовать следующие модели (encoder):

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

Для использования, вам необходимо скачать из репозитория следующие файлы: `model.onnx`, `config.json`, `tokenizer.json`, `tokenizer_config.json`, `special_tokens_map.json`.
Далее их следует положить в директорию `./model` проекта.

### Конфигурация

В `config.yaml` можно устанавливать значение для полей:

- `server.host` - хост для запуска gRPC сервера.
- `server.port` - порт для запуска gRPC сервера.
- `logging.log_level` - уровень детализации логов.
- `embeddings.model_path` - путь к директории с файлами модели.
- `embeddings.max_text_size` - максимальный размер текста.

### Использование

Для отправки запроса на сервер возьмите `text_embeddings.proto` (из директории `./proto`), и используйте в своем клиенте.
Проверить работоспособность можно например через `Postman`.

Cтруктура запроса для rpc `EmbedText`:

```Json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "text": "Hello, how are you?"
}
```

В результате распознавания сервер вернёт JSON вида:

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

Cтруктура потока запросов для rpc `EmbedTextStream`:

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

В результате распознавания сервер вернёт поток JSON вида:

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

### Локальный запуск

1) Для установки `Rust` на Unix-подобные системы (MacOS, Linux, ...) - запускаем в терминале команду.
   По окончании загрузки вы получите последнюю стабильную версию Rust для вашей платформы, а так же последнюю версию Cargo.

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2) Для проверки выполните следующую команду в терминале.

```shell
cargo --version
```

3) Открываем проект, и запускаем команды.

Проверяет код на возможность компиляции (без запуска).
```shell
cargo check
```

Сборка + запуск проекта (в режиме релиза с оптимизациями).
```shell
cargo run --release
```

UDP: Если вдруг у вас Windows, посмотрите [Инструкцию тут](https://forge.rust-lang.org/infra/other-installation-methods.html)
