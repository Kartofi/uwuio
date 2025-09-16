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



#[derive(Debug,Default,Deserialize)]
pub struct ListWaifus{
    pub images: Vec<WaifuImage>
}

#[derive(Debug,Default,Deserialize)]
pub struct WaifuImage{
    pub url: String,
}
impl WaifuImage{
    pub fn new(url:&str) -> WaifuImage{
        WaifuImage { url: url.to_string() }
    }
    pub fn save(&self,path: &str) -> Result<(),reqwest::Error>{
        utils::save_image(&self.url, path)
    }
}
