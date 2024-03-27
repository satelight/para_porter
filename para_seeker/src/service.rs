use std::f32::consts::E;

use library::encode_shift_jis::ParseParaFile;
#[allow(unused_imports)]
use dialoguer::Select;
use dialoguer::Confirm;
use library::common_variable::OMOTE_FOLDER_PATH;



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
                        let file_path = std::path::Path::new(OMOTE_FOLDER_PATH).join(file_name);
                        let parse_file = ParseParaFile::new(OMOTE_FOLDER_PATH,&file_path.to_str().unwrap());
                        let sunpo= parse_file.get_kijyu_sunpou();
                        return find_files.push(sunpo);
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

#[allow(dead_code)]
pub async fn is_there_the_para_file(_hinmoku_code:&str){
    // setting.jsonから他の設備のIPアドレスを取得。
    // http://取得したIPアドレス/receive_para/{hinmoku_code}
    // reqwest.getでコピーしたい品目がないか問い合わせ（並列処理）。
    // 問い合わせたデータをvecで追加していく。
    // 更新時間が新しいものを最初にする。
    // どのデータを利用するかを選べるように一覧にする。
    //　選んだら、そのデータを反映させる。
    // バリ画、表、裏、表側のItemMaster.INIと裏のItemMaster.INIに書き込む。
    // 終了
}