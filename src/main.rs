use crossterm::{
    event::{self,KeyCode,KeyEventKind}, execute, terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,LeaveAlternateScreen
    }, ExecutableCommand
};

use ratatui::{
    prelude::{CrosstermBackend,Stylize,Terminal},
    widgets::Paragraph,
};

use::std::io::{stdout, Result};

fn main()-> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    term.clear()?;

    //main loop
    loop{
        //draw the ui
        term.draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new("Hello World!").white().on_blue(), area);
        })?;
        //handle events
        if event::poll(std::time::Duration::from_millis(16))?{
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') { break; }
            } 
        }
    }


    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
