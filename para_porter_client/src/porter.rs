use library::{encode_shift_jis, para_history_json, para_info, setting_json::SettingFile};
use serde_json::json;

pub struct Porter {
    pub bariga_folder: String,
    pub omote_folder: String,
    pub ura_folder: String,
    pub bariga_file_names: Vec<String>,
    pub omote_file_names: Vec<String>,
    pub ura_file_names: Vec<String>,
}

impl Porter {
    pub fn init() -> Self {
        SettingFile::init();
        let read_file = SettingFile::read();

        Self {
            bariga_folder: read_file.bariga_folder_path,
            omote_folder: read_file.omote_folder_path,
            ura_folder: read_file.ura_folder_path,
            bariga_file_names: vec![],
            omote_file_names: vec![],
            ura_file_names: vec![],
        }
    }

    // pub fn scan_files_in_folder(){

    // }

    pub async fn send_post(&self, url: &str, query: &para_info::ParaInfo) {
        let client = reqwest::Client::new();
        client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&json!(query))
            .send()
            .await
            .unwrap();
    }
}
