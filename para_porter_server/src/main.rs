#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use library::para_info::{ParaInfo, ParaKind};
use library::encode_shift_jis::ShiftjisFile;
use library::setting_json::{self, SettingFile, SETTING_JSON};
use rocket::serde::json::Json;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "hello!! para porter site"
}

#[post("/post_para",data="<para_info>")]
fn post_para(para_info:Json<ParaInfo>){
    let para_obj = para_info.0;
    let setting_content = SettingFile::read();
    match para_obj.para_kind {
        Some(ParaKind::Bariga) =>para_obj.write_file(&setting_content.bariga_folder_path),
        Some(ParaKind::Omote) => para_obj.write_file(&setting_content.omote_folder_path),
        Some(ParaKind::Ura) => para_obj.write_file(&setting_content.ura_folder_path),
        Some(ParaKind::ItemMaster) => println!("ItemMaster"),
        None => {},
    };
    println!("{:?}",para_obj.file_name);
}

#[rocket::main]
async fn main() -> Result<(),rocket::Error> {
    match SettingFile::is_file(){
        true => {rocket::build()
            .mount("/", routes![index])
            .mount("/", routes![post_para])
            .launch().await?;}

        false => {
            SettingFile::init();
            println!("{}がありません。初期設定を行いました。",SETTING_JSON);
            println!("もう一度起動を行ってください。");
        },
    };

    Ok(())
}