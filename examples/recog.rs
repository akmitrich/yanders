use std::io::Read;

use yanders::stt;

#[tokio::main]
async fn main() {
    let tls = yanders::tls_config();
    recognize("num.wav", include_str!(".secret"), tls).await;
}

async fn recognize(
    audio_path: impl AsRef<std::path::Path>,
    token: &str,
    tls: tonic::transport::ClientTlsConfig,
) {
    let recognize_options = stt::StreamingOptions {
        recognition_model: Some(stt::RecognitionModelOptions {
            audio_format: Some(stt::AudioFormatOptions {
                audio_format: Some(stt::audio_format_options::AudioFormat::RawAudio(
                    stt::RawAudio {
                        audio_encoding: stt::raw_audio::AudioEncoding::Linear16Pcm as _,
                        sample_rate_hertz: 8000,
                        audio_channel_count: 1,
                    },
                )),
            }),
            text_normalization: Some(stt::TextNormalizationOptions {
                text_normalization: stt::text_normalization_options::TextNormalization::Enabled
                    as _,
                profanity_filter: true,
                literature_text: false,
                ..Default::default()
            }),
            language_restriction: Some(stt::LanguageRestrictionOptions {
                restriction_type:
                    stt::language_restriction_options::LanguageRestrictionType::Whitelist as _,
                language_code: vec!["ru-RU".into()],
            }),
            audio_processing_type: stt::recognition_model_options::AudioProcessingType::RealTime
                as _,
            ..Default::default()
        }),
        ..Default::default()
    };
    let events = create_events(recognize_options, audio_path);
    let mut req = tonic::Request::new(futures_util::stream::iter(events));
    req.metadata_mut().insert(
        "authorization",
        format!("Bearer {}", token.trim()).parse().unwrap(),
    );
    req.metadata_mut()
        .insert("x-folder-id", include_str!(".folder-id").parse().unwrap());
    let mut stub =
        stt::recognizer_client::RecognizerClient::connect("http://stt.api.cloud.yandex.net:443")
            .await
            .expect("Need connection to yandex");
    match stub.recognize_streaming(req).await {
        Ok(resp) => todo!(),
        Err(status) => println!("Error in recognize_streaming\n{}", status),
    }
}

fn create_events(
    options: stt::StreamingOptions,
    audio_path: impl AsRef<std::path::Path>,
) -> Vec<stt::StreamingRequest> {
    let mut events = vec![stt::StreamingRequest {
        event: Some(stt::streaming_request::Event::SessionOptions(options)),
    }];
    let mut input = std::io::BufReader::new(
        std::fs::File::open(audio_path.as_ref())
            .expect(&format!("Open file {:?}", audio_path.as_ref())),
    )
    .bytes()
    .filter_map(|x| x.ok());
    let mut complete = false;
    loop {
        let mut chunk = stt::AudioChunk {
            data: Vec::with_capacity(4096),
        };
        for _ in 0..4096 {
            match input.next() {
                Some(b) => chunk.data.push(b),
                None => {
                    complete = true;
                    break;
                }
            }
        }
        events.push(stt::StreamingRequest {
            event: Some(stt::streaming_request::Event::Chunk(chunk)),
        });
        if complete {
            break;
        }
    }
    events
}
