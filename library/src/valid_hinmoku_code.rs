#![allow(dead_code)]
#![allow(unused_imports)]
use regex::Regex;


/// 品目コード、バリ画のファイル名、表面のパラメータファイル名が正常か判定するストラクト
#[derive(Debug)]
pub struct VaildHinmokuCode{
    pub hinmoku_code:String,
}

impl VaildHinmokuCode {
    
    pub fn new(hinmoku_code:&str)-> Self{
        Self { hinmoku_code: hinmoku_code.to_string()  }
    }

    /// CO0008Y10,CO0008Y1A のような
    /// 数字以外の2文字,数字４文字,数字以外１文字、英数字２文字  
    /// もしくは  
    /// CO21234-A00,CO21234-A0Cのような
    /// 数字以外の2文字,数字5文字,-(ハイフン),数字以外１文字,英数字２文字
    /// のパターンの場合のみtrueを返す。
    pub fn is_hinmoku_code(&self)-> bool{
        let re = Regex::new(r"^\D{2}\d{4}\D\w{2}$|^\D{2}\d{5}-\D\w{2}$").unwrap();
        let is_match = re.is_match(&self.hinmoku_code);
        is_match
    }

    /// CO0008Y10.txt,CO0008Y1A.txt のような
    /// 数字以外の2文字,数字４文字,数字以外１文字、英数字２文字  
    /// もしくは  
    /// CO21234-A00.txt,CO21234-A0C.txtのような
    /// 数字以外の2文字,数字5文字,-(ハイフン),数字以外１文字,英数字２文字
    /// のパターンの場合のみtrueを返す。
    pub fn is_bariga_file(&self)-> bool{
        let re = Regex::new(r"^\D{2}\d{4}\D\w{2}.txt$|^\D{2}\d{5}-\D\w{2}.txt$").unwrap();
        let is_match = re.is_match(&self.hinmoku_code);
        is_match
    }

    /// CO0008Y1(sh5a0-a).txt,のような
    /// 数字以外の2文字,数字４文字,数字以外１文字、英数字1文字,(任意の文字５文字).txt  
    /// もしくは  
    /// CO21234-A00,CO21234-A0Cのような
    /// 数字以外の2文字,数字5文字,-(ハイフン),数字以外１文字,英数字1文字,(任意の文字５文字).txt
    /// のパターンの場合のみtrueを返す。
    pub fn is_hyomen_file(&self)->bool {
        let re = Regex::new(r"^\D{2}\d{4}\D\w\(.{7}\).txt$|^\D{2}\d{5}-\D\w\(.{7}\).txt$").unwrap();
        let is_match = re.is_match(&self.hinmoku_code);
        is_match
    }
}


#[test]
fn vaild_test(){
    let valid_hinmoku = VaildHinmokuCode::new("CO20008-A1A.txt");
    let is_hinmoku = valid_hinmoku.is_bariga_file();
    let hyomen_file = VaildHinmokuCode::new("CO10008-Y1(sh5a0-a).txt");
    let is_hyomen_file = hyomen_file.is_hyomen_file();
    println!("{:?}",valid_hinmoku.hinmoku_code);
    println!("is_hinmoku:{:?}",is_hinmoku);
    println!("is_hyomen_file:{:?}",is_hyomen_file);
    
}
