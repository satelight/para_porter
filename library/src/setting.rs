const SETTING_DIR_PATH:&str = "setting";
pub const SETTING_JSON_PATH:&str = "setting.json";

#[derive(Debug,serde::Serialize,serde::Deserialize)]
pub struct SettingJson{
    pub machine_name:String,
    pub bariga_folder_path:String,
    pub omote_folder_path:String,
    pub ura_folder_path:String,
    pub friend_ips:Vec<String>,
    pub no_target_hinmoku_code:Vec<String>,
}


impl SettingJson {
    pub fn is_file()-> bool{
        let setting_folder = std::path::Path::new(SETTING_DIR_PATH);
        let setting_json_path = setting_folder.join(SETTING_JSON_PATH);
        setting_json_path.exists() 
    }

    pub fn init(){
        let setting_folder = std::path::Path::new(SETTING_DIR_PATH);
        let setting_json_path = setting_folder.join(SETTING_JSON_PATH);

        if !setting_json_path.exists(){
            std::fs::create_dir_all(setting_folder).unwrap();
            let f = std::fs::File::create(setting_json_path).unwrap();
            let setting_file:SettingJson = SettingJson { 
                machine_name:"DV999".to_string(),
                bariga_folder_path: "D:/Item".to_string(), 
                omote_folder_path:"D:/NOKENV/".to_string(), 
                ura_folder_path: "P:/NOKENV/".to_string(),
                friend_ips: vec![],
                no_target_hinmoku_code:vec![],
            };

            serde_json::to_writer_pretty(f, &setting_file).unwrap();
        }

    }

    pub fn read()-> Self {
        let setting_folder = std::path::Path::new(SETTING_DIR_PATH);
        let setting_json_path = setting_folder.join(SETTING_JSON_PATH);

        let s = std::fs::read_to_string(setting_json_path).unwrap();
        let setting_file:SettingJson = serde_json::from_str(&s).unwrap();
        
        setting_file
    }
}

pub struct Config;

impl Config {
    pub fn get_my_ip_address()-> String{
        let address = local_ip_address::local_ip().unwrap();
        address.to_string()
    }
}

#[test]
fn config_test(){
    let ip_address = Config::get_my_ip_address();
    println!("{:?}",ip_address);
}