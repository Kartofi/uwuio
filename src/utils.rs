use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Error},
    process,
};

use crate::structs::*;

pub fn get_nekos_list(count: u8) -> Result<ListNekos, reqwest::Error> {
    let req = reqwest::blocking::get(crate::NEKOS_LIST_IDS_URL)?.error_for_status()?;

    let res: ListNekos = serde_json::from_str(&req.text().unwrap_or_default()).unwrap_or_default();
    Ok(res)
}

pub fn get_waifus_list(tag: &str) -> Result<ListWaifus, reqwest::Error> {
    let req = reqwest::blocking::get(crate::WAIFU_URL)?.error_for_status()?;

    let res: ListWaifus = serde_json::from_str(&req.text().unwrap_or_default()).unwrap_or_default();

    Ok(res)
}

pub fn save_nekos_list(list_nekos: &ListNekos) -> Result<(), reqwest::Error> {
    for image in &list_nekos.images {
        image.save(&("./images/neko_".to_string() + &image.id + ".png"))?;
    }
    Ok(())
}
pub fn save_image(url: &str, path: &str) -> Result<(), reqwest::Error> {
    let mut req = reqwest::blocking::get(url)?.error_for_status()?;
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)
        .unwrap();
    let mut write_stream = BufWriter::new(file);
    req.copy_to(&mut write_stream).unwrap();

    Ok(())
}

pub fn save_img_clipboard(path: &str) -> bool {
    process::Command::new("sh")
        .args(&["-c", &format!("wl-copy --type image/png < {}", path)])
        .spawn()
        .is_ok()
}
