#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use library::para_info::{ParaInfo, ParaKind};
use library::para_history_json::ParaHistoryJson;
use library::setting_json::{SettingFile, SETTING_JSON};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[macro_use] extern crate rocket;

#[derive(Debug,Serialize,Deserialize)]
pub struct CheckPara {
    pub hinmoku_code:String,
    pub machine_para:String,
    pub address:String,
    pub is_para:bool,
}

#[get("/")]
async fn index() -> &'static str {
    "hello!! para porter site"
}

#[get("/check_para/<hinmoku_code>")]
async fn check_para(hinmoku_code:&str)->Json<CheckPara> {
    rocket::serde::json::Json( CheckPara{
        hinmoku_code:hinmoku_code.to_string(),
        machine_para:"".to_string(),
        address:"".to_string(),
        is_para:true,
    })

}


#[post("/post_para",data="<para_info>")]
async fn post_para(para_info:Json<ParaInfo>){
    let para_obj = para_info.0;
    let setting_content = SettingFile::read();
    match para_obj.para_kind {
        ParaKind::Bariga => {
            para_obj.write_file(&setting_content.bariga_folder_path);
            
        },
        ParaKind::Hyomen => {
            para_obj.write_file(&setting_content.omote_folder_path);
            para_obj.write_file(&setting_content.ura_folder_path);

        },
    };
    
    ParaHistoryJson::init();
    ParaHistoryJson::write(&para_obj);
    
    println!("{:?}",para_obj.file_name);
}

#[rocket::main]
async fn main() -> Result<(),rocket::Error> {
    match SettingFile::is_file(){
        true => {rocket::build()
            .mount("/", routes![index])
            .mount("/", routes![post_para])
            .mount("/", routes![check_para])
            .launch().await?;}

        false => {
            SettingFile::init();
            println!("{}がありません。初期設定を行いました。",SETTING_JSON);
            println!("もう一度起動を行ってください。");
        },
    };

    Ok(())
}