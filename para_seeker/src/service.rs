use std::sync::Arc;

use library::encode_shift_jis::ParseParaFile;
#[allow(unused_imports)]
use dialoguer::Select;
use dialoguer::Confirm;
use library::common_variable::OMOTE_FOLDER_PATH;
use library::para_info::ParaInfo;
use library::setting::SettingJson;

pub fn see_my_folder(){
    let mut find_files = vec![];
    match std::fs::read_dir(OMOTE_FOLDER_PATH){
        Ok(dir) =>{
            for d_result in dir {
                let dir_entry = d_result.unwrap();
                let file_name = dir_entry.file_name().to_str().unwrap().to_string();
                match file_name.find(".txt"){
                    Some(_) => {
                        // println!("file_name:{:?},find_number{:?}",file_name,find_number);
                        let parse_file = ParseParaFile::new(OMOTE_FOLDER_PATH,&file_name);
                        match parse_file.get_kijyu_sunpou(){
                            Some(sunpo) => find_files.push(sunpo),
                            None => {},
                        }
                    },
                    None =>{},
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
    // let setting_json = SettingJson::read();
    // let friend_ips = setting_json.friend_ips; 
    let friend_ips = vec![String::from("127.0.0.1"),String::from("127.0.0.1")];
    // let hinmoku_code_arc = Arc::new(hinmoku_code);

    // http://取得したIPアドレス/receive_para/{hinmoku_code}
    // reqwest.getでコピーしたい品目がないか問い合わせ（並列処理）。
    let mut handlers =vec![]; 
    
    for url in friend_ips.clone() {
        let hinmoku_code = String::from(hinmoku_code);
        let handler = tokio::spawn(async move {
            let url = format!("http://{}:8080/receive_para/{}",url,hinmoku_code.clone());
            let response_string = reqwest::get(&url).await.unwrap().text().await.unwrap();
            let res:ParaInfo = serde_json::from_str(&response_string).unwrap();
            return res;    
        });
        
        handlers.push(handler);
    }
    
    for handler in handlers {
        // 問い合わせたデータをvecで追加していく。
        match handler.await{
            Ok(para_info) => para_infos.push(para_info),
            Err(_) => {},
        }
    }

    for para in para_infos.iter(){
        println!("{:?}:{}",para.hinmoku_code,para.is_file);
    }
    para_infos
}