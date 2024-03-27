#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use library::para_info::ParaInfo;
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


#[get("/receive_para/{hinmoku_code}")]
async fn receive_para(hinmoku_code:web::Path<String>)-> HttpResponse {
    let setting_file = SettingJson::read();
    let machine_para = setting_file.machine_name;
    let address = Config::get_my_ip_address();
    let hinmoku_code = hinmoku_code.into_inner();
    let para_info = ParaInfo::new(&hinmoku_code,&machine_para,&address);
    HttpResponse::Ok().json(para_info)

}



#[post("/post_para")]
async fn post_para(para_info:web::Json<ParaInfo>)->HttpResponse{
    let para_obj = para_info.0;
    para_obj.write_file();
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