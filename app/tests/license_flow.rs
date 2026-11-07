//! Activating a Lemon Squeezy key through the signer, against a stand-in
//! signer on localhost: the activation is kept, the same key on the same
//! computer reuses it, a full key is refused, and Remove frees it.

mod common;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

use arkyv_app::license::{self, ActivateError};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use common::Harness;
use ed25519_dalek::{Signer, SigningKey};

/// The requests the stand-in signer saw: (path, body).
type Seen = Arc<Mutex<Vec<(String, String)>>>;

/// Answers each request with the next canned (status, body), and records
/// the requests' paths and bodies.
fn signer(answers: Vec<(u16, String)>) -> (String, Seen) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let seen = Arc::new(Mutex::new(Vec::new()));
    let log = seen.clone();
    std::thread::spawn(move || {
        for ((status, body), stream) in answers.into_iter().zip(listener.incoming()) {
            let mut stream = stream.unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            let path = line.split_whitespace().nth(1).unwrap_or("").to_owned();
            let mut length = 0;
            loop {
                let mut header = String::new();
                reader.read_line(&mut header).unwrap();
                if header.trim().is_empty() {
                    break;
                }
                if let Some(v) = header.to_ascii_lowercase().strip_prefix("content-length:") {
                    length = v.trim().parse().unwrap();
                }
            }
            let mut request = vec![0; length];
            reader.read_exact(&mut request).unwrap();
            log.lock()
                .unwrap()
                .push((path, String::from_utf8(request).unwrap()));
            write!(
                stream,
                "HTTP/1.1 {status} X\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
                body.len()
            )
            .unwrap();
        }
    });
    (url, seen)
}

fn signed(key: &SigningKey, payload: &str) -> String {
    let mut bytes = payload.as_bytes().to_vec();
    bytes.extend_from_slice(&key.sign(payload.as_bytes()).to_bytes());
    format!("ARKYV-PRO-{}", URL_SAFE_NO_PAD.encode(bytes))
}

#[test]
fn license_flow() {
    let _h = Harness::new(); // a fresh settings folder
    let key = SigningKey::from_bytes(&[9; 32]);
    let public = key.verifying_key().to_bytes();
    let pro = signed(&key, "1|ls:777|2026-10-31");
    let ok = |instance: &str| {
        (
            200,
            format!(r#"{{"key":"{pro}","instance_id":"{instance}"}}"#),
        )
    };
    let (url, seen) = signer(vec![
        ok("i1"),
        ok("i1"),
        (409, r#"{"error":"limit"}"#.into()),
        (200, "{}".into()),
    ]);
    let body = |i: usize| -> serde_json::Value {
        serde_json::from_str(&seen.lock().unwrap()[i].1).unwrap()
    };

    // First activation on this computer: no instance yet; kept afterwards.
    let license = license::activate_store_key(&url, "GOOD", &public).unwrap();
    assert_eq!(license.id, "ls:777");
    assert_eq!(body(0)["license_key"], "GOOD");
    assert!(body(0)["instance_id"].is_null());
    assert!(
        body(0)["instance_name"]
            .as_str()
            .unwrap()
            .starts_with("Arkyv on ")
    );

    // The same key again on this computer reuses its activation.
    license::activate_store_key(&url, "GOOD", &public).unwrap();
    assert_eq!(body(1)["instance_id"], "i1");

    // Another key whose five computers are taken.
    assert_eq!(
        license::activate_store_key(&url, "FULL", &public),
        Err(ActivateError::Limit)
    );
    assert!(
        body(2)["instance_id"].is_null(),
        "not this key's activation"
    );

    // Freeing an activation.
    assert!(license::deactivate(&format!("{url}/"), "GOOD", "i1"));
    let (path, _) = seen.lock().unwrap()[3].clone();
    assert_eq!(path, "/deactivate");
    assert_eq!(body(3)["instance_id"], "i1");
}
