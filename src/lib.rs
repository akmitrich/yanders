pub mod tts {
    tonic::include_proto!("speechkit.tts.v3");
}

pub fn tls_config() -> tonic::transport::ClientTlsConfig {
    const CERT_PATH: &str = "/etc/ssl/certs/GlobalSign_Root_CA.pem";
    let pem = std::fs::read(CERT_PATH).expect("read the cert file");
    let cert = tonic::transport::Certificate::from_pem(pem);
    tonic::transport::ClientTlsConfig::new().ca_certificate(cert)
}
