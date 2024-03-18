use library::{encode_shift_jis,para_history_json,para_info};
use serde_json::json;

pub struct Porter;

impl Porter {
    // pub fn scan_files_in_folder(){

    // }

    pub async fn send_post(url:&str,query:&para_info::ParaInfo){
        let client = reqwest::Client::new();
        client.post(url)
            .header("Content-Type", "application/json")
            .json(&json!(query)).send().await.unwrap();
    }
}