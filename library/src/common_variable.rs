#[cfg(debug_assertions)]
pub const OMOTE_FOLDER_PATH:&str = "NOKENV";
#[cfg(debug_assertions)]
pub const URA_FOLDER_PATH:&str = "NOKENV";
#[cfg(debug_assertions)]
pub const BARIGA_FOLDER_PATH:&str = "Item";

#[cfg(not(debug_assertions))]
pub const OMOTE_FOLDER_PATH:&str = "D:/NOKENV";
#[cfg(not(debug_assertions))]
pub const URA_FOLDER_PATH:&str = "P:/NOKENV";
#[cfg(not(debug_assertions))]
pub const BARIGA_FOLDER_PATH:&str = "D:/Item";



