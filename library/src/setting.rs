const SETTING_DIR_PATH:&str = "setting";
pub const SETTING_JSON_PATH:&str = "setting.json";

#[derive(Debug,serde::Serialize,serde::Deserialize)]
pub struct SettingJson{
    pub machine_name:String,
    pub friend_ips:Vec<String>,
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
                friend_ips: vec![],
            };

            serde_json::to_writer_pretty(f, &setting_file).unwrap();
        }

    }

    pub fn read(init_ok:bool)-> Self {
        if init_ok {SettingJson::init();}

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

// #[test]
// fn config_test(){
//     let ip_address = Config::get_my_ip_address();
//     println!("{:?}",ip_address);
// }