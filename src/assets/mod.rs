use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Weak},
};

mod model;
pub use model::*;

#[derive(Debug)]
pub enum AssetError {
    InvalidPath,
    ReadError(String),
}

pub trait Asset: Sized {
    fn from_bytes(bytes: &Arc<AssetBytes>) -> Result<Self>;
}

type AssetBytes = Vec<u8>;

type Result<T> = std::result::Result<T, AssetError>;

pub struct AssetRegistry {
    base_path: PathBuf,
    entries: HashMap<PathBuf, AssetEntry>,
}

pub struct AssetEntry {
    bytes: Weak<AssetBytes>,
}

impl AssetRegistry {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        if !path.is_dir() {
            return Err(AssetError::InvalidPath);
        }

        Ok(Self {
            base_path: path.to_path_buf(),
            entries: Default::default(),
        })
    }

    pub fn load_asset<A, P>(&mut self, asset_path: P) -> Result<A>
    where
        A: Asset,
        P: AsRef<Path>,
    {
        // Load the asset here

        let path: &Path = asset_path.as_ref();

        // If asset entry already exists, get the bytes
        if let Some(entry) = self.get_entry(&path.to_path_buf()) {
            let bytes = entry
                .bytes
                .upgrade()
                .expect("Unable to upgrade asset pointer.");

            A::from_bytes(&bytes)
        } else {
            let bytes = Arc::new(
                fs::read(self.base_path.join(asset_path.as_ref()))
                    .map_err(|e| AssetError::ReadError(e.to_string()))?,
            );

            let entry = AssetEntry {
                bytes: Arc::downgrade(&bytes),
            };

            self.entries.insert(path.into(), entry);
            A::from_bytes(&bytes)
        }
    }

    pub(crate) fn get_entry(&self, path_buf: &PathBuf) -> Option<&AssetEntry> {
        self.entries
            .get(path_buf)
            .filter(|val| val.bytes.strong_count() > 0)
    }

    pub fn base_path(&self) -> &PathBuf {
        &self.base_path
    }
}
