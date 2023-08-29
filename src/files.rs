
use std::{fs::{File, read_dir, self}, path::PathBuf};
use crate::app::App;

pub enum DirectoryItem{
    File((String,u64)),
    Directory(String)
    
}

pub fn get_files_for_current_directory(app: &mut App)-> Result<Vec<DirectoryItem>,std::io::Error>{
    let dir_items : Vec<PathBuf>= match fs::read_dir(app.current_directory.as_path()){
        Ok(val)=> val.map(|f| f.unwrap().path()).collect(),
        Err(err)=> return Err(err)
    };

    let  mut files:Vec<DirectoryItem>=Vec::new();
    for item in dir_items{
        let file=File::open(item.clone());
        let file_size=match file{
            Ok(file)=>(file.metadata().unwrap().len() as f64/1000.0).ceil() as u64,
            Err(err)=> 0
        };
        if item.is_file(){
            let file=DirectoryItem::File(
                (
                String::from(item.to_str().unwrap()),
                file_size
                )
            );
            files.push(file);
        }else{
            let dir = DirectoryItem::Directory(String::from(item.to_str().unwrap()));
            files.push(dir);
        }

    };
    Ok(files)
}

pub fn get_file_content(directoryItem:DirectoryItem)-> Result<String, std::io::Error>{
    match directoryItem{
        DirectoryItem::File((path,_))=>{
            let content=fs::read_to_string(path);
            content 
        },
        _=> Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Not a file"))
    }

    

}

