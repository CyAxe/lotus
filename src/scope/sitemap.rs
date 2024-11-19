use std::fs::File;
use std::io::{self, Read, Write};
use std::collections::HashMap;
use std::sync::RwLock;
use bincode;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequestResponse {
    pub url: String,
    pub method: String,
    pub request: String,
    pub response: String,
    pub status_code: u16,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SiteMap {
    map: RwLock<HashMap<String, HttpRequestResponse>>,
}

impl SiteMap {
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }

    pub fn add_entry(&self, entry: HttpRequestResponse) {
        let mut map = self.map.write().unwrap();
        map.insert(entry.url.clone(), entry);
    }

    pub fn get_entry(&self, url: &str) -> Option<HttpRequestResponse> {
        let map = self.map.read().unwrap();
        map.get(url).cloned()
    }

    pub fn remove_entry(&self, url: &str) -> Option<HttpRequestResponse> {
        let mut map = self.map.write().unwrap();
        map.remove(url)
    }

    pub fn list_entries(&self) -> Vec<HttpRequestResponse> {
        let map = self.map.read().unwrap();
        map.values().cloned().collect()
    }

    pub fn save_to_file(&self, file_path: &str) -> io::Result<()> {
        let map = self.map.read().unwrap();
        let serialized = bincode::serialize(&*map).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let mut file = File::create(file_path)?;
        file.write_all(&serialized)
    }

    pub fn load_from_file(file_path: &str) -> io::Result<Self> {
        let mut file = File::open(file_path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        let deserialized: HashMap<String, HttpRequestResponse> = bincode::deserialize(&contents).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(Self {
            map: RwLock::new(deserialized),
        })
    }
}
