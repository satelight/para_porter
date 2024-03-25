use std::path::Path;
use filetime::FileTime;
use std::fs;
use super::encode_shift_jis::ShiftjisFile;



#[derive(Debug,serde::Deserialize,serde::Serialize,Clone)]
pub struct ParaInfo {
    pub hinmoku_code:String,
    pub file_name:String,
    pub content:String,
    pub update_time_unix_seconds:i64,
    pub machine_name:String,
    pub is_file:bool
}


impl ParaInfo {
    pub fn new(hinmoku_code:&str,dir_path:&str,file_name:&str,machine_name:&str) -> Self{
        if file_name.len() >= 1{
            let path = std::path::Path::new(dir_path).join(file_name);
            let content = ShiftjisFile::new(dir_path,file_name);
            let meta = fs::metadata(&path).unwrap();
            let mtime = FileTime::from_last_modification_time(&meta);
            let unix_seconds = mtime.unix_seconds();
    
            Self {
                hinmoku_code:String::from(hinmoku_code),
                file_name: String::from(file_name), 
                content:content.utf8_content, 
                update_time_unix_seconds: unix_seconds, 
                machine_name:machine_name.to_string(),
                is_file:true,
            }

        } else {
            Self {
                hinmoku_code:String::from(hinmoku_code),
                file_name: String::from(file_name), 
                content:String::from(""), 
                update_time_unix_seconds: 0,
                machine_name:machine_name.to_string(),
                is_file:false,
            }
        }
    }

    pub fn write_file(&self,folder_path:&str){
        std::fs::create_dir_all(folder_path).unwrap();
        let written_path = Path::new(folder_path).join(&self.file_name);
        
        let shift_jis_file = ShiftjisFile{
            file_path:written_path.to_str().unwrap().to_string(),
            file_name:self.file_name.clone(),
            utf8_content:self.content.clone(),
        };       
        shift_jis_file.write(written_path.to_str().unwrap_or(""));
    }

}

// #[test]
// fn filetime_test(){
//     let meta = fs::metadata("t.txt").unwrap();
//     let mtime = FileTime::from_last_modification_time(&meta);
//     let unix_seconds = mtime.unix_seconds();
//     println!("{}",unix_seconds);
// }