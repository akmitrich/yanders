fn main() {
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile(
            &[
                "google/api/http.proto",
                "google/api/annotations.proto",
                "google/rpc/status.proto",
            ],
            &["../cloudapi/third_party/googleapis/"],
        )
        .unwrap();
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile(
            &[
                "yandex/cloud/api/operation.proto",
                "yandex/cloud/operation/operation.proto",
                "yandex/cloud/ai/tts/v3/tts_service.proto",
                "yandex/cloud/ai/tts/v3/tts.proto",
                "yandex/cloud/ai/stt/v3/stt_service.proto",
                "yandex/cloud/ai/stt/v3/stt.proto",
            ],
            &["../cloudapi/", "../cloudapi/third_party/googleapis/"],
        )
        .unwrap();
}
