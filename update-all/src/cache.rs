use crate::CACHE_FILE;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::prelude::*;

use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheEntry {
    name: String,
    last_mod: SystemTime,
}

impl CacheEntry {
    pub fn new(name: String) -> CacheEntry {
        return CacheEntry {
            name,
            last_mod: SystemTime::now(),
        };
    }
    pub fn update(&mut self) {
        self.last_mod = SystemTime::now();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    entries: HashMap<String, CacheEntry>,
}

impl Cache {
    pub fn new() -> Cache {
        let entries = HashMap::new();
        return Cache { entries };
    }

    // @TODO: turn Cache into a "namespace" thing...
    pub fn ensure_cache_file() {
        trace!("execute ensure_cache_file");
        let cache_path = Path::new(CACHE_FILE);
        if !cache_path.exists() {
            fs::File::create(cache_path).unwrap();
        }
    }

    pub fn from_cache_file() -> Cache {
        let cache_raw: String = fs::read_to_string(CACHE_FILE).expect("cannot read cache data");
        let cache: Cache = serde_json::from_str(&cache_raw).expect("cannot parse cache data");
        return cache;
    }

    pub fn could_load_from_file() -> bool {
        let result = match fs::read_to_string(CACHE_FILE) {
            Ok(cache_raw) => match serde_json::from_str::<Cache>(&cache_raw) {
                Ok(_) => true,
                Err(_) => true,
            },
            Err(_) => false,
        };
        result
    }

    /// Write current cache to cache file
    pub fn export(&self) -> io::Result<()> {
        let cache_raw = match serde_json::to_string_pretty(&self) {
            Ok(s) => s,
            Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        };
        Cache::ensure_cache_file();
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(CACHE_FILE)
            .unwrap();
        file.write(cache_raw.as_bytes())
            .expect("Canot write cache to file");
        Ok(())
    }

    pub fn remove_file() -> io::Result<()> {
        let cache_path = Path::new(CACHE_FILE);
        if cache_path.exists() {
            debug!("Delete cache file");
            fs::remove_file(cache_path).expect("Cannot remove cache file")
        }
        Cache::ensure_cache_file();
        Ok(())
    }

    pub fn update(&mut self, key: &String) -> io::Result<()> {
        if self.entries.contains_key(key) {
            if let Some(entry) = self.entries.get_mut(key) {
                entry.update();
            }
        } else {
            self.entries
                .insert(key.clone(), CacheEntry::new(key.clone()));
        }
        Ok(())
    }

    pub fn exists(&self, key: &String) -> bool {
        return self.entries.contains_key(key);
    }
}