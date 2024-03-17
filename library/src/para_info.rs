
#[derive(Debug,serde::Deserialize,serde::Serialize)]
pub enum ParaKind {
    Bariga, // バリ画用ファイル
    Omote,
    Ura,
    ItemMaster,
}

#[derive(Debug,serde::Deserialize,serde::Serialize)]
pub struct ParaInfo {
    pub file_name:Option<String>,
    pub content:Option<String>,
    pub para_kind:Option<ParaKind>,
}