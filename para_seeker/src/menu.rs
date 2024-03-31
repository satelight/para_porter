use dialoguer::{Select,Input};
use library::para_info::ParaInfo;

#[derive(Debug,Clone, Copy)]
pub enum HomeSelectionItem {
    SearchMyFolder,
    SearchOtherMachine,
    FriendIPs,
    End,
}

impl std::fmt::Display for HomeSelectionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            HomeSelectionItem::SearchMyFolder => write!(f,"この設備のファイル一覧"),
            HomeSelectionItem::SearchOtherMachine => write!(f,"他の設備からコピーする"),
            HomeSelectionItem::FriendIPs =>write!(f,"他の設備のIPアドレス取得する。"),
            HomeSelectionItem::End => write!(f,"終了"),
        }
    }
}

pub struct Menu;
impl Menu {
    pub async fn home()-> HomeSelectionItem{
        let items = vec![
            HomeSelectionItem::SearchMyFolder,
            HomeSelectionItem::SearchOtherMachine,
            HomeSelectionItem::FriendIPs,
            HomeSelectionItem::End,
        ];
    
        let selection = Select::new()
            .with_prompt("どの処理を行いますか？")
            .items(&items)
            .interact()
            .unwrap();
    
        // println!("You chose: {:?}", items[selection]);
        
        items[selection]
    }

    pub async fn search_hinmoku_menu_1st()-> String{
        let name: String = Input::new()
        .with_prompt("検索したい品目コードを入力してください。例：CO1234A00,CO21234-B0A")
        .interact_text()
        .unwrap();

        name.to_uppercase()
    }

    #[allow(dead_code)]
    pub async fn search_hinmoku_menu_2nd(para_infos:&Vec<ParaInfo>)-> Option<ParaInfo>{
        
        
        match para_infos.is_empty() {
            true => None,
            false => {
                let mut select_menus = vec![];

                for para_info in para_infos.iter(){
                    let machine_name = &para_info.machine_name;
                    let hyomen_file_name = &para_info.hyomen_file_name;
                    let select_menu = format!("設備名:{} ファイル名:{}",machine_name,hyomen_file_name);
                    select_menus.push(select_menu);
                }
                select_menus.push(String::from("戻る"));
                //　選んだら、そのデータを反映させる。
                // バリ画、表、裏、表側のItemMaster.INIと裏のItemMaster.INIに書き込む。
                // 終了
                
                let selection = Select::new()
                    .with_prompt("どの設備のファイルをコピーしますか？")
                    .items(&select_menus)
                    .interact()
                    .unwrap();
            
                if select_menus[selection] == String::from("戻る"){
                    None
                }else{
                    Some(para_infos[selection].clone())
                }
            }
        }

    }
}

