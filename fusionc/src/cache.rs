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
use sha2::{Digest, Sha256};
pub fn hash_build_inputs(
    src: &str,
    entry_path: &Path,
    release: FBool,
    fusion_toml: Option<&str>,
) -> FString {
    let mut h = Sha256::new();
    h.update(b"fusionc-v0.1");
    let profile: &[u8] = if release { b"release" } else { b"debug" };
    h.update(profile);
    h.update(entry_path.to_string_lossy().as_bytes());
    h.update(src.as_bytes());
    if let Some(t) = fusion_toml {
        h.update(t.as_bytes());
    }
    hex::encode(h.finalize())
}
pub fn default_out_path(
    project_root: Option<&Path>,
    package_name: Option<&str>,
    release: FBool,
) -> PathBuf {
    if let Some(root) = project_root {
        let tgt = root.join("target").join(if release { "release" } else { "debug" });
        let name = package_name.unwrap_or("app");
        return tgt.join(format!("{name}.fbc"));
    }
    PathBuf::from(if release { "./a.release.fbc" } else { "./a.debug.fbc" })
}
