//! Arkyv Pro license (M9). Honor system: all code is public, and the key
//! only says who paid. A key is a payload signed with Arkyv's Ed25519
//! private key (kept offline and in the signer, never in this repository);
//! the app checks it with the public key below, offline.
//!
//! Buyers get a Lemon Squeezy license key. `activate` exchanges it once with
//! the signer (`signer/`) for a signed key; after that nothing goes over the
//! network. A signed key can also be pasted directly (sponsors, keys made
//! with `signer/sign.mjs`).
//!
//! Key: `ARKYV-PRO-` + base64url (no padding) of the payload followed by the
//! 64-byte signature. Payload: `1|<id>|<issued, YYYY-MM-DD>`, where the id
//! names the purchase (`ls:<order>`, `gh:<sponsor>`) and nothing else.

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::{Signature, VerifyingKey};

use crate::settings::Settings;

pub const PREFIX: &str = "ARKYV-PRO-";
/// Keys signed while the app was called Crate (before 0.2.36): the same
/// signature, an older prefix. They keep working.
const OLD_PREFIX: &str = "CRATE-PRO-";

/// Whether `key` looks like a signed key (either prefix), not a store key.
pub fn is_signed_key(key: &str) -> bool {
    let key = key.trim();
    key.starts_with(PREFIX) || key.starts_with(OLD_PREFIX)
}
const SETTING: &str = "pro.key";

/// The public half of ~/crate-license/license-ed25519.pem.
const PUBLIC_KEY: [u8; 32] = [
    0xb3, 0xb7, 0x97, 0x63, 0x69, 0xd6, 0x49, 0x44, 0x79, 0x00, 0xd5, 0xf4, 0x39, 0xb3, 0x6e, 0xac,
    0x5b, 0x74, 0x46, 0x1a, 0x8c, 0x83, 0x62, 0x4f, 0x2e, 0x1d, 0x19, 0x65, 0x4d, 0x73, 0xed, 0xac,
];

/// The signer's address, set at build time by the release workflow (a
/// repository variable). Builds without it can't activate a store key, but
/// still take signed keys.
const ACTIVATE_URL: Option<&str> = option_env!("ARKYV_LICENSE_URL");

/// A valid key's contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct License {
    pub id: String,
    pub issued: String,
    /// The key as entered, to save.
    pub key: String,
}

impl License {
    /// The issue date as a time (midnight UTC), for showing it.
    pub fn issued_time(&self) -> Option<std::time::SystemTime> {
        let mut parts = self.issued.split('-').map(|p| p.parse::<i64>().ok());
        let (Some(Some(y)), Some(Some(m)), Some(Some(d))) =
            (parts.next(), parts.next(), parts.next())
        else {
            return None;
        };
        if !(1..=12).contains(&m) || !(1..=31).contains(&d) {
            return None;
        }
        // Howard Hinnant's days_from_civil.
        let y = if m <= 2 { y - 1 } else { y };
        let era = y.div_euclid(400);
        let yoe = y - era * 400;
        let doy = (153 * (m + if m > 2 { -3 } else { 9 }) + 2) / 5 + d - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        let days = u64::try_from(era * 146_097 + doe - 719_468).ok()?;
        Some(std::time::UNIX_EPOCH + std::time::Duration::from_secs(days * 86_400))
    }
}

/// Checks a signed key. `None` for anything that isn't one Arkyv signed.
pub fn verify(key: &str) -> Option<License> {
    verify_with(key, &PUBLIC_KEY)
}

fn verify_with(key: &str, public: &[u8; 32]) -> Option<License> {
    let key = key.trim();
    let bytes = URL_SAFE_NO_PAD
        .decode(
            key.strip_prefix(PREFIX)
                .or_else(|| key.strip_prefix(OLD_PREFIX))?,
        )
        .ok()?;
    let split = bytes.len().checked_sub(Signature::BYTE_SIZE)?;
    let (payload, sig) = bytes.split_at(split);
    let sig = Signature::from_slice(sig).ok()?;
    VerifyingKey::from_bytes(public)
        .ok()?
        .verify_strict(payload, &sig)
        .ok()?;
    let mut parts = std::str::from_utf8(payload).ok()?.split('|');
    let (Some("1"), Some(id), Some(issued), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return None;
    };
    Some(License {
        id: id.to_owned(),
        issued: issued.to_owned(),
        key: key.to_owned(),
    })
}

/// The saved license, if it's still valid.
pub fn current() -> Option<License> {
    from_settings(&Settings::load())
}

pub fn from_settings(s: &Settings) -> Option<License> {
    verify(&s.get::<String>(SETTING)?)
}

pub fn is_pro() -> bool {
    current().is_some()
}

/// Saves a valid key; the caller has verified it.
pub fn save(license: &License) {
    let mut s = Settings::load();
    s.set(SETTING, &license.key);
    s.save();
}

// A key activated through the signer also keeps the Lemon Squeezy key and
// this computer's activation, to free it again on Remove license.
const STORE_KEY: &str = "pro.store_key";
const INSTANCE: &str = "pro.instance";
// Activations Remove license couldn't free (offline): "key\tinstance" lines,
// tried again when Preferences opens.
const PENDING: &str = "pro.deactivate";

/// Forgets the key (Preferences → Remove license), and frees this computer's
/// activation so it doesn't count toward the key's limit. Blocks on the
/// network for a store key; call it off the UI thread.
pub fn remove() {
    let mut s = Settings::load();
    let store_key = s.get::<String>(STORE_KEY).unwrap_or_default();
    let instance = s.get::<String>(INSTANCE).unwrap_or_default();
    s.set(SETTING, "");
    s.set(STORE_KEY, "");
    s.set(INSTANCE, "");
    s.save();
    if !store_key.is_empty() && !instance.is_empty() {
        let freed = signer_url().is_some_and(|url| deactivate(url, &store_key, &instance));
        if !freed {
            let mut s = Settings::load();
            let mut pending = s.get::<String>(PENDING).unwrap_or_default();
            pending.push_str(&format!("{store_key}\t{instance}\n"));
            s.set(PENDING, pending);
            s.save();
        }
    }
}

/// Frees activations an earlier Remove license couldn't. Blocks; call it off
/// the UI thread.
pub fn retry_deactivations() {
    let Some(url) = signer_url() else { return };
    let pending = Settings::load().get::<String>(PENDING).unwrap_or_default();
    if pending.is_empty() {
        return;
    }
    let left: String = pending
        .lines()
        .filter(|line| match line.split_once('\t') {
            Some((key, instance)) => !deactivate(url, key, instance),
            None => false,
        })
        .map(|line| format!("{line}\n"))
        .collect();
    let mut s = Settings::load();
    s.set(PENDING, left);
    s.save();
}

/// Why a key wasn't accepted. The UI turns these into sentences.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActivateError {
    /// Lemon Squeezy doesn't know the key, or it was refunded or disabled.
    Rejected,
    /// The key is already active on as many computers as it allows (5).
    Limit,
    /// The signer couldn't be reached.
    Network(String),
    /// This build has no signer address.
    Unavailable,
}

fn signer_url() -> Option<&'static str> {
    ACTIVATE_URL.filter(|u| !u.is_empty())
}

/// Enters a key: a signed key is checked here; anything else is taken as a
/// Lemon Squeezy key and sent to the signer. Blocks; call it off the UI
/// thread. Saves the license on success.
pub fn activate(entered: &str) -> Result<License, ActivateError> {
    let entered = entered.trim();
    if is_signed_key(entered) {
        let license = verify(entered).ok_or(ActivateError::Rejected)?;
        save(&license);
        return Ok(license);
    }
    if entered.is_empty() {
        return Err(ActivateError::Rejected);
    }
    let url = signer_url().ok_or(ActivateError::Unavailable)?;
    activate_store_key(url, entered, &PUBLIC_KEY)
}

/// `activate` for a store key, against a given signer and public key (tests).
#[doc(hidden)]
pub fn activate_store_key(
    url: &str,
    store_key: &str,
    public: &[u8; 32],
) -> Result<License, ActivateError> {
    // The same key entered again on this computer reuses its activation.
    let s = Settings::load();
    let instance = (s.get::<String>(STORE_KEY).as_deref() == Some(store_key))
        .then(|| s.get::<String>(INSTANCE))
        .flatten()
        .filter(|i| !i.is_empty());
    let (signed, instance) = exchange(url, store_key, instance.as_deref())?;
    let license = verify_with(&signed, public).ok_or(ActivateError::Rejected)?;
    let mut s = Settings::load();
    s.set(SETTING, &license.key);
    s.set(STORE_KEY, store_key);
    s.set(INSTANCE, instance);
    s.save();
    Ok(license)
}

fn agent() -> ureq::Agent {
    ureq::Agent::config_builder()
        .timeout_global(Some(std::time::Duration::from_secs(20)))
        .http_status_as_error(false)
        .build()
        .new_agent()
}

/// What Lemon Squeezy shows for an activation: no name, host or user, only
/// the system.
fn instance_name() -> String {
    format!("Arkyv on {}", std::env::consts::OS)
}

/// POST {license_key, instance_id?, instance_name} → 200 {key, instance_id};
/// 409: the key's computers are used up; other 4xx: not a key we sold.
fn exchange(
    url: &str,
    store_key: &str,
    instance: Option<&str>,
) -> Result<(String, String), ActivateError> {
    let body = serde_json::json!({
        "license_key": store_key,
        "instance_id": instance,
        "instance_name": instance_name(),
    });
    let mut response = agent()
        .post(url)
        .send_json(&body)
        .map_err(|e| ActivateError::Network(e.to_string()))?;
    let status = response.status().as_u16();
    if status == 409 {
        return Err(ActivateError::Limit);
    }
    if (400..500).contains(&status) {
        return Err(ActivateError::Rejected);
    }
    if status != 200 {
        return Err(ActivateError::Network(format!("HTTP {status}")));
    }
    let reply: serde_json::Value = response
        .body_mut()
        .read_json()
        .map_err(|e| ActivateError::Network(e.to_string()))?;
    let field = |name: &str| match &reply[name] {
        serde_json::Value::String(v) => Some(v.clone()),
        serde_json::Value::Number(v) => Some(v.to_string()),
        _ => None,
    };
    match (field("key"), field("instance_id")) {
        (Some(key), Some(instance)) => Ok((key, instance)),
        _ => Err(ActivateError::Network("no key in the reply".into())),
    }
}

