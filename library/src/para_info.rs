use std::path::Path;
use filetime::FileTime;
use std::fs;
use super::encode_shift_jis::ShiftjisFile;


#[derive(Debug,serde::Deserialize,serde::Serialize,Clone)]
pub enum ParaKind {
    Bariga, // バリ画用ファイル
    Hyomen,
}

#[derive(Debug,serde::Deserialize,serde::Serialize,Clone)]
pub struct ParaInfo {
    pub hinmoku_code:String,
    pub file_name:String,
    pub content:String,
    pub update_time_unix_seconds:i64,
    pub para_kind:ParaKind,
    pub machine_name:String,
}


impl ParaInfo {
    pub fn new(hinmoku_code:&str,file_name:&str,para_kind:ParaKind,machine_name:&str) -> Self{
        let content = ShiftjisFile::to_utf8(file_name);
        let meta = fs::metadata(file_name).unwrap();
        let mtime = FileTime::from_last_modification_time(&meta);
        let unix_seconds = mtime.unix_seconds();

        Self {
            hinmoku_code:String::from(hinmoku_code),
            file_name: String::from(file_name), 
            content:content.utf8_content, 
            update_time_unix_seconds: unix_seconds, 
            para_kind,
            machine_name:machine_name.to_string()
        }
    }

    pub fn write_file(&self,folder_path:&str){
        std::fs::create_dir_all(folder_path).unwrap();
        let file_name = &self.file_name.clone();
        let written_path = Path::new(folder_path).join(file_name);
        let shift_jis_file = ShiftjisFile::new(file_name,&self.content.clone());       
        shift_jis_file.write(written_path.to_str().unwrap_or(""));
    }

}

#[test]
fn filetime_test(){
    let meta = fs::metadata("t.txt").unwrap();
    let mtime = FileTime::from_last_modification_time(&meta);
    let unix_seconds = mtime.unix_seconds();
    println!("{}",unix_seconds);
}