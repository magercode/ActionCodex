use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, anyhow};
use sha2::{Digest, Sha256};

const KEY_DIR: &str = ".action-codex";
const KEY_FILE: &str = "ed25519_signing_key.pem";
const BLOB_MAGIC: &[u8; 4] = b"ACSV";
const BLOB_VERSION: u8 = 1;

pub fn write_signature_blob_for_file(target_path: &Path, content: &[u8]) -> Result<PathBuf> {
    let private_key_path = ensure_private_key()?;
    let public_key_der = export_public_key_der(&private_key_path)?;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("waktu sistem sebelum UNIX_EPOCH")?
        .as_secs();

    let mut hasher = Sha256::new();
    hasher.update(content);
    let hash: [u8; 32] = hasher.finalize().into();

    let target_label = target_path.to_string_lossy().to_string();
    let payload = build_payload_to_sign(timestamp, &target_label, &hash)?;
    let signature = sign_payload(&private_key_path, &payload)?;

    let (marker_id, marker_path) = allocate_marker_path(target_path)?;
    let blob = build_binary_blob(
        timestamp,
        &target_label,
        &marker_id,
        &hash,
        &public_key_der,
        &signature,
    )?;

    fs::write(&marker_path, blob)
        .with_context(|| format!("gagal menulis file marker {}", marker_path.display()))?;

    Ok(marker_path)
}

fn ensure_private_key() -> Result<PathBuf> {
    let key_dir = PathBuf::from(KEY_DIR);
    fs::create_dir_all(&key_dir).with_context(|| format!("gagal membuat direktori {}", KEY_DIR))?;

    let key_path = key_dir.join(KEY_FILE);
    if key_path.exists() {
        return Ok(key_path);
    }

    let output = Command::new("openssl")
        .args(["genpkey", "-algorithm", "ED25519", "-out"]) 
        .arg(&key_path)
        .output()
        .context("gagal menjalankan openssl untuk generate kunci ED25519")?;

    if !output.status.success() {
        return Err(anyhow!(
            "openssl genpkey gagal: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(key_path)
}

fn export_public_key_der(private_key_path: &Path) -> Result<Vec<u8>> {
    let output = Command::new("openssl")
        .args(["pkey", "-in"])
        .arg(private_key_path)
        .args(["-pubout", "-outform", "DER"])
        .output()
        .context("gagal menjalankan openssl untuk export public key")?;

    if !output.status.success() {
        return Err(anyhow!(
            "openssl pkey -pubout gagal: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(output.stdout)
}

fn sign_payload(private_key_path: &Path, payload: &[u8]) -> Result<Vec<u8>> {
    let temp_path = std::env::temp_dir().join(format!("action-codex-sign-{:016x}.bin", rand::random::<u64>()));
    fs::write(&temp_path, payload)
        .with_context(|| format!("gagal menulis payload sementara {}", temp_path.display()))?;

    let output = Command::new("openssl")
        .args(["pkeyutl", "-sign", "-rawin", "-inkey"])
        .arg(private_key_path)
        .args(["-in"])
        .arg(&temp_path)
        .output()
        .context("gagal menjalankan openssl untuk sign payload")?;

    let _ = fs::remove_file(&temp_path);

    if !output.status.success() {
        return Err(anyhow!(
            "openssl pkeyutl -sign gagal: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(output.stdout)
}

fn allocate_marker_path(target_path: &Path) -> Result<(String, PathBuf)> {
    let dir = target_path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));

    for _ in 0..256 {
        let id = format!("{:016x}", rand::random::<u64>());
        let path = dir.join(format!(".{}", id));
        if !path.exists() {
            return Ok((id, path));
        }
    }

    Err(anyhow!("gagal membuat nama marker acak unik"))
}

fn build_payload_to_sign(timestamp: u64, target: &str, hash: &[u8; 32]) -> Result<Vec<u8>> {
    let mut payload = Vec::new();
    payload.extend_from_slice(b"ACSIGP1");
    payload.extend_from_slice(&timestamp.to_le_bytes());
    push_u16_and_bytes(&mut payload, target.as_bytes())?;
    payload.extend_from_slice(hash);
    Ok(payload)
}

fn build_binary_blob(
    timestamp: u64,
    target: &str,
    marker_id: &str,
    hash: &[u8; 32],
    public_key_der: &[u8],
    signature: &[u8],
) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    out.extend_from_slice(BLOB_MAGIC);
    out.push(BLOB_VERSION);
    out.extend_from_slice(&timestamp.to_le_bytes());

    push_u16_and_bytes(&mut out, target.as_bytes())?;
    push_u16_and_bytes(&mut out, marker_id.as_bytes())?;
    push_u16_and_bytes(&mut out, hash)?;
    push_u16_and_bytes(&mut out, public_key_der)?;
    push_u16_and_bytes(&mut out, signature)?;

    Ok(out)
}

fn push_u16_and_bytes(out: &mut Vec<u8>, data: &[u8]) -> Result<()> {
    let len: u16 = data
        .len()
        .try_into()
        .context("data terlalu besar untuk format biner")?;
    out.extend_from_slice(&len.to_le_bytes());
    out.extend_from_slice(data);
    Ok(())
}
