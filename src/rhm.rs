use std::collections::HashMap;
use tokio::io::{Result};
use crate::storage::Storage;

#[derive(Debug)]
pub enum RhmResult {
    NewInsertOk,
    PreviousValue(String),
    NoKey,
    Value(String),
}

impl RhmResult {
    pub fn value(&self) -> String {
        match self {
            RhmResult::NewInsertOk => { "Ok".to_string() }
            RhmResult::PreviousValue(value) => { value.to_string() }
            RhmResult::NoKey => { "NoKey".to_string() }
            RhmResult::Value(value) => { value.to_string() }
        }
    }
}

#[derive(Debug)]
pub struct Rhm {
    pub items: HashMap<String, String>,
    storage: Storage,
}

impl Rhm {
    pub async fn new() -> Result<Self> {
        let mut rhm = Rhm { items: HashMap::new(), storage: Storage::new().await?, };
        rhm.load().await?;
        Ok(rhm)
    }

    async fn load(&mut self) -> Result<()> {
        let mut contents = self.storage.load().await?;
        for line in contents.lines() {
            let mut parts = line.splitn(3, '|');
            match (parts.next(), parts.next(), parts.next()) {
                (Some("SET"), Some(key), Some(value)) => {
                    self.items.insert(key.to_string(), value.to_string());
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub async fn set(&mut self, key: &str, value: &str) -> Result<RhmResult> {
        let result = match self.items.insert(key.to_string(), value.to_string()) {
            Some(old_value) => RhmResult::PreviousValue(old_value),
            None => RhmResult::NewInsertOk,
        };
        self.storage.save(&format!("SET|{}|{}\r\n", key, value)).await?;
        Ok(result)
    }

    pub fn get(&self, key: &str) -> RhmResult {
        self.items.get(key).map_or(RhmResult::NoKey, |v| RhmResult::Value(v.clone()))
    }
}