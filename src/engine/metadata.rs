
use anyhow::Result;
use std::path::Path;

pub(crate) struct EngineMetadataRepo {
    sled_db: sled::Db,
}

impl EngineMetadataRepo {
    pub(crate) fn new(path: &Path) -> Result<Self> {
        log::info!("Opening meta data db");
        let sled_db = sled::open(path)
            .map_err(anyhow::Error::new)?;
        Ok(Self {
            sled_db,
        })
    }

    fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        match self.sled_db.get(key)? {
            Some(bytes) => Ok(Some(bytes.to_vec())),
            None => Ok(None),
        }
    }

    fn set(&self, key: &str, value: &[u8]) -> Result<()> {
        self.sled_db.insert(key, value)
            .map_err(anyhow::Error::new)?;
        Ok(())
    }

    fn remove(&self, key: &str) -> Result<()> {
        self.sled_db.remove(key)
            .map_err(anyhow::Error::new)?;
        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.sled_db.flush()
            .map_err(anyhow::Error::new)?;
        Ok(())
    }

    fn set_last_indexed_timestamp(&self, repo_name: &str, timestamp: u64) -> Result<()> {
        self.set(&format!("last_indexed_time_{}", repo_name), &timestamp.to_be_bytes())
    }

    pub(crate) fn get_last_indexed_timestamp(&self, repo_name: &str) -> Result<Option<u64>> {
        match self.get(&format!("last_indexed_time_{}", repo_name))? {
            Some(bytes) => {
                let mut array = [0; 8];
                array.copy_from_slice(&bytes);
                Ok(Some(u64::from_be_bytes(array)))
            }
            None => Ok(None),
        }
    }

    fn get_number_of_indexed_files(&self, repo_name: &str) -> Result<Option<u64>> {
        match self.get(&format!("number_of_indexed_files_{}", repo_name))? {
            Some(bytes) => {
                let mut array = [0; 8];
                array.copy_from_slice(&bytes);
                Ok(Some(u64::from_be_bytes(array)))
            }
            None => Ok(None),
        }
    }

    fn set_number_of_indexed_files(&self, repo_name: &str, number_of_files: u64) -> Result<()> {
        self.set(&format!("number_of_indexed_files_{}", repo_name), &number_of_files.to_be_bytes())
    }

    fn get_indexing_status(&self, repo_name: &str) -> Result<Option<String>> {
        match self.get(&format!("indexing_status_{}", repo_name))? {
            Some(bytes) => {
                Ok(Some(String::from_utf8(bytes).unwrap()))
            }
            None => Ok(None),
        }
    }

    fn set_indexing_status(&self, repo_name: &str, status: &str) -> Result<()> {
        self.set(&format!("indexing_status_{}", repo_name), status.as_bytes())
    }

    pub(crate) fn is_repo_indexed(&self, repo_name: &str) -> bool {
        self.get_last_indexed_timestamp(repo_name).unwrap().is_some()
    }

    fn mark_repo_indexed(&self, repo_name: &str) -> Result<()> {
        self.set_last_indexed_timestamp(repo_name, chrono::Utc::now().timestamp() as u64)?;
        Ok(())
    }
}