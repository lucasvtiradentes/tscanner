use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hash_content(content: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    hasher.finish()
}
