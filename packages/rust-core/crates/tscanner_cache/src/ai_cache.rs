use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tscanner_constants::cache_dir_name;
use tscanner_types::Issue;

#[derive(Clone, Serialize, Deserialize)]
struct AiCacheEntry {
    prompt_hash: u64,
    issues: Vec<Issue>,
}

pub struct AiCache {
    entries: DashMap<u64, AiCacheEntry>,
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
            if let Ok(entries) = serde_json::from_str::<Vec<(u64, AiCacheEntry)>>(&content) {
                for (key, entry) in entries {
                    self.entries.insert(key, entry);
                }
            }
        }
    }

    fn save_to_disk(&self) {
        if let Some(cache_dir) = &self.cache_dir {
            let cache_file = cache_dir.join(format!("ai_cache_{}.json", self.config_hash));

            let entries: Vec<(u64, AiCacheEntry)> = self
                .entries
                .iter()
                .map(|entry| (*entry.key(), entry.value().clone()))
                .collect();

            if let Ok(content) = serde_json::to_string(&entries) {
                let _ = fs::write(&cache_file, &content);
            }
        }
    }

    pub fn get(&self, cache_key: u64, prompt_hash: u64) -> Option<Vec<Issue>> {
        if let Some(entry) = self.entries.get(&cache_key) {
            if entry.prompt_hash == prompt_hash {
                return Some(entry.issues.clone());
            }
        }
        None
    }

    pub fn insert(&self, cache_key: u64, prompt_hash: u64, issues: Vec<Issue>) {
        self.entries.insert(
            cache_key,
            AiCacheEntry {
                prompt_hash,
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
