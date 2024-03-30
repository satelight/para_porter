use std::collections::HashMap;
use ini::Ini;
use super::encode_shift_jis::ShiftjisFile;
use super::common_variable::{OMOTE_FOLDER_PATH,ITEMASTER_INI_PATH};

pub struct ItemMasteINI;
impl ItemMasteINI {
    pub fn read()->Ini{
        let shift_jis_file = ShiftjisFile::new(OMOTE_FOLDER_PATH,ITEMASTER_INI_PATH);
        let utf8_string = shift_jis_file.utf8_content;
        let ini_file = Ini::load_from_str(&utf8_string).unwrap();
        ini_file
    }

    pub fn update(ini:&mut Ini,hinmoku_code:&str,hyomen_file:&str){
        let hyomen_file = String::from(hyomen_file).replace(".txt", "");
        ini.with_section(Some(hinmoku_code)).set("FileName", hyomen_file);
    }

    pub fn write_file(ini:Ini){
        let mut content = String::new();
        for (option_str ,properties)  in ini.iter(){
            let section_name = format!("[{}]\n",option_str.unwrap());
            let prop = String::new();
            for (key,value) in properties.iter(){
                let prop = format!("{}={}\n\n",key,value);
            }
            let section_data = format!("{}{}",section_name.clone(),prop.clone());
            content.push_str(&section_data);
        }
        let item_master_path = std::path::Path::new(OMOTE_FOLDER_PATH).join(ITEMASTER_INI_PATH);

        let shift_jis_file = ShiftjisFile::new(OMOTE_FOLDER_PATH,ITEMASTER_INI_PATH);
        shift_jis_file.write(item_master_path.to_str().unwrap());
    }
}

#[test]
fn item_master_test(){
    let ini = ItemMasteINI::read();
    println!("{:?}",ini);
}