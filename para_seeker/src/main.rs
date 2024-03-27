// use std::thread;
// use std::time::Duration;
// use console::Term;
// use console::style;

use anyhow::Ok;
mod menu;
mod service;
use menu::{HomeSelectionItem,Menu};




#[tokio::main]
async fn main()->anyhow::Result<(),anyhow::Error>{
    loop {
        let select_item = Menu::home().await;
        match select_item {
            HomeSelectionItem::SearchMyFolder => service::see_my_folder(),
            HomeSelectionItem::SearchOtherMachine => {
                let hinmoku_code = Menu::search_hinmoku_menu_1st().await;
                println!("hinmoku_code:{:?}",hinmoku_code);
                service::is_there_the_para_file(&hinmoku_code).await;
            },
            HomeSelectionItem::End => break,
            _ => {}
        }
    }

    // let term = Term::stdout();
    // term.write_line("Hello World!")?;
    // thread::sleep(Duration::from_millis(2000));
    // println!("This is {} neat", style("quite").cyan());
    // term.clear_line()?;
    Ok(())
}