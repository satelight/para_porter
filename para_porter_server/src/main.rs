// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use library::para_info::{ParaInfo, ParaKind};
use library::para_history_json::ParaHistoryJson;
use library::setting::{SettingJson, SETTING_JSON_PATH,Config};
use serde::{Deserialize, Serialize};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[derive(Debug,Serialize,Deserialize)]
pub struct CheckPara {
    pub hinmoku_code:String,
    pub machine_para:String,
    pub address:String,
    pub is_para:bool,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("hello!! para porter site")
}

#[get("/check_para/{hinmoku_code}")]
async fn check_para(hinmoku_code:web::Path<String>)-> impl Responder {
    let setting_file = SettingJson::read();
    let machine_para = setting_file.machine_name;
    let address = Config::get_my_ip_address();
    let hinmoku_code = hinmoku_code.into_inner();

    let dir = std::fs::read_dir("NOKENV").unwrap();
    let mut is_para = false;
    for dir_entry_result in dir {
        let dir_entry = dir_entry_result.unwrap();
        let file_name = dir_entry.file_name().into_string().unwrap_or_default();
        match file_name.find(&hinmoku_code){
            Some(_) =>  {
                is_para = true;
                break;
            },
            None => {},
        };
    }

    HttpResponse::Ok().json(
        CheckPara {
            hinmoku_code:hinmoku_code.to_string(),
            machine_para,
            address,
            is_para,
        }
    )

}

#[get("/receive_para/{hinmoku_code}")]
async fn receive_para(hinmoku_code:web::Path<String>)-> HttpResponse {
    let setting_file = SettingJson::read();
    let machine_para = setting_file.machine_name;
    let address = Config::get_my_ip_address();
    let hinmoku_code = hinmoku_code.into_inner();

    let dir = std::fs::read_dir("NOKENV").unwrap();
    let mut is_para = false;
    let mut target_file_name = String::new();
    for dir_entry_result in dir {
        let dir_entry = dir_entry_result.unwrap();
        let file_name = dir_entry.file_name().into_string().unwrap_or_default();
        match file_name.find(&hinmoku_code){
            Some(_) =>  {
                is_para = true;
                target_file_name = file_name;
                break;
            },
            None => {},
        };
    }

    let para_info = ParaInfo::new(
        &hinmoku_code,
        &target_file_name,
        ParaKind::Hyomen,
        &machine_para
    );
    
    HttpResponse::Ok().json(para_info)

}



#[post("/post_para")]
async fn post_para(para_info:web::Json<ParaInfo>)->HttpResponse{
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
    
    HttpResponse::Ok().json(para_obj)
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    match SettingJson::is_file(){
        true => {
            HttpServer::new(||{
                App::new()
                .service(index)
                .service(post_para)
                .service(check_para)
                .service(receive_para)
            })
            .bind(("127.0.0.1",8080))?
            .run()
            .await?
            }
        false => {
            SettingJson::init();
            println!("{}がありません。初期設定を行いました。",SETTING_JSON_PATH);
            println!("もう一度起動を行ってください。");
        },
    };

    Ok(())
}