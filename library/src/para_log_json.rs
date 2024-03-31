use crate::para_info::ParaInfo;
use chrono::{DateTime, Local};

/// settingフォルダパス
const SETTING_FOLDER: &str = "setting";

/// パラメータのメタ情報一覧を保存するpara_porter_log.jsonのパス
const LOG_JSON: &str = "para_porter_log.json";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ParaLog {
    pub hinmoku_code:String,
    pub bariga_file_name: String,
    pub hyomen_file_name: String,
    pub machine_name: String,
    pub arrival_time: String,
}

impl ParaLog {
    pub fn new(hinmoku_code:&str,bariga_file_name: &str,hyomen_file_name:&str,machine_name: &str) -> Self {
        let arrival_time_raw: DateTime<Local> = Local::now();
        let arrival_time_format = arrival_time_raw.format("%Y-%m-%d %H:%M:%S");
        Self {
            hinmoku_code:hinmoku_code.to_string(),
            bariga_file_name: bariga_file_name.to_string(),
            hyomen_file_name: hyomen_file_name.to_string(),
            machine_name: machine_name.to_string(),
            arrival_time: arrival_time_format.to_string(),
        }
    }

    pub fn read_from_static_path() -> Vec<Self> {
        let folder_path = std::path::Path::new(SETTING_FOLDER);
        let file_path = folder_path.join(LOG_JSON);
        match std::fs::File::open(file_path){
            Ok(rdr) => {
                match serde_json::from_reader(rdr){
                    Ok(para_log_json) => para_log_json,
                    Err(_) => vec![],
                }
            },
            Err(_) => vec![],
        }
    }

    pub fn write_file(para_info:&ParaInfo){
        let folder_path = std::path::Path::new(SETTING_FOLDER);
        let file_path = folder_path.join(LOG_JSON);
        let mut contents = ParaLog::read_from_static_path();
        
        let hinmoku_code = &para_info.hinmoku_code;
        let bariga_file_name = &para_info.bariga_file_name;
        let hyomen_file_name =&para_info.hyomen_file_name;
        let machine_name = &para_info.machine_name;
        let content = ParaLog::new(&hinmoku_code,&bariga_file_name,&hyomen_file_name,&machine_name);
        contents.push(content);
        
        let f = std::fs::File::create(file_path).unwrap();
        serde_json::to_writer_pretty(f, &contents).unwrap();
    }
}
