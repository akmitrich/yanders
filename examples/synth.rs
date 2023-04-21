use std::io::Write;
use tokio_stream::StreamExt;

use yanders::tts;

#[tokio::main]
async fn main() {
    let tls = yanders::tls_config();
    synthesize(
        "Привет, Яндерс!\nДанный пример создан для того, чтобы продемонстрировать возможности Rustв деле освоения Яндекс Speechkit API.", 
        include_str!(".secret"), 
        tls
    ).await;
}

async fn synthesize(text: &str, token: &str, tls: tonic::transport::ClientTlsConfig) {
    let req = prepare_request(text, token);
    let channel = connect_to_yandex(tls).await;
    let mut stub = tts::synthesizer_client::SynthesizerClient::new(channel);
    match stub.utterance_synthesis(req).await {
        Ok(resp) => {
            streaming_response(resp).await;
        }
        Err(status) => println!("Error in utterance_synthesis: {}", status),
    }
}

fn prepare_request(text: &str, token: &str) -> tonic::Request<tts::UtteranceSynthesisRequest> {
    let mut req = tonic::Request::new(tts::UtteranceSynthesisRequest {
        utterance: Some(tts::utterance_synthesis_request::Utterance::Text(
            text.into(),
        )),
        output_audio_spec: Some(tts::AudioFormatOptions {
            audio_format: Some(tts::audio_format_options::AudioFormat::ContainerAudio(
                tts::ContainerAudio {
                    container_audio_type: tts::container_audio::ContainerAudioType::Wav as _,
                },
            )),
        }),
        loudness_normalization_type:
            tts::utterance_synthesis_request::LoudnessNormalizationType::Lufs as _,
        ..Default::default()
    });
    req.metadata_mut().insert(
        "authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );
    req.metadata_mut()
        .insert("x-folder-id", include_str!(".folder-id").parse().unwrap());
    req
}

async fn connect_to_yandex(tls: tonic::transport::ClientTlsConfig) -> tonic::transport::Channel {
    tonic::transport::Channel::from_static("http://tts.api.cloud.yandex.net:443")
        .tls_config(tls)
        .expect("TLS config must pass")
        .connect()
        .await
        .expect("tonic Channel")
}

async fn streaming_response(
    resp: tonic::Response<tonic::codec::Streaming<tts::UtteranceSynthesisResponse>>,
) {
    println!(
        "Meta data keys: {:?}",
        resp.metadata().keys().collect::<Vec<_>>()
    );
    let server_trace_id = resp.metadata().get("x-server-trace-id");
    println!("Server trace id = {:?}", server_trace_id);
    let mut resp = resp.into_inner();
    let wav = std::fs::File::create("hello_yandex.wav").expect("create audio file");
    let mut writer = std::io::BufWriter::new(wav);
    while let Some(chunk) = resp.next().await {
        match chunk {
            Ok(resp) => match resp.audio_chunk {
                Some(chunk) => {
                    println!("Audio chunk arrived. {} bytes.", chunk.data.len());
                    writer
                        .write_all(&chunk.data)
                        .expect("cannot write to audio file");
                }
                None => println!("Audio chunk is empty."),
            },
            Err(status) => println!("Error: {}", status),
        }
    }
}