/// POST {license_key, instance_id} to `<signer>/deactivate`. True once the
/// activation is gone (also when the signer says it never existed).
#[doc(hidden)]
pub fn deactivate(url: &str, store_key: &str, instance: &str) -> bool {
    let body = serde_json::json!({ "license_key": store_key, "instance_id": instance });
    agent()
        .post(&format!("{}/deactivate", url.trim_end_matches('/')))
        .send_json(&body)
        .is_ok_and(|r| r.status().as_u16() < 500)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

    fn sign(signing: &SigningKey, payload: &str) -> String {
        let mut bytes = payload.as_bytes().to_vec();
        bytes.extend_from_slice(&signing.sign(payload.as_bytes()).to_bytes());
        format!("{PREFIX}{}", URL_SAFE_NO_PAD.encode(bytes))
    }

    fn test_key() -> SigningKey {
        SigningKey::from_bytes(&[7; 32])
    }

    #[test]
    fn accepts_a_signed_key() {
        let signing = test_key();
        let key = sign(&signing, "1|ls:12345|2026-10-31");
        let license =
            verify_with(&format!("  {key}\n"), &signing.verifying_key().to_bytes()).expect("valid");
        assert_eq!(license.id, "ls:12345");
        assert_eq!(license.issued, "2026-10-31");
        assert_eq!(license.key, key);
        let secs = license
            .issued_time()
            .unwrap()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        assert_eq!(secs, 1_793_404_800); // 2026-10-31T00:00:00Z
    }

    #[test]
    fn rejects_other_keys() {
        let signing = test_key();
        let public = signing.verifying_key().to_bytes();
        let good = sign(&signing, "1|ls:1|2026-10-31");
        // Signed by someone else.
        let other = sign(&SigningKey::from_bytes(&[8; 32]), "1|ls:1|2026-10-31");
        assert!(verify_with(&other, &public).is_none());
        // One character changed.
        let mut tampered = good.clone().into_bytes();
        let i = PREFIX.len() + 3;
        tampered[i] = if tampered[i] == b'A' { b'B' } else { b'A' };
        assert!(verify_with(std::str::from_utf8(&tampered).unwrap(), &public).is_none());
        // Unknown version, wrong shape, garbage.
        assert!(verify_with(&sign(&signing, "2|ls:1|2026-10-31"), &public).is_none());
        assert!(verify_with(&sign(&signing, "1|ls:1"), &public).is_none());
        assert!(verify_with("ARKYV-PRO-", &public).is_none());
        // A key from before the rename: the same signature, the old prefix.
        let old = good.replacen(PREFIX, OLD_PREFIX, 1);
        assert!(verify_with(&old, &public).is_some());
        assert!(is_signed_key(&old) && is_signed_key(&good) && !is_signed_key("ABCD-1234"));
        assert!(verify_with("ABCD-1234", &public).is_none());
        // The real public key doesn't take a test key.
        assert!(verify(&good).is_none());
    }

    /// signer/sign.mjs (the same code as the Worker's) makes keys this file
    /// takes. Needs node and openssl; skipped without them.
    #[test]
    fn signer_keys_verify() {
        use std::process::Command;
        let dir = tempfile::tempdir().unwrap();
        let pem = dir.path().join("key.pem");
        let ok = |c: &mut Command| c.status().is_ok_and(|s| s.success());
        if !ok(Command::new("openssl")
            .args(["genpkey", "-algorithm", "ed25519", "-out"])
            .arg(&pem))
        {
            eprintln!("skipped: no openssl");
            return;
        }
        let der = Command::new("openssl")
            .args(["pkey", "-pubout", "-outform", "DER", "-in"])
            .arg(&pem)
            .output()
            .unwrap()
            .stdout;
        let public: [u8; 32] = der[der.len() - 32..].try_into().unwrap();
        let script = concat!(env!("CARGO_MANIFEST_DIR"), "/../signer/sign.mjs");
        let Ok(out) = Command::new("node")
            .arg(script)
            .arg("gh:octocat")
            .arg(&pem)
            .output()
        else {
            eprintln!("skipped: no node");
            return;
        };
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        let key = String::from_utf8(out.stdout).unwrap();
        let license = verify_with(&key, &public).expect("sign.mjs key verifies");
        assert_eq!(license.id, "gh:octocat");
        assert_eq!(license.issued.len(), 10);
    }
}
