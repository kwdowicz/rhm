use std::io;
use tokio::fs::{File, metadata, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const DATA_FILE: &str = "data.txt";

#[derive(Debug)]
pub struct Storage {
    file: File,
}

impl Storage {
    pub async fn new() -> tokio::io::Result<Self> {
        if Self::not_exists().await? {
            Self::create().await?
        }

        Ok(Self {
            file: Self::open().await?
        })
    }

    async fn not_exists() -> std::io::Result<bool> {
        match metadata(DATA_FILE).await {
            Ok(_) => Ok(false),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(true),
            Err(e) => Err(e),
        }
    }

    async fn create() -> tokio::io::Result<()> {
        File::create(DATA_FILE).await?;
        Ok(())
    }

    pub async fn open() -> tokio::io::Result<File> {
        let mut file = OpenOptions::new()
            .append(true)
            .read(true)
            .open(DATA_FILE)
            .await?;
        Ok(file)
    }

    pub async fn save(&mut self, data: &str) -> tokio::io::Result<()> {
        self.file.write_all(data.as_bytes()).await?;
        Ok(())
    }

    pub async fn load(&mut self) -> tokio::io::Result<String> {
        let mut contents = String::new();
        self.file.read_to_string(&mut contents).await?;
        Ok(contents)
    }
}