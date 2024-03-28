use dialoguer::{Select,Input};

#[derive(Debug,Clone, Copy)]
pub enum HomeSelectionItem {
    SearchMyFolder,
    SearchOtherMachine,
    End,
}

impl std::fmt::Display for HomeSelectionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            HomeSelectionItem::SearchMyFolder => write!(f,"この設備のファイル一覧"),
            HomeSelectionItem::SearchOtherMachine => write!(f,"他の設備からコピーする"),
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

        String::from(name)
    }


    pub async fn search_hinmoku_menu_2nd()-> String{
        //　選んだら、そのデータを反映させる。
    // バリ画、表、裏、表側のItemMaster.INIと裏のItemMaster.INIに書き込む。
    // 終了
        String::from("")
    }
}

