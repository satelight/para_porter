use std::collections::HashMap;
use ini::Ini;

use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct ShiftjisFile {
    pub file_path:String,
    pub file_name:String,
    pub utf8_content:String,
    pub is_file:bool,
}


impl ShiftjisFile {
    pub fn new(dir_path:&str,file_name:&str)->Self{
        let file_path = std::path::Path::new(dir_path).join(file_name);
        match std::fs::read(&file_path){
            Ok(file_data) =>{
                let (cow,_,_) = encoding_rs::SHIFT_JIS.decode(&file_data);
                Self {
                    file_path:file_path.to_str().unwrap().to_string(),
                    file_name:file_name.to_string(), 
                    utf8_content:cow.to_string(),
                    is_file:true,
                }
            },
            Err(_) =>{
                Self {
                file_path:file_path.to_str().unwrap().to_string(),
                file_name:file_name.to_string(), 
                utf8_content:String::from(""),
                is_file:false,
            }}
        }
    }

    pub fn write(&self,to_path:&str)-> bool{
        let (encode,_,_) = encoding_rs::SHIFT_JIS.encode(&self.utf8_content);
        std::fs::write(to_path, encode).unwrap();
        true
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ParseParaFile{
    pub file_name:String,
    pub content:HashMap<String,HashMap<String,String>>
}

impl ParseParaFile {
    pub fn new(dir_path:&str,file_name:&str)->Self{
        let shift_jis_file = ShiftjisFile::new(dir_path,file_name);
        let mut insert_hashmap = HashMap::new();        
        let conf = Ini::load_from_str(&shift_jis_file.utf8_content).unwrap();
        for (option_section,properies) in conf.iter(){
            let mut hash_content = HashMap::new();
            if let Some(section_name) = option_section {
                for (key,value) in properies.iter(){
                    hash_content.insert(key.to_string(), value.to_string());
                }
                insert_hashmap.insert(section_name.to_string(), hash_content);
            };
        }
        
        // insert_hashmap.insert(k, v)
        Self { file_name: file_name.to_string(), content:insert_hashmap }
    }

    pub fn get_kijyu_sunpou(&self)-> Option<Sunpo>{
        if self.content.is_empty() {
            None
        }else {    
            let hash_map:HashMap<String,String> = self.content.get("基準寸法")?.clone();
            let naikei = hash_map.get("内径基準値")?.parse::<f32>().unwrap_or(0.0);
            let gaikei = hash_map.get("外径基準値")?.parse::<f32>().unwrap_or(0.0);
            let hutosa = hash_map.get("リング幅基準値")?.parse::<f32>().unwrap_or(0.0);
            Some(Sunpo { file_path:self.file_name.clone(), naikei, gaikei, hutosa})
        }

    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Sunpo{
    pub file_path:String,
    pub naikei:f32,
    pub gaikei:f32,
    pub hutosa:f32,
}

#[test]
fn shiftjis_test(){
    // let file_name = "sample/CO0013Q9(mh5a0-a).txt";
    // let shift_jis_file = ShiftjisFile::to_utf8(file_name);
    // shift_jis_file.write("CO0013-Q1.txt");
    // println!("{:?}",shift_jis_file);
    // ShiftJisFile::to_utf8(file_name, shift_jis_file)

    let para_file = ParseParaFile::new("NOKENV","CO0013Q9(mh5a0-a).txt");
    println!("{:?}",para_file.get_kijyu_sunpou());
}