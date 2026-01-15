/*
    plannrs, a terminal user interface for creating and managing scheduling and tasks.
    Copyright (C) 2026  Alexis Williams

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

//! The app module for the program. A common initialisation could look like:
//! ```rs
//! ratatui::run(|terminal| app::App::default().run(terminal))?;
//! ```

use std::{io, str::FromStr};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::task;

use toml::value::Datetime;

// TODO: maybe Task Form stuff should be in another file?
/// The location of the cursor within a form for submitting a task. The states represent
/// each text input location and the manual submit button. Use for determining border
/// highlights on selection and functionality of `TaskFormState::enter()`.
#[derive(Debug, Default)]
pub enum TaskFormLocation {
    #[default]
    Title,
    Description,
    Date,
    Submit,
}

/// The state for the task form, if active. Used inside the `App` for both
/// preparing a form ready to submit, as well as submitting.
#[derive(Debug, Default)]
struct TaskFormState {
    pub current_location: TaskFormLocation,

    // Not a fan of the duplication here. Is there a cleaner way to do this?
    pub title: Option<String>,
    pub description: Option<String>,
    pub date: Option<Datetime>,
    pub should_submit: bool,
}
impl TaskFormState {
    /// Enter a value from the input, if it exists, into the current section in the form
    /// or submit if on submit.
    pub fn enter(&mut self, input: &Option<String>) {
        match self.current_location {
            TaskFormLocation::Title => self.title = input.clone(),
            TaskFormLocation::Description => self.title = input.clone(),
            // I really want to use `.map()` here, but it would pose a problem :(
            TaskFormLocation::Date => match input {
                None => self.date = None,
                Some(s) => match Datetime::from_str(&s) {
                    Ok(d) => self.date = Some(d),
                    Err(_) => self.date = None,
                },
            },
            TaskFormLocation::Submit => self.should_submit = true,
        }
    }

    /// Resets the state of the task form to the default. Use after exiting the task form
    /// i.e. after a manual exit or a form submission.
    pub fn clear(&mut self) {
        // is there a way to more idiomatically make this call `.default()` on all items?
        self.current_location = TaskFormLocation::Title;
        self.title = None;
        self.description = None;
        self.date = None;
        self.should_submit = false;
    }
}

/// The App struct for the actual application. This is for shared state, and some
/// required functions for running the program and drawing to the terminal. The only
/// exposed function should be `App::run()`, as the main logic of the program should
/// fit within this struct.
/// # Usage
/// ```rs
/// ratatui::run(|terminal| app::App::default().run(terminal))?;
/// ```
#[derive(Debug, Default)]
pub struct App {
    tasks: Vec<task::Task>,
    task_form_state: TaskFormState,
    should_exit: bool,
}
impl Widget for &App {
    // For now, I am largely following the ratatui tutorial to get some basics up.
    // This will not be permanent! We will need more rendering functions that are internal,
    // so that we can render the different screens when they come around.
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let title = Line::from("plannrs".bold());
        let instructions = Line::from(vec![
            " Switch Input ".into(),
            "<h/k>".blue().bold(),
            " Sumbit ".into(),
            "<Return>".blue().bold(),
            " Quit ".into(),
            "<q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let test_text = Text::from(vec![Line::from(vec![
            "Test Layout: ".into(),
            "Flannrs is blazing fast, fearless pudding with a rusty crust".yellow(),
        ])]);

        Paragraph::new(test_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
impl App {
    /// # Usage
    /// ```rs
    /// ratatui::run(|terminal| app::App::default().run(terminal))?;
    /// ```
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// This is the function for drawing to the terminal. Should only be
    /// used inside of `App::run()`.
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// Checks for inputs to text areas, pressing of UI buttons, and keystrokes that
    /// would control app state. Should only be used inside of `App::run()`.
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.should_exit = true;
    }
}
