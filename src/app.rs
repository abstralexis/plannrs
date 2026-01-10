//! The app module for the program. A common initialisation could look like:
//! ```rs
//! ratatui::run(|terminal| app::App::default().run(terminal))?;
//! ```

use std::{io, str::FromStr};

use crate::task;

use ratatui::{DefaultTerminal, Frame};
use toml::value::Datetime;
use tui_textarea::{Input, Key, TextArea};

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
        todo!();
    }

    /// Checks for inputs to text areas, pressing of UI buttons, and keystrokes that
    /// would control app state. Should only be used inside of `App::run()`.
    fn handle_events(&mut self) -> io::Result<()> {
        todo!();
    }
}
