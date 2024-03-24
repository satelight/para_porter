use dialoguer::Select;

#[derive(Debug,Clone, Copy)]
pub enum HomeSelectionItem {
    SeeMyFolder,
    IsThereTheParaFile,
    SendFile,
    End,
}

impl std::fmt::Display for HomeSelectionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            HomeSelectionItem::IsThereTheParaFile => write!(f,"IsThereTheParaFile"),
            HomeSelectionItem::SeeMyFolder => write!(f,"SeeMyFolder"),
            HomeSelectionItem::SendFile => write!(f,"SendFile"),
            HomeSelectionItem::End => write!(f,"End"),

        }
    }
}

pub fn show_home_menu()-> HomeSelectionItem{
    let items = vec![
        HomeSelectionItem::SeeMyFolder,
        HomeSelectionItem::IsThereTheParaFile,
        HomeSelectionItem::SendFile,
        HomeSelectionItem::End,
    ];

    let selection = Select::new()
        .with_prompt("What do you choose?")
        .items(&items)
        .interact()
        .unwrap();

    println!("You chose: {:?}", items[selection]);
    
    items[selection]
}