use anyhow::Result;
use ratatui::{DefaultTerminal, Frame, crossterm};

mod task;

fn main() -> Result<()> {
    ratatui::run(app)?;

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
