use std::slice::Chunks;
use std::{io, fs::DirEntry, path::PathBuf};
use crate::app::App;
use crate::files::DirectoryItem;
use tui::layout::Direction;
use tui::style::{Style, Modifier, Color};
use tui::widgets::{Block,Text, Borders};
use tui::{layout::{Layout, Constraint, Rect}, Frame};
use tui::widgets::Widget;

pub fn draw(app:&mut App ) -> Result<(),io::Error> {
    let command_string=app.get_command_buffer_as_string();
    let App {
        current_directory,
        terminal,
        selection_index,
        max_file_selection,
        directory_content,
        window_height,
        command_buffer,
        preview_content,
    }= app;
    
    terminal.hide_cursor()?;
    terminal.draw( 
        |mut f| {
            let chunks = Layout::default()
                .direction(tui::layout::Direction::Vertical)
                .constraints([Constraint::Min(3),Constraint::Length(3)])
                .split(f.size());

            draw_file_list( &mut f,
                chunks[0],
                directory_content,
                selection_index,
                current_directory);

            // if let Some(err)=error{
            //     
            // }else{
            //
            // }



        } 

    )?;





    Ok(())



}


pub fn draw_file_list<B: tui::backend::Backend>(
    frame : &mut Frame<B>,
    area:Rect,
    files:&Vec<DirectoryItem>,
    selected_file:&Option<usize>,
    current_directory:&PathBuf
    ){

    let mut names : Vec<Text> =Vec::new();
    let mut sizes: Vec<Text> = Vec::new();

    let area_split:Vec<Rect>=vec![
        Rect::new(area.x, area.y, area.width/2, area.height),
        Rect::new(area.x+area.width/2, area.y, area.width/2, area.height)
    ];

    let inner_rects= vec![
        Rect::new(
        area_split[0].x+1,
        area_split[0].y+1,
        area_split[0].width -1,
        area_split[0].height -1
        
    ),
        Rect::new(
        area_split[1].x+1,
        area_split[1].y+1,
        area_split[1].width -1,
        area_split[1].height -1
        
    )];

    Block::default()
        .borders(Borders::ALL)
        .title(format!("🔎 {} ",current_directory.to_string_lossy().replace("/home/jay", "~")).as_ref())
        .render(frame,area_split[0]);

    Block::default()
        .borders(Borders::ALL)
        .title("File preview")
        .render(frame,area_split[1]);
    if files.len() !=0{
         for file in files {
            match file {
                DirectoryItem::File((path,size))=>{
                    let split:Vec<&str>=path.split('/').collect();
                    let string =String::from(format!("📄 {}\n", split[split.len() - 1 as usize]));
                    names.push(Text::raw(string));
                    sizes.push(Text::raw(format!("{}KB\n",size.to_string())));
                }
                DirectoryItem::Directory(path)=>{
                    let split:Vec<&str>=path.split('/').collect();
                    let string =String::from(format!("📁 {}\n", split[split.len() - 1 as usize]));
                    names.push(Text::raw(string));
                    sizes.push(Text::raw("\n"))

                }
            }
        }




        if let Some(selection_index)=selected_file{
            let selected= match  &mut names[*selection_index]{
                Text::Raw(val)=> val,
                _=> ""
            }.to_string();


            names.insert(*selection_index, Text::styled(selected, Style::default()
                .modifier(Modifier::BOLD)
                .fg(Color::Indexed(2))
            ));
            names.remove(selection_index+1);


        };
        let columns=(names.len() as f32/ (area.height -2) as f32).ceil() as u16;
        let column_size =100/columns;
        
        let mut constraints:Vec<Constraint>=Vec::new();


        for _ in 1..=columns as u32 {
                constraints.push(Constraint::Percentage(column_size));
        }


        let chunks=Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(inner_rects[0]);

        for i in 0..=columns -1 {
            let height=(area.height -2) as usize;
            let from : usize=(i as usize *height) as usize;
            let mut to :usize=((i+1) as usize * height);


            if to >=names.len(){
                to=names.len();
            }

            let names_iter=names[from..to].iter();
            let sizes_iter=sizes[from..to].iter();

            tui::widgets::Paragraph::new(names_iter).wrap(false).render(frame, chunks[i as usize]);


            tui::widgets::Paragraph::new(sizes_iter).alignment(tui::layout::Alignment::Right).wrap(false)
                .render(frame, Rect { x: chunks[i as usize].x, y:chunks[i as usize].y , width:chunks[i as usize].width -2 , height: chunks[i as usize].height });


        }
    }

}

pub fn draw_error<B: tui::backend::Backend>(frame: &mut Frame<B>,area:Rect,error:&String){
    let text:Vec<Text>=vec![Text::styled(error, Style::default().fg(Color::Red))];
    tui::widgets::Paragraph::new(text.iter()).block(Block::default().title("Error").borders(Borders::ALL)).render(frame, area);

}
