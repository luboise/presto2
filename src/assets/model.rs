use std::sync::Arc;

use crate::assets::{Asset, AssetBytes};

#[derive(Debug)]
pub struct Model {
    bytes: Arc<AssetBytes>,
}

impl Asset for Model {
    fn from_bytes(bytes: &Arc<AssetBytes>) -> super::Result<Self> {
        Ok(Self {
            bytes: bytes.clone(),
        })
    }
}
