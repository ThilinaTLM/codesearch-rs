
use anyhow::Result;

pub fn init_data_dir(path: &str) -> Result<()> {
    if !std::path::Path::new(path).exists() {
        std::fs::create_dir_all(path)
            .map_err(|e| anyhow::anyhow!("Failed to create data directory: {}", e))?;
        log::info!("Data directory created at: {}", path);
        Ok(())
    } else {
        log::info!("Data directory already exists at: {}", path);
        Ok(())
    }
}