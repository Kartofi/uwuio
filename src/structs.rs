use serde::{Deserialize, Serialize};

use crate::utils;

#[derive(Debug, Default, Deserialize)]
pub struct ListNekos {
    pub images: Vec<NekoImage>,
}
#[derive(Debug, Default, Deserialize)]
pub struct NekoImage {
    pub id: String,
}
impl NekoImage {
    pub fn new(id: &str) -> NekoImage {
        NekoImage { id: id.to_string() }
    }
    pub fn save(&self, path: &str) -> Result<(), reqwest::Error> {
        utils::save_image(&(crate::NEKOS_IMAGE_URL.to_string() + &self.id), path)
    }
}
