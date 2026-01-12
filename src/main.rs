use color_eyre::Result;
use ratatui::{DefaultTerminal, Frame, crossterm};

mod app;
mod task;

fn main() -> Result<()> {
    color_eyre::install()?;

    ratatui::run(|terminal| app::App::default().run(terminal))?;

    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("Hello Ratatui!", frame.area());
}
