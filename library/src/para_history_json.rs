use crate::para_info::ParaInfo;
use chrono::{DateTime, Local};
use std::{collections::HashMap, iter::zip};

/// settingフォルダパス
const SETTING_FOLDER: &str = "setting";

/// パラメータのメタ情報一覧を保存するpara_history.jsonのパス
const PARAMETA_JSON: &str = "para_history.json";

pub struct ParaHistoryJson;

impl ParaHistoryJson {
    pub fn init() {
        let folder_path = std::path::Path::new(SETTING_FOLDER);
        let file_path = folder_path.join(PARAMETA_JSON);
        if !file_path.exists() {
            let f = std::fs::File::create(&file_path).unwrap();
            let mut content = HashMap::new();

            let hinmoku_codes = ["A.example"];
            let txt_files = ["example(sh5a-a).txt"];
            let arrival_time_raw: DateTime<Local> = Local::now();
            let arrival_time_format = arrival_time_raw.format("%Y-%m-%d %H:%M:%S");
            
            for (himoku_code, txt_file) in zip(hinmoku_codes, txt_files) {
                content.insert(
                    himoku_code, 
                    ParaHistoryInfo {
                        file_name: String::from(txt_file),
                        update_time:0,
                        original_machine_name: String::from("DV999"),
                        arrival_time: arrival_time_format.to_string(),
                        comment: vec![
                            String::from("説明用のデータです。"),
                            String::from("'file_name'はパラメータファイル名です。自動で入力されます。"),
                            String::from("'update_time'はファイル更新日です。自動で入力されます。"),
                            String::from("'original_machine_name'はファイル送信元の設備の号機名です。自動で入力されます。"),
                            String::from("'comment'は注意書きのようにご利用ください。"),
                        ],
                    }
                );
            }
            serde_json::to_writer_pretty(f, &content).unwrap();
        }
    }

    pub fn read() -> ParaHistoryContent {
        let folder_path = std::path::Path::new(SETTING_FOLDER);
        let file_path = folder_path.join(PARAMETA_JSON);
        let rdr = std::fs::File::open(file_path).unwrap();
        serde_json::from_reader(rdr).unwrap()
    }

    pub fn write(para_info: &ParaInfo) {
        let mut json_content = self::ParaHistoryJson::read();
        json_content.insert(
            para_info.hinmoku_code.clone(),
            ParaHistoryInfo::new(
                &para_info.file_name,
                para_info.update_time_unix_seconds,
                &para_info.machine_name,
            ),
        );
        let folder_path = std::path::Path::new(SETTING_FOLDER);
        let file_path = folder_path.join(PARAMETA_JSON);
        let f = std::fs::File::create(file_path).unwrap();
        serde_json::to_writer_pretty(f, &json_content).unwrap();
    }

    pub fn is_hinmoku_code_key(key: &str, meta_file_content: ParaHistoryContent) -> bool {
        meta_file_content.get(key).is_some()
    }
}

/// Hashmapのkey部分は品目コードとなる。
pub type ParaHistoryContent = HashMap<String, ParaHistoryInfo>;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ParaHistoryInfo {
    pub file_name: String,
    pub update_time: i64,
    pub original_machine_name: String,
    pub arrival_time: String,
    pub comment: Vec<String>,
}

impl ParaHistoryInfo {
    pub fn new(file_name: &str, update_time: i64, original_machine_name: &str) -> Self {
        let arrival_time_raw: DateTime<Local> = Local::now();
        let arrival_time_format = arrival_time_raw.format("%Y-%m-%d %H:%M:%S");
        Self {
            file_name: file_name.to_string(),
            update_time,
            original_machine_name: original_machine_name.to_string(),
            arrival_time: arrival_time_format.to_string(),
            comment: vec![],
        }
    }
}
