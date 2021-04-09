use std::path::{Path, PathBuf};
use anyhow::Result;

use super::StorageProvider;
pub struct LocalStorageProvider {
    data_dir: PathBuf
}

impl LocalStorageProvider {
    pub fn new(data_dir: &PathBuf) -> Result<Self> {
        if !Path::exists(&data_dir) {
            std::fs::create_dir_all(&data_dir)?;
        }
        Ok(Self { data_dir: data_dir.clone() })
    }
}

impl super::StorageProvider for LocalStorageProvider {
    fn save(wa_reference: WebAssemblyReference, data: Vec<u8>) -> Result<()> {
        todo!()
    }

    fn load(wa_reference: WebAssemblyReference) -> Vec<u8> {
        todo!()
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_new() -> Result<()> {
        let data_dirs = vec!["./data_a", "/tmp/data_b", "./data_c/data_d"];

        for path in data_dirs.iter().map(Path::new) {
            LocalStorageProvider::new(&PathBuf::from(path))?; 
            assert!(Path::exists(path));
            std::fs::remove_dir_all(path)?;
        }
        std::fs::remove_dir_all(Path::new("./data_c"))?;

        Ok(())
    }
}