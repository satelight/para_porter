use std::path::Path;

use library::encode_shift_jis::ParseParaFile;
#[allow(unused_imports)]
use dialoguer::Select;
use dialoguer::Confirm;
use library::common_variable::{
    BARIGA_FOLDER_PATH, OMOTE_FOLDER_PATH,URA_FOLDER_PATH,OmoteUra,SERVER_PORT
};
use library::para_info::ParaInfo;
use library::setting::SettingJson;
use library::item_master_ini::ItemMasteINI;
use library::encode_shift_jis::ShiftjisFile;
#[cfg(not(debug_assertions))]
use library::setting::Config;

pub fn see_my_folder(){
    let mut find_files = vec![];
    match std::fs::read_dir(OMOTE_FOLDER_PATH){
        Ok(dir) =>{
            for d_result in dir {
                let dir_entry = d_result.unwrap();
                let file_name = dir_entry.file_name().to_str().unwrap().to_string();
                if file_name.contains(".txt"){
                    // println!("file_name:{:?},find_number{:?}",file_name,find_number);
                    let parse_file = ParseParaFile::read_from_file_path(OMOTE_FOLDER_PATH,&file_name);
                    if let Some(sunpo) = parse_file.get_kijyu_sunpou(){find_files.push(sunpo)};
                };
            }
            
            find_files.sort_by(|a,b|a.gaikei.partial_cmp(&b.gaikei).unwrap());
            // let mut select_file_list = vec![];
            println!("--------------------------------------------------------------");
            for find_file in  find_files.iter() {
                let menu_sentence = format!("{}: 寸法:{} ✕ {} ✕ {}",find_file.file_path,find_file.naikei,find_file.gaikei,find_file.hutosa);
                // select_file_list.push(menu_sentence);
                println!("{}",menu_sentence)
        
            }
            println!("--------------------------------------------------------------");
        },
        Err(_) =>{
            println!("{}フォルダがありません。",OMOTE_FOLDER_PATH);
        }
    };


    let _ = Confirm::new()
        .with_prompt("'Y'か'N'を押すホーム画面に戻ります。")
        .interact()
        .unwrap();

}


pub async fn is_there_the_para_file(hinmoku_code:&str)->Vec<ParaInfo>{
    let mut para_infos:Vec<ParaInfo> = vec![];  
    // setting.jsonから他の設備のIPアドレスを取得。
    let setting_json = SettingJson::read(true);
    let friend_ips = setting_json.friend_ips;  
    // let friend_ips = vec![String::from("127.0.0.1"),String::from("127.0.0.1")];
    // let hinmoku_code_arc = Arc::new(hinmoku_code);

    // http://取得したIPアドレス/receive_para/{hinmoku_code}
    // reqwest.getでコピーしたい品目がないか問い合わせ（並列処理）。
    let mut handlers =vec![]; 
    
    for url in friend_ips.clone() {
        let hinmoku_code = String::from(hinmoku_code);
        let handler = tokio::spawn(async move {
            let url = format!("http://{}:{}/receive_para/{}",url,SERVER_PORT,hinmoku_code.clone());
            let response_string = reqwest::get(&url).await.unwrap().text().await.unwrap();
            let res:ParaInfo = serde_json::from_str(&response_string).unwrap();
            res    
        });
        handlers.push(handler);
    }
    
    for handler in handlers {
        // 問い合わせたデータをvecで追加していく。
        if let Ok(para_info) =  handler.await{
            if para_info.is_file{
                para_infos.push(para_info)
            }
        }
    }
    para_infos
}

pub async fn receive_friend_ips()->Vec<SettingJson>{
    let mut setting_jsons:Vec<SettingJson> = vec![];  
    // setting.jsonから他の設備のIPアドレスを取得。
    let setting_json = SettingJson::read(true);
    let friend_ips = setting_json.friend_ips;
    
    #[cfg(not(debug_assertions))]
    let my_address = Config::get_my_ip_address();
    #[cfg(not(debug_assertions))]
    if let Some(index) = friend_ips.iter().position(|ip|ip==my_address){
        friend_ips.remove(index);
    }
    
    // http://取得したIPアドレス/receive_para/{hinmoku_code}
    // reqwest.getでコピーしたい品目がないか問い合わせ（並列処理）。
    let mut handlers =vec![]; 
    
    for url in friend_ips.clone() {
        let handler = tokio::spawn(async move {
            let url = format!("http://{}:{}/receive_friend_ips",url,SERVER_PORT);
            let response_string = reqwest::get(&url).await.unwrap().text().await.unwrap();
            let res:SettingJson = serde_json::from_str(&response_string).unwrap();
            res    
        });
        handlers.push(handler);
    }
    
    for handler in handlers {
        // 問い合わせたデータをvecで追加していく。
        if let Ok(serde_json) =  handler.await{setting_jsons.push(serde_json)}
    }
    
    setting_jsons
}

#[derive(Debug)]
pub struct WriteResult{
    pub bariga:bool,
    pub omote:bool,
    pub ura:bool,
}

impl WriteResult{
    pub fn new_as_all_false()->Self{
        Self { bariga: false, omote: false, ura: false }
    }
}


pub async fn put_files_several_folder(selected_para_info:&ParaInfo)->WriteResult{
    let mut write_result = WriteResult::new_as_all_false();
    let hinmoku_code = &selected_para_info.hinmoku_code;
    let bariga_file_name = &selected_para_info.bariga_file_name;
    let hyomen_file_name = &selected_para_info.hyomen_file_name;
    
    let bariga_dir_path = Path::new(BARIGA_FOLDER_PATH);
    let omote_dir_path = Path::new(OMOTE_FOLDER_PATH);
    let ura_dir_path = Path::new(URA_FOLDER_PATH);
    
    let bariga_path = bariga_dir_path.join(bariga_file_name);
    let omote_path = omote_dir_path.join(hyomen_file_name);
    let ura_path = ura_dir_path.join(hyomen_file_name);



    if bariga_dir_path.exists() {
        let bariga_shift_jis_file = ShiftjisFile::read_from_file_path(BARIGA_FOLDER_PATH,&bariga_file_name);
        bariga_shift_jis_file.write(bariga_path.to_str().unwrap());
        write_result.bariga = true;
    }
    
    if omote_dir_path.exists() {
        let bariga_shift_jis_file = ShiftjisFile::read_from_file_path(OMOTE_FOLDER_PATH,&hyomen_file_name);
        bariga_shift_jis_file.write(omote_path.to_str().unwrap());
        let mut omote_ini_file = ItemMasteINI::read(OmoteUra::Omote);
        omote_ini_file.update(&hinmoku_code, &hyomen_file_name);
        omote_ini_file.write_file();
        write_result.omote = true;
        
    }

    if ura_dir_path.exists() {
        let bariga_shift_jis_file = ShiftjisFile::read_from_file_path(URA_FOLDER_PATH,&hyomen_file_name);
        bariga_shift_jis_file.write(ura_path.to_str().unwrap());
        let mut ura_ini_file = ItemMasteINI::read(OmoteUra::Ura);
        ura_ini_file.update(&hinmoku_code, &hyomen_file_name);
        ura_ini_file.write_file();
        write_result.ura = true;
    }

    write_result



}

