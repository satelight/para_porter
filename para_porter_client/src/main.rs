#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use library::para_info::{ParaInfo, ParaKind};
mod porter;
use porter::Porter;
use library::setting_json::SettingFile;

const POST_URL: &str = "http://127.0.0.1:8000/post_para";

#[tokio::main]
async fn main() {
    let setting_file = SettingFile::read();
    let porter_bot = Porter::init();
    let hinmoku_code = "CO0008Y2";
    let file_name = "CO0008Y2(sh5a0-a).txt";
    let para_info = ParaInfo::new(hinmoku_code, file_name, ParaKind::Bariga,&setting_file.machine_name);
    porter_bot.send_post(POST_URL, &para_info).await;
    // println!("{}",res);
}
