//! Module for the task struct and various operations upon a task.

use serde::{Deserialize, Serialize};
use toml;

/// The tasks that may be registered in lists within the application. All fields are public,
/// but this is because each `Task` is designed to be largely ephemeral in nature - operated
/// on briefly then saved back into the tasks file.
#[derive(Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub is_complete: bool,
    /// The due date is stored as `toml::value::Datetime`, because `chrono::Datetime<chrono::Utc>`
    /// does not `impl Serialize + Deserialize`.
    ///
    /// **See:** https://docs.rs/toml/latest/toml/value/struct.Datetime.html
    pub due_date: toml::value::Datetime,
}
impl Task {
    /// Creates a task object. This is **not** registration of the task to any list,
    /// this must be done at the calling location if needed.
    pub fn new(title: String, description: String, due_date: toml::value::Datetime) -> Self {
        Self {
            title,
            description,
            is_complete: true,
            due_date,
        }
    }
}
