/// TODO：
/// 表裏、ItemMaster.INIファイルの更新
/// バリ、表、裏にファイルの書き込み
/// 各設備にリクエスト処理する前に自分のアドレスに聞いてみてあったら、既にあることを伝える。
/// 他の設備が持っているIPアドレスもついでに取得する。
///　全てのコメント書き

use anyhow::Ok;
use library::setting::SettingJson;
mod menu;
mod service;

use std::collections::HashSet;
use std::iter::FromIterator;
use library::common_variable::{BARIGA_FOLDER_PATH, OMOTE_FOLDER_PATH, URA_FOLDER_PATH};
use library::valid_hinmoku_code::VaildHinmokuCode;
use library::para_log_json::ParaLog;
use library::setting::{SETTING_DIR_PATH,SETTING_JSON_PATH};
use menu::{HomeSelectionItem,Menu};



async fn search_other_machine(){
    let hinmoku_code = Menu::search_hinmoku_menu_1st().await;
    let valid_hiumoku_code = VaildHinmokuCode::new(&hinmoku_code);
    let is_himoku_code = valid_hiumoku_code.is_hinmoku_code();
    
    match is_himoku_code {
        true =>{
            let para_infos = service::is_there_the_para_file(hinmoku_code.trim()).await;
            
            let option_selected_para_info = Menu::search_hinmoku_menu_2nd(&para_infos).await;
            
            match option_selected_para_info {
                Some(selected_para_info) => {
                    let process_result = service::put_files_several_folder(&selected_para_info).await;
                    println!("設備名:{}のファイルを基に以下に保存しました。",selected_para_info.machine_name);
                    
                    if process_result.bariga{
                        println!("バリ画ファイル:{}/{}に保存しました。",BARIGA_FOLDER_PATH,selected_para_info.bariga_file_name);
                    } else {
                        println!("バリ画ファイル:{}/{}に保存に失敗しました。",BARIGA_FOLDER_PATH,selected_para_info.bariga_file_name);
                    }

                    if process_result.omote {
                        println!("表表面ファイル:{}/{}に保存しました。",OMOTE_FOLDER_PATH,selected_para_info.hyomen_file_name);
                    }else {
                        println!("表表面ファイル:{}/{}に保存に失敗しました。",OMOTE_FOLDER_PATH,selected_para_info.hyomen_file_name);

                    }

                    if process_result.ura {
                        println!("裏表面ファイル:{}/{}に保存しました。",URA_FOLDER_PATH,selected_para_info.hyomen_file_name);
                    }else {
                        println!("裏表面ファイル:{}/{}に保存に失敗しました。",URA_FOLDER_PATH,selected_para_info.hyomen_file_name);
                    }

                    ParaLog::write_file(&selected_para_info);


                },
                None => {println!("{}のファイルが存在していません。",hinmoku_code);}
            }
        },
        false => {
            println!("{}は品目コードとして認識しません。CO1234A00もしくはCO21234-C0Aの桁数を入力してください。",hinmoku_code)
        }
    }
}

pub async fn search_ips(){
    let mut friend_ips = vec![];
    let setting_jsons = service::receive_friend_ips().await;
    for setting_json in setting_jsons.iter(){
        for friend_ip in setting_json.friend_ips.iter(){
            friend_ips.push(friend_ip.clone());
        }
    }

    let mut unique:HashSet<&str> = HashSet::new();
    for friend_ip in friend_ips.iter(){
        unique.insert(friend_ip);
    }
    let unique_ips = Vec::from_iter(unique);

    let setting_json_path = std::path::Path::new(SETTING_DIR_PATH).join(SETTING_JSON_PATH);

    let mut  setting_json  = match setting_json_path.exists(){
        true => {
            SettingJson::read(false)
        },
        false => {
            SettingJson::read(true)
        },
    };

    setting_json.update_ips(&unique_ips);    
    setting_json.write_file();
    
    println!("{:?}を取得しました。",unique_ips);
    println!("IPアドレス収集完了しました。{:?}/{:?}をご確認ください。",SETTING_DIR_PATH,SETTING_JSON_PATH);
}


#[tokio::main]
async fn main()->anyhow::Result<(),anyhow::Error>{
    loop {
        let select_item = Menu::home().await;
        match select_item {
            HomeSelectionItem::SearchMyFolder => service::see_my_folder(),
            HomeSelectionItem::SearchOtherMachine => search_other_machine().await,
            HomeSelectionItem::FriendIPs => search_ips().await,
            HomeSelectionItem::End => break,
        }
    }
    Ok(())
}