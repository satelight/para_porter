// use std::thread;
// use std::time::Duration;
// use console::Term;
// use console::style;

use anyhow::Ok;
mod home_menu;
mod service;
use home_menu::{HomeSelectionItem,show_home_menu};




#[tokio::main]
async fn main()->anyhow::Result<(),anyhow::Error>{
    loop {
        let select_item = show_home_menu();
        match select_item {
            HomeSelectionItem::SeeMyFolder => service::see_my_folder(),
            HomeSelectionItem::IsThereTheParaFile => {
                let file_name = "CO0013Q9";
                service::is_there_the_para_file(file_name);
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