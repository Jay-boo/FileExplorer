
mod files;

mod app;
mod ui;
use app::App;

use std::{io, thread, time};

use termion::input::TermRead;
use termion::raw::IntoRawMode;

use tui::backend::TermionBackend;
use tui::Terminal;

fn main() -> Result<(),io::Error>{
    let stdout=io::stdout().into_raw_mode()?;
    let backend= TermionBackend::new(stdout);
    

    let mut terminal =Terminal::new(backend)?;
    terminal.clear();


    let mut stdin =termion::async_stdin().keys();
    let mut app=App::new(terminal);



    loop {
        
        // app.update_window_height();
        let input = stdin.next();
        if let  Some(Ok(key))=input{

            if key==termion::event::Key::Esc{
                break;
            }
            
            match key{
                termion::event::Key::Down=> app.move_selection_down(),
                termion::event::Key::Up=> app.move_selection_up(),
                termion::event::Key::Char('\n')=> app.open_folder(),
                _=>{} 
            }


        }
        app.populate_files();
        ui::draw(&mut app)?;

        
        
    }
    Ok(())
}
