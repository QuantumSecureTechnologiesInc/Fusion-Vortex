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
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FusionToml {
    pub package: Option<Package>,
    pub dependencies: Option<std::collections::BTreeMap<FString, toml::Value>>,
    pub monolith: Option<Monolith>,
    pub runtime: Option<Runtime>,
    pub build: Option<Build>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: FString,
    pub version: Option<FString>,
    pub edition: Option<FString>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monolith {
    pub enabled: Option<FBool>,
    pub persistence_path: Option<FString>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Runtime {
    pub profile: Option<FString>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Build {
    pub optimization_level: Option<u8>,
}
#[derive(thiserror::Error, Debug)]
pub enum ProjectError {
    #[error("Fusion.toml not found at {0}")]
    NotFound(FString),
    #[error("failed to read Fusion.toml: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse Fusion.toml: {0}")]
    Parse(#[from] toml::de::Error),
}
pub fn find_project_root(start: &Path) -> Option<PathBuf> {
    let mut cur = Some(start.to_path_buf());
    while let Some(p) = cur {
        if p.join("Fusion.toml").exists() {
            return Some(p);
        }
        cur = p.parent().map(|pp| pp.to_path_buf());
    }
    None
}
pub fn load_fusion_toml(project_root: &Path) -> Result<FusionToml, ProjectError> {
    let path = project_root.join("Fusion.toml");
    if !path.exists() {
        return Err(ProjectError::NotFound(path.display().to_string()));
    }
    let src = fs::read_to_string(path)?;
    Ok(toml::from_str(&src)?)
}
pub fn cache_dir(project_root: &Path, cfg: &FusionToml) -> PathBuf {
    cfg.monolith
        .as_ref()
        .and_then(|m| m.persistence_path.as_ref())
        .map(|s| project_root.join(s))
        .unwrap_or_else(|| project_root.join(".fusion").join("cache"))
}
