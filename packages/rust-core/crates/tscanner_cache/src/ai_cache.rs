use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;
use tscanner_constants::cache_dir_name;
use tscanner_types::Issue;

fn get_mtime_secs(path: &Path) -> Option<u64> {
    fs::metadata(path)
        .ok()?
        .modified()
        .ok()?
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs())
}

#[derive(Clone, Serialize, Deserialize)]
struct AiCacheEntry {
    prompt_mtime: u64,
    files_mtimes: HashMap<PathBuf, u64>,
    issues: Vec<Issue>,
}

pub struct AiCache {
    entries: DashMap<String, AiCacheEntry>,
    config_hash: u64,
    cache_dir: Option<PathBuf>,
}

impl Default for AiCache {
    fn default() -> Self {
        Self::new()
    }
}

impl AiCache {
    pub fn new() -> Self {
        Self {
            entries: DashMap::new(),
            config_hash: 0,
            cache_dir: Self::get_cache_dir(),
        }
    }

    pub fn with_config_hash(config_hash: u64) -> Self {
        let cache_dir = Self::get_cache_dir();
        let mut cache = Self {
            entries: DashMap::new(),
            config_hash,
            cache_dir: cache_dir.clone(),
        };

        if let Some(dir) = cache_dir {
            cache.load_from_disk(&dir);
        }

        cache
    }

    fn get_cache_dir() -> Option<PathBuf> {
        let cache_dir = dirs::cache_dir()?.join(cache_dir_name());
        fs::create_dir_all(&cache_dir).ok()?;
        Some(cache_dir)
    }

    fn load_from_disk(&mut self, cache_dir: &Path) {
        let cache_file = cache_dir.join(format!("ai_cache_{}.json", self.config_hash));

        if !cache_file.exists() {
            return;
        }

        if let Ok(content) = fs::read_to_string(&cache_file) {
            if let Ok(entries) = serde_json::from_str::<Vec<(String, AiCacheEntry)>>(&content) {
                for (rule_name, entry) in entries {
                    self.entries.insert(rule_name, entry);
                }
            }
        }
    }

    fn save_to_disk(&self) {
        if let Some(cache_dir) = &self.cache_dir {
            let cache_file = cache_dir.join(format!("ai_cache_{}.json", self.config_hash));

            let entries: Vec<(String, AiCacheEntry)> = self
                .entries
                .iter()
                .map(|entry| (entry.key().clone(), entry.value().clone()))
                .collect();

            if let Ok(content) = serde_json::to_string(&entries) {
                let _ = fs::write(&cache_file, &content);
            }
        }
    }

    pub fn get(
        &self,
        rule_name: &str,
        prompt_path: &Path,
        files: &[(PathBuf, String)],
    ) -> Option<Vec<Issue>> {
        let entry = self.entries.get(rule_name)?;

        let current_prompt_mtime = get_mtime_secs(prompt_path)?;
        if entry.prompt_mtime != current_prompt_mtime {
            return None;
        }

        if entry.files_mtimes.len() != files.len() {
            return None;
        }

        for (path, _) in files {
            let current_mtime = get_mtime_secs(path)?;
            let cached_mtime = entry.files_mtimes.get(path)?;
            if *cached_mtime != current_mtime {
                return None;
            }
        }

        Some(entry.issues.clone())
    }

    pub fn insert(
        &self,
        rule_name: &str,
        prompt_path: &Path,
        files: &[(PathBuf, String)],
        issues: Vec<Issue>,
    ) {
        let prompt_mtime = match get_mtime_secs(prompt_path) {
            Some(m) => m,
            None => return,
        };

        let mut files_mtimes = HashMap::new();
        for (path, _) in files {
            if let Some(mtime) = get_mtime_secs(path) {
                files_mtimes.insert(path.clone(), mtime);
            }
        }

        self.entries.insert(
            rule_name.to_string(),
            AiCacheEntry {
                prompt_mtime,
                files_mtimes,
                issues,
            },
        );
    }

    pub fn clear(&self) {
        self.entries.clear();
        if let Some(cache_dir) = &self.cache_dir {
            let cache_file = cache_dir.join(format!("ai_cache_{}.json", self.config_hash));
            let _ = fs::remove_file(&cache_file);
        }
    }

    pub fn flush(&self) {
        self.save_to_disk();
    }
}
