#[cfg(debug_assertions)]
pub const OMOTE_FOLDER_PATH:&str = "NOKENV";
#[cfg(debug_assertions)]
pub const URA_FOLDER_PATH:&str = "NOKENV";
#[cfg(debug_assertions)]
pub const BARIGA_FOLDER_PATH:&str = "Item";
#[cfg(debug_assertions)]
pub const ITEMASTER_INI_PATH:&str = "ItemMaster.INI";


#[cfg(not(debug_assertions))]
pub const OMOTE_FOLDER_PATH:&str = "D:/NOKENV";
#[cfg(not(debug_assertions))]
pub const URA_FOLDER_PATH:&str = "P:/NOKENV";
#[cfg(not(debug_assertions))]
pub const BARIGA_FOLDER_PATH:&str = "D:/Item";
#[cfg(not(debug_assertions))]
pub const ITEMASTER_INI_PATH:&str = "D:/ItemMaster.INI";

#[cfg(debug_assertions)]
pub const SERVER_PORT:u16 = 8080;
#[cfg(not(debug_assertions))]
pub const SERVER_PORT:u16 = 49153;


#[derive(Debug)]
pub enum OmoteUra{
    Omote,
    Ura,
}





