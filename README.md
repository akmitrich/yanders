# Rust-библиотека для работы с Яндекс Speechkit API
## Примеры находятся в соответствующих ветках
См. branches.

## Сборка библиотеки
1. Склонируйте репозиторий Yandex Cloud API:
```bash
git clone https://github.com/yandex-cloud/cloudapi
```
2. Склонируйте ```yanders``` рядом с cloudapi. То есть в той же директории, из которой выполняли п. 1, выполните:
```bash
git clone https://github.com/akmitrich/yanders.git
```
3. Выполните
```bash
cargo build
```

## Выполнение примеров
1. Соберите библиотеку, как описано выше.
2. Выберите нужный branch с помощью git или другим привычным для вас способом.
3. Создайте аккаунт для работы с API SpeechKit.
4. Проверьте права вашего аккаунта на доступ к API v3 синтеза. При необходимости запишите ваш ```folder-id``` в файл ```.folder-id```.
5. Получите IAM-токен для сервисного аккаунта. Запишите его в файл ```.secret```.
6. Выполните
```bash
cargo run --example <example>.rs
```
## TLS-соединение
В tonic для открытия безопасного соединения нужно явно указать root certificate. Подробнее https://jessitron.com/2022/11/02/make-https-work-on-grpc-in-rust-load-a-root-certificate-into-the-tls-config/

В ```lib.rs``` функция ```fn tls_config()``` для корректного взаимодействия с grpc-сервером Яндекс может потребовать правки пути к файлу с сертификатом ```CERT_PATH```.