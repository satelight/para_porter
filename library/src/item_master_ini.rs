use ini::Ini;
use crate::common_variable::URA_FOLDER_PATH;

use super::encode_shift_jis::ShiftjisFile;
use super::common_variable::{OMOTE_FOLDER_PATH,ITEMASTER_INI_PATH,OmoteUra};

#[derive(Debug)]
pub struct ItemMasteINI{
    ini:Ini,
    omote_ura:OmoteUra,
}
impl ItemMasteINI {
    pub fn read(omote_ura:OmoteUra)-> Self{
        let dir_path = match omote_ura {
            OmoteUra::Omote => OMOTE_FOLDER_PATH,
            OmoteUra::Ura => URA_FOLDER_PATH,
        };
        let shift_jis_file = ShiftjisFile::read_from_file_path(dir_path,ITEMASTER_INI_PATH);
        let utf8_string = shift_jis_file.utf8_content;
        let ini = Ini::load_from_str(&utf8_string).unwrap();
        
        Self {ini,omote_ura}
    }

    pub fn update(&mut self,hinmoku_code:&str,hyomen_file:&str){
        let hyomen_file = String::from(hyomen_file).replace(".txt", "");
        self.ini.with_section(Some(hinmoku_code)).set("FileName", hyomen_file);
    }

    pub fn write_file(&self){
        let mut content = String::new();
        for (option_str ,properties)  in self.ini.iter(){
            if let Some(s) = option_str{
                let section_name = format!("[{}]\r\n",s);
                let mut prop = String::new();
                for (key,value) in properties.iter(){
                    prop.push_str(&format!("{}={}\r\n\r\n",key,value));
                }
                let section_data = format!("{}{}",section_name.clone(),prop.clone());
                content.push_str(&section_data);
            }
        }


        let dir_path = match self.omote_ura {
            OmoteUra::Omote => OMOTE_FOLDER_PATH,
            OmoteUra::Ura => URA_FOLDER_PATH,
        };


        let item_master_path = std::path::Path::new(dir_path).join(ITEMASTER_INI_PATH);
        let shift_jis_file = ShiftjisFile::new(ITEMASTER_INI_PATH,dir_path,&content,true);
        shift_jis_file.write(item_master_path.to_str().unwrap());
    }
}

#[test]
fn item_master_test(){
    let mut ini = ItemMasteINI::read(OmoteUra::Omote);
    ini.update("test", "test(derea)");
    ini.write_file();
}