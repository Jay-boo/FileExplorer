use std::fs::DirEntry;
use std::path::PathBuf;
use std::{io::Stdout, array::from_fn};
use std::{path, fs, env};

use termion::raw::RawTerminal;
use tui::{Terminal, backend::TermionBackend};

use crate::files::DirectoryItem;

 pub struct App {
    pub current_directory:path::PathBuf,
    pub terminal: Terminal<TermionBackend<RawTerminal<Stdout>>>,
    pub selection_index:Option<usize>,
    pub max_file_selection :usize, //Not negative integer type 
    pub directory_content: Vec<DirectoryItem>,
    pub window_height:u16,
    pub command_buffer:Vec<char>,
    pub preview_content:Option<String>
    // pub home_directory:PathBuf

    
 }

impl App{
    pub fn new(terminal: Terminal<TermionBackend<RawTerminal<Stdout>>>) -> App{

        // let current_directory=path::PathBuf::from("./");
        
        let window_height=terminal.size().unwrap().height -5;
        // println!("actual path : {}\n -------",current_directory.display());
        let mut current_directory=env::current_dir();
        let current_directory=match current_directory{
            Ok(val)=> val,
            _=>path::PathBuf::new()
        };

        let mut app=App{
            current_directory:current_directory,
            terminal:terminal,
            selection_index:Some(0),
            max_file_selection:0,
            directory_content:Vec::new(),
            window_height:window_height,
            command_buffer:Vec::new(),
            preview_content:Some(String::from("")),
        };
        app.populate_files();
        app

    }

    pub fn update_window_height(&mut self){
        self.window_height=self.terminal.size().unwrap().height -5
    }


    pub fn move_selection_down( &mut self){

        if let Some(selection_index)= self.selection_index{
            if  selection_index < self.max_file_selection -1 {
               self.selection_index=Some(selection_index +1) 
            }
        }

    }

    pub fn move_selection_up( &mut self){

        if let Some(selection_index)= self.selection_index{
            if  selection_index > 0 {
               self.selection_index=Some(selection_index -1) 
            }
        }

    }

    pub fn open_folder(&mut self){
        if let Some(selection_index)=self.selection_index{
            if let DirectoryItem::Directory(path)= &self.directory_content[selection_index]{
                let previous_directory=self.current_directory.clone();
                self.current_directory=path.into();


                if let Err(err)=self.populate_files(){
                    self.current_directory=previous_directory;
                }
                else{
                    self.selection_index=Some(0)
                }
            }
        }

    }


    pub fn move_back_directory(&mut self){
        let current_directory: &str= self.current_directory.to_str().unwrap();

        


    }





    pub fn populate_files(&mut self) -> Result<(),std::io::Error>{
        let files: Vec<DirectoryItem>= crate::files::get_files_for_current_directory(self)?;
        
        // for path in files {
        //     println!("Name: {:?}",path)
        // }

        self.directory_content=files;
        self.max_file_selection=self.directory_content.len();
        if self.max_file_selection==0{
            self.selection_index=None;
        };
        Ok(())

    }


    pub fn add_to_command_buffer(mut self,command_char: char){
        self.command_buffer.push(command_char);

    }
    pub fn get_command_buffer_as_string(&self) -> String{
        let mut concatenated_scommand_buffer=String::new();
        for c in &self.command_buffer{concatenated_scommand_buffer.push(*c)};
        concatenated_scommand_buffer
        


    }

}
