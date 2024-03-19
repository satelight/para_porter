use std::path::Path;
use super::encode_shift_jis::ShiftjisFile;

#[derive(Debug,serde::Deserialize,serde::Serialize,Clone)]
pub enum ParaKind {
    Bariga, // バリ画用ファイル
    Omote,
    Ura,
}

#[derive(Debug,serde::Deserialize,serde::Serialize,Clone)]
pub struct ParaInfo {
    pub file_name:Option<String>,
    pub content:Option<String>,
    pub para_kind:Option<ParaKind>,
}


impl ParaInfo {
    pub fn write_file(&self,folder_path:&str){
        let file_name = &self.file_name.clone().unwrap_or("".to_string());
        let written_path = Path::new(folder_path).join(file_name);
        let shift_jis_file = ShiftjisFile::new(&file_name,&self.content.clone().unwrap_or("".to_string()));       
        shift_jis_file.write(&written_path.to_str().unwrap_or(""));
    }

}