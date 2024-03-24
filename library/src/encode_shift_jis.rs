use std::collections::HashMap;
use ini::Ini;

use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct ShiftjisFile {
    pub file_name:String,
    pub utf8_content:String,
}


impl ShiftjisFile {
    pub fn new(file_name:&str)->Self{
        let file_data = std::fs::read(file_name).unwrap();
        let (cow,_,_) = encoding_rs::SHIFT_JIS.decode(&file_data);
        Self { 
            file_name:file_name.to_string(), 
            utf8_content:cow.to_string(),
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
    pub fn new(file_name:&str)-> Self{
        let shift_jis_file = ShiftjisFile::new(file_name);
        let mut insert_hashmap = HashMap::new();        
        let conf = Ini::load_from_str(&shift_jis_file.utf8_content).unwrap();
        for (option_section,properies) in conf.iter(){
            let mut hash_content = HashMap::new();
            match option_section {
                Some(section_name) => {
                    for (key,value) in properies.iter(){
                        hash_content.insert(key.to_string(), value.to_string());
                    }
                    insert_hashmap.insert(section_name.to_string(), hash_content);
                },
                None => {}
            }
        }
        
        // insert_hashmap.insert(k, v)
        Self { file_name: file_name.to_string(), content:insert_hashmap }
    }

    pub fn get_kijyu_sunpou(&self)->Sunpo{
        let hash_map:HashMap<String,String> = self.content.get("基準寸法").unwrap().clone();
        let naikei = hash_map.get("内径基準値").unwrap().parse::<f32>().unwrap();
        let gaikei = hash_map.get("外径基準値").unwrap().parse::<f32>().unwrap();
        let hutosa = hash_map.get("リング幅基準値").unwrap().parse::<f32>().unwrap();

        Sunpo { file_path:self.file_name.clone(), naikei, gaikei, hutosa}
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

    let para_file = ParseParaFile::new("NOKENV/CO0013Q9(mh5a0-a).txt");
    println!("{:?}",para_file.get_kijyu_sunpou());
}