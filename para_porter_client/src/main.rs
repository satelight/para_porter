#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use library::para_info::{ParaInfo,ParaKind};
mod porter;

const POST_URL:&str = "http://127.0.0.1:8000/post_para";


#[tokio::main]
async fn main() {
    
    let query = ParaInfo { 
        file_name: Some("test.txt".to_string()), 
        content: Some("tesgabagarta".to_string()), 
        para_kind:Some(ParaKind::Omote) 
    };
    porter::Porter::send_post(POST_URL, &query).await;
    // println!("{}",res);
}
