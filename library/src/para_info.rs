use std::path::Path;
use filetime::FileTime;
use std::fs;
use string_py::StrLikePy;

use super::encode_shift_jis::ShiftjisFile;
use super::common_variable::{OMOTE_FOLDER_PATH,BARIGA_FOLDER_PATH,URA_FOLDER_PATH};



#[derive(Debug,serde::Deserialize,serde::Serialize,Clone)]
pub struct ParaInfo {
    pub hinmoku_code:String,
    pub bariga_file_name:String,
    pub hyomen_file_name:String,
    pub bariga_content:String,
    pub hyomen_content:String,
    pub update_time_unix_seconds:i64,
    pub machine_name:String,
    pub is_file:bool,
    pub address:String,
}


impl ParaInfo {
    pub fn new(hinmoku_code:&str,machine_name:&str,address:&str) -> Self{
            let mut bariga_file_name = String::from(hinmoku_code);
            bariga_file_name.push_str(".txt");
            let bariga_file_name = bariga_file_name.to_ascii_uppercase();
            let hyomen_target_hinmoku = StrLikePy::new(hinmoku_code);
            let hyomen_target_hinmoku = hyomen_target_hinmoku.slice(0,-1);
            // バリ画ファイル
            let path = std::path::Path::new(BARIGA_FOLDER_PATH).join(&bariga_file_name);
            let bariga_file = ShiftjisFile::new(BARIGA_FOLDER_PATH,&bariga_file_name);
            let meta = fs::metadata(&path).unwrap();
            let mtime = FileTime::from_last_modification_time(&meta);
            let unix_seconds = mtime.unix_seconds();

            // 表面ファイル
            let read_dir = std::fs::read_dir(OMOTE_FOLDER_PATH).unwrap();
            let mut hyomen_file_name = String::new();
            for dir_result in read_dir {
                let dir_entry = dir_result.unwrap();

                let original_file_name = dir_entry.file_name().to_str().unwrap().to_string();
                let upper_case_file_name = original_file_name.clone().to_uppercase();

                match upper_case_file_name.find(&hyomen_target_hinmoku){
                    Some(_) => {
                        
                        hyomen_file_name = original_file_name;
                        break;
                    },
                    None => {},
                }
            }

            let mut hyomen_content = String::new();
            if !hyomen_file_name.is_empty(){
                let hyomen_file = ShiftjisFile::new(OMOTE_FOLDER_PATH,&hyomen_file_name);
                hyomen_content = hyomen_file.utf8_content;
            }
            
            Self {
                hinmoku_code:String::from(hinmoku_code),
                bariga_file_name, 
                bariga_content:bariga_file.utf8_content,
                hyomen_file_name,
                hyomen_content,
                update_time_unix_seconds: unix_seconds, 
                machine_name:machine_name.to_string(),
                is_file:true,
                address:address.to_string(),
            }
    }

    pub fn write_file(&self){
        
        // バリ画ファイル書き込む
        std::fs::create_dir_all(BARIGA_FOLDER_PATH).unwrap();
        let written_path = Path::new(BARIGA_FOLDER_PATH).join(&self.bariga_file_name);
        
        let shift_jis_file = ShiftjisFile{
            file_path:written_path.to_str().unwrap().to_string(),
            file_name:self.bariga_file_name.clone(),
            utf8_content:self.bariga_content.clone(),
        };       
        shift_jis_file.write(written_path.to_str().unwrap_or(""));

        //表面ファイルを書き込む
        let hyomen_folder_paths = [OMOTE_FOLDER_PATH,URA_FOLDER_PATH];
        for hyomen_folder_path in hyomen_folder_paths.iter(){
            std::fs::create_dir_all(hyomen_folder_path).unwrap();
            let written_hyomen_path = Path::new(hyomen_folder_path).join(&self.hyomen_file_name);
                
            let shift_jis_file = ShiftjisFile{
                file_path:written_path.to_str().unwrap().to_string(),
                file_name:self.hyomen_file_name.clone(),
                utf8_content:self.hyomen_content.clone(),
            };       
            shift_jis_file.write(written_hyomen_path.to_str().unwrap_or(""));
        }

    }

}

// #[test]
// fn filetime_test(){
//     let meta = fs::metadata("t.txt").unwrap();
//     let mtime = FileTime::from_last_modification_time(&meta);
//     let unix_seconds = mtime.unix_seconds();
//     println!("{}",unix_seconds);
// }