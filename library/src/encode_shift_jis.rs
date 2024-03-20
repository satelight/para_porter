#[derive(Debug,serde::Deserialize,serde::Serialize)]
pub struct ShiftjisFile {
    pub file_name:String,
    pub utf8_content:String,
}


impl ShiftjisFile {
    pub fn new(file_name:&str,utf_content:&str)-> Self{
        Self { file_name:file_name.to_string(), utf8_content: utf_content.to_string() }
    }

    pub fn to_utf8(file_name:&str)->Self{
        let file_data = std::fs::read(file_name).unwrap();
        let (cow,_,_) = encoding_rs::SHIFT_JIS.decode(&file_data);
        Self { 
            file_name:file_name.to_string(), 
            utf8_content:cow.to_string(),
        }
    }

    pub fn write(&self,to_path:&str)-> bool{
        let (encode,_,_) = encoding_rs::SHIFT_JIS.encode(&self.utf8_content);
        std::fs::write(to_path, encode).unwrap();
        true
    }
}

// #[test]
// fn shiftjis_test(){
//     let file_name = "sample/CO0013Q9(mh5a0-a).txt";
//     let shift_jis_file = ShiftjisFile::to_utf8(file_name);
//     shift_jis_file.write("CO0013-Q1.txt");
//     println!("{:?}",shift_jis_file);
//     // ShiftJisFile::to_utf8(file_name, shift_jis_file)
// }