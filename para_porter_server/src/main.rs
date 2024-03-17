use library::para_info::{ParaInfo, ParaKind};
use library::encode_shift_jis::ShiftjisFile;
use library::setting_json::{SettingFile, SETTING_JSON};
use rocket::serde::json::Json;

#[macro_use] extern crate rocket;



#[get("/")]
fn index() -> Json<ParaInfo> {
    let shift_jis_file = ShiftjisFile::to_utf8("sample/CO0013Q9(mh5a0-a).txt");

    Json(ParaInfo{
        file_name:Some(shift_jis_file.file_name),
        content:Some(shift_jis_file.utf8_content),
        para_kind:Some(ParaKind::Bariga),
    })
}

#[post("/post_para",data="<para_info>")]
fn post_para(para_info:Json<ParaInfo>) {
    println!("{:?}",para_info.0.file_name);
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