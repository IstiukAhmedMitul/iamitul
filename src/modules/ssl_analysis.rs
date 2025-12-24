use openssl::ssl::{SslConnector, SslMethod};
use std::net::TcpStream;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct SslInfo {
    pub issuer: String,
    pub subject: String,
    pub valid_from: String,
    pub valid_to: String,
    pub fingerprint: String,
}

pub async fn analyze_ssl(target: &str) -> Option<SslInfo> {
    let connector = SslConnector::builder(SslMethod::tls()).ok()?.build();
    let stream = TcpStream::connect(format!("{}:443", target)).ok()?;
    let ssl_stream = connector.connect(target, stream).ok()?;
    
    let cert = ssl_stream.ssl().peer_certificate()?;
    
    // Get certificate information
    let issuer = get_x509_name_string(&cert.issuer_name());
    let subject = get_x509_name_string(&cert.subject_name());
    let not_before = cert.not_before().to_string();
    let not_after = cert.not_after().to_string();
    
    // Calculate fingerprint
    let fingerprint = match cert.digest(openssl::hash::MessageDigest::sha256()) {
        Ok(fingerprint) => hex::encode(fingerprint.as_ref()),
        Err(_) => "Unknown".to_string(),
    };
    
    Some(SslInfo {
        issuer,
        subject,
        valid_from: not_before,
        valid_to: not_after,
        fingerprint,
    })
}

fn get_x509_name_string(name: &openssl::x509::X509NameRef) -> String {
    name.entries()
        .map(|entry| {
            let obj = entry.object().to_string();
            let value = entry.data().as_utf8()
                .map(|s| s.to_string())
                .unwrap_or_else(|_| "INVALID".to_string());
            format!("{}={}", obj, value)
        })
        .collect::<Vec<_>>()
        .join(", ")
}
