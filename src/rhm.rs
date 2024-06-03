use std::collections::HashMap;
use std::io;
use tokio::fs::{OpenOptions, File, metadata};
use tokio::io::{AsyncWriteExt, Result};

const DATA_FILE: &str = "data.txt";

pub struct Rhm {
    pub items: HashMap<String, String>,
    storage: File,
}

impl Rhm {

    pub async fn new() -> Result<Self> {
        if !Self::storage_exists().await? {
            File::create(DATA_FILE).await?;
        }

        let mut file = OpenOptions::new()
            .append(true)
            .read(true)
            .open(DATA_FILE)
            .await?;

        let mut new_rhm = Rhm { items: HashMap::new(), storage: file, };

        new_rhm.load().await?;
        Ok(new_rhm)
    }

    async fn storage_exists() -> Result<bool> {
        match metadata(DATA_FILE).await {
            Ok(_) => Ok(true),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e),
        }
    }

    pub async fn insert(&mut self, key: &str, value: &str) -> Result<String> {
        let result = self.items.insert(key.to_string(), value.to_string()).unwrap_or_else(|| "Ok".to_string());
        self.storage.write_all(format!("{}|{}\r\n", key, value).as_bytes()).await?;
        Ok(result)
    }

    pub fn get(&self, key: &str) -> String {
        self.items.get(key).map_or("NoKey".to_string(), |v| v.clone())
    }
}