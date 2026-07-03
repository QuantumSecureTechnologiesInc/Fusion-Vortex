// __FU_COMPAT_START__
use std::fs;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

type FBool = bool;
type FChar = char;
type FInt = i32;
type FI64 = i64;
type FString = String;
type FU32 = u32;
type FU64 = u64;
type FSize = usize;
type FVec<T> = Vec<T>;
type FMap<K, V> = HashMap<K, V>;
type FBTreeMap<K, V> = BTreeMap<K, V>;
type FSet<T> = HashSet<T>;
type FBTreeSet<T> = BTreeSet<T>;
// __FU_COMPAT_END__
use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde_json::json;

pub fn lookup(monolith_url: &str, sha256: &str) -> Result<Option<FVec<u8>>> {
    let payload = json!({ "action": "lookup", "hash": sha256, "unit": "module" });
    let resp = ureq::post(monolith_url)
        .send_json(payload)
        .context("failed to contact monolith")?;
    let v: serde_json::Value = resp.into_json().context("bad monolith json")?;
    if v.get("cached").and_then(|x| x.as_bool()) == Some(true) {
        if let Some(b64) = v.get("artifact").and_then(|a| a.get("bytecode")).and_then(|x| x.as_str()) {
            let bytes = STANDARD.decode(b64).context("bad base64 bytecode")?;
            return Ok(Some(bytes));
        }
    }
    Ok(None)
}

pub fn store(monolith_url: &str, sha256: &str, bytecode: &[u8], effects: serde_json::Value) -> Result<()> {
    let b64 = STANDARD.encode(bytecode);
    let artifact = json!({
        "bytecode": b64,
        "effects": effects,
        "build": { "profile": "default" }
    });
    let payload = json!({ "action": "store", "hash": sha256, "unit": "module", "artifact": artifact });
    let resp = ureq::post(monolith_url)
        .send_json(payload)
        .context("failed to contact monolith")?;
    let _v: serde_json::Value = resp.into_json().context("bad monolith json")?;
    Ok(())
}
