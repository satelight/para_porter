use library::encode_shift_jis::ParseParaFile;
use dialoguer::Select;
use dialoguer::Confirm;


const NOKENV:&str = "NOKENV";

pub fn see_my_folder(){
    let mut find_files = vec![];
    let dir = std::fs::read_dir(NOKENV).unwrap();
    for d_result in dir {
        let dir_entry = d_result.unwrap();
        let file_name = dir_entry.file_name().to_str().unwrap().to_string();
        let find_number = file_name.find(".txt").unwrap_or(0);
        if find_number > 0{
            // println!("file_name:{:?},find_number{:?}",file_name,find_number);
            let file_path = std::path::Path::new(NOKENV).join(file_name);
            let parse_file = ParseParaFile::new(&file_path.to_str().unwrap());
            let sunpo= parse_file.get_kijyu_sunpou();
            find_files.push(sunpo)
        }

        
        // let d = String::from(file_name).find(").txt").unwrap();
        
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


    let _ = Confirm::new()
        .with_prompt("'Y'か'N'を押すホーム画面に戻ります。")
        .interact()
        .unwrap();

}


pub fn is_there_the_para_file(file_name:&str){

}


//　ポストメソッド使うとき用para_porter_clientフォルダを削除したので保存用にコメントアウト
// pub async fn send_post(&self, url: &str, query: &para_info::ParaInfo) {
//     let client = reqwest::Client::new();
//     client
//         .post(url)
//         .header("Content-Type", "application/json")
//         .json(&json!(query))
//         .send()
//         .await
//         .unwrap();
// }