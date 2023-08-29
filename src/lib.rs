mod app;mod files;

#[cfg(test)]
mod tests {
    use crate::files::{get_file_content,DirectoryItem};




    #[test]
    fn test_file_content() {
        // match 
        // println!("file path {}",std::fs::read_to_string("./src/main.rs")));
        let file:DirectoryItem= DirectoryItem::File(("./src/text.txt".to_string(),0));

        let content:String= match get_file_content(file){
            Ok(content)=>content,
            Err(content)=>"".to_string()
        };

        println!("go content  :{} length :{}",content,content.len()); 
        let mut substrings:Vec<String>=Vec::new();
        let substring_size:usize=5;

        for i in (0..content.len()).step_by(substring_size){
            let substring:String=content.chars().skip(i).take(substring_size).collect();
            substrings.push(substring);
        }
        println!("Substrings: {:?}", substrings);


        // assert_eq!(2 + 2, 4);
    }

}
