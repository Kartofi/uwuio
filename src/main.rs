mod structs;
mod utils;

const NEKOS_LIST_IDS_URL: &str = "https://nekos.moe/api/v1/random/image?nsfw=false&count=";
const NEKOS_IMAGE_URL: &str = "https://nekos.moe/image/";

const WAIFU_URL: &str = "https://api.waifu.im/search?included_tags=waifu&included_tags=maid";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("ERROR! No args! Use \"neko\" or \"waifu\".");
        return;
    }
    match args[1].to_string().as_str() {
        "neko" => {
            handle_neko();
        }
        "waifu" => {
            if args.len() < 3 {
                println!("ERROR! No tag for source waifu! Example is \"uwuio waifu maid\".");
                return;
            }
            handle_waifu(&args[2]);
        }
        _ => {
            println!("Unknown source!")
        }
    }
}
fn handle_neko() {
    let list_nekos = utils::get_nekos_list(1).unwrap_or_default();
    let image = list_nekos.images.first().unwrap();
    image.save("./tmp.png").unwrap();
    utils::save_img_clipboard("./tmp.png");

    utils::send_notification("uwuio", "Saved neko image to clipboard <3");
}
fn handle_waifu(tag: &str) {
    let list_waifu = utils::get_waifus_list(tag).unwrap_or_default();
    let image = list_waifu.images.first().unwrap();
    image.save("./tmp.png").unwrap();
    utils::save_img_clipboard("./tmp.png");

    utils::send_notification("uwuio", &format!("Saved waifu {} image to clipboard <3", tag));
}
