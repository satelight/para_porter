const SETTING_FOLDER:&str = "setting";
pub const SETTING_JSON:&str = "setting.json";

#[derive(Debug,serde::Serialize,serde::Deserialize)]
pub struct SettingFile{
    pub bariga_folder_path:String,
    pub omote_folder_path:String,
    pub ura_folder_path:String,
    pub friend_ips:Vec<String>,
}


impl SettingFile {
    pub fn is_file()-> bool{
        let setting_folder = std::path::Path::new(SETTING_FOLDER);
        let setting_json_path = setting_folder.join(SETTING_JSON);
        setting_json_path.exists() 
    }

    pub fn init(){
        let setting_folder = std::path::Path::new(SETTING_FOLDER);
        let setting_json_path = setting_folder.join(SETTING_JSON);

        if !setting_json_path.exists(){
            std::fs::create_dir_all(setting_folder).unwrap();
            let f = std::fs::File::create(setting_json_path).unwrap();
            let setting_file:SettingFile = SettingFile { 
                bariga_folder_path: "D:/Item".to_string(), 
                omote_folder_path:"D:/NOKENV/".to_string(), 
                ura_folder_path: "P:/NOKENV/".to_string(),
                friend_ips: vec![],
            };

            serde_json::to_writer_pretty(f, &setting_file).unwrap();
        }

    }

    pub fn read()-> Self {
        let setting_folder = std::path::Path::new(SETTING_FOLDER);
        let setting_json_path = setting_folder.join(SETTING_JSON);

        let s = std::fs::read_to_string(setting_json_path).unwrap();
        serde_json::from_str(&s).unwrap()
    }



}