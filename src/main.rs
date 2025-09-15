mod structs;
mod utils;

const NEKOS_LIST_IDS_URL: &str = "https://nekos.moe/api/v1/random/image?nsfw=false&count=";
const NEKOS_IMAGE_URL: &str = "https://nekos.moe/image/";

const WAIFU_URL: &str = "https://api.waifu.im/search?included_tags=waifu&included_tags=maid";

fn main() {
    let list_nekos = utils::get_nekos_list(1).unwrap_or_default();
    let image = list_nekos.images.first().unwrap();
    image.save("./tmp.png").unwrap();
    utils::save_img_clipboard("./tmp.png");
}
