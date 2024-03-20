use std::collections::HashMap;

/// settingフォルダパス
const SETTING_FOLDER:&str = "setting";

/// パラメータのメタ情報一覧を保存するpara_meta.jsonのパス
const PARAMETA_JSON:&str = "para_history.json";


pub struct ParaHistoryJson;

impl ParaHistoryJson{
    pub fn init(){
        let folder_path = std::path::Path::new(SETTING_FOLDER);
        let file_path = folder_path.join(PARAMETA_JSON);
        if !file_path.exists() {
            let f = std::fs::File::create(&file_path).unwrap();
            let mut content = HashMap::new();
            let txt_files = ["test.txt","test2.txt"];
    
            for txt_file in txt_files {
                content.insert(txt_file,ParaHistoryInfo::new(txt_file,0));
            }
            serde_json::to_writer_pretty(f, &content).unwrap();
        }
    }
    
    #[allow(dead_code)]
    pub fn read()-> ParaHistoryContent{
        let folder_path = std::path::Path::new(SETTING_FOLDER);
        let file_path = folder_path.join(PARAMETA_JSON);
        let rdr = std::fs::File::open(file_path).unwrap();
        serde_json::from_reader(rdr).unwrap()
    }
    
    #[allow(dead_code)]
    pub fn write(value:&ParaHistoryInfo){
        let folder_path = std::path::Path::new(SETTING_FOLDER);
        let file_path = folder_path.join(PARAMETA_JSON);
        let f = std::fs::File::create(file_path).unwrap();
        serde_json::to_writer_pretty(f, value).unwrap();
    }
    
    #[allow(dead_code)]
    pub fn is_key_file(key:&str,meta_file_content:ParaHistoryContent)->bool{
        meta_file_content.get(key).is_some()
    }
}

pub type ParaHistoryContent = HashMap<String,ParaHistoryInfo>;


#[derive(Debug,serde::Serialize,serde::Deserialize)]
pub struct ParaHistoryInfo{
    pub file_path: String,
    pub update_date_time:i64,
}

impl ParaHistoryInfo {
    fn new(file_path:&str,update_date_time:i64)->Self {
        Self { 
            file_path:file_path.to_string(), 
            update_date_time,
        }
    }
}