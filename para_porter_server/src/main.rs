#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use library::para_info::{ParaInfo, ParaKind};
use library::para_history_json::ParaHistoryJson;
use library::setting::{SettingJson, SETTING_JSON_PATH,Config};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

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
    let setting_file = SettingJson::read();
    let machine_para = setting_file.machine_name;
    let address = Config::get_my_ip_address();

    let dir = std::fs::read_dir("NOKENV").unwrap();
    let mut is_para = false;
    for dir_entry_result in dir {
        let dir_entry = dir_entry_result.unwrap();
        let file_name = dir_entry.file_name().into_string().unwrap_or_default();
        match file_name.find(hinmoku_code){
            Some(_) =>  {
                is_para = true;
                break;
            },
            None => {},
        };
    }

    rocket::serde::json::Json(
        CheckPara {
            hinmoku_code:hinmoku_code.to_string(),
            machine_para,
            address,
            is_para,
        }
    )

}

#[get("/receive_para/<hinmoku_code>")]
async fn receive_para(hinmoku_code:&str)->Json<ParaInfo> {
    let setting_file = SettingJson::read();
    let machine_para = setting_file.machine_name;
    let address = Config::get_my_ip_address();

    let dir = std::fs::read_dir("NOKENV").unwrap();
    let mut is_para = false;
    let mut target_file_name = String::new();
    for dir_entry_result in dir {
        let dir_entry = dir_entry_result.unwrap();
        let file_name = dir_entry.file_name().into_string().unwrap_or_default();
        match file_name.find(hinmoku_code){
            Some(_) =>  {
                is_para = true;
                target_file_name = file_name;
                break;
            },
            None => {},
        };
    }

    rocket::serde::json::Json(
        ParaInfo::new(hinmoku_code,&target_file_name,ParaKind::Hyomen,&machine_para) 
    )

}



#[post("/post_para",data="<para_info>")]
async fn post_para(para_info:Json<ParaInfo>){
    let para_obj = para_info.0;
    let setting_content = SettingJson::read();
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
    match SettingJson::is_file(){
        true => {rocket::build()
            .mount("/", routes![index])
            .mount("/", routes![post_para])
            .mount("/", routes![check_para])
            .mount("/", routes![receive_para])
            .launch().await?;}

        false => {
            SettingJson::init();
            println!("{}がありません。初期設定を行いました。",SETTING_JSON_PATH);
            println!("もう一度起動を行ってください。");
        },
    };

    Ok(())
}