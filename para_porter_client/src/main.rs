#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use library::para_info::{ParaInfo, ParaKind};
mod porter;
use porter::Porter;

const POST_URL: &str = "http://127.0.0.1:8000/post_para";

#[tokio::main]
async fn main() {
    let porter_bot = Porter::init();
    let para_luggage = ParaInfo {
        file_name: Some("t.txt".to_string()),
        content: Some("tesgabagarta".to_string()),
        para_kind: Some(ParaKind::Bariga),
    };

    porter_bot.send_post(POST_URL, &para_luggage).await;
    // println!("{}",res);
}
