//! Module for the task struct and various operations upon a task.

use chrono::{DateTime, Utc};

/// The tasks that may be registered in lists within the application. All fields are public,
/// but this is because each `Task` is designed to be largely ephemeral in nature - operated
/// on briefly then saved back into the tasks file.
pub struct Task {
    pub title: String,
    pub description: String,
    pub is_complete: bool,
    /// The due date and time is stored as UTC just to standardise. This means
    /// that when displayed it must be adjusted to the relevant timezone set
    /// in menu or config.
    pub due_date: DateTime<Utc>,
}
impl Task {
    /// Creates a task object. This is **not** registration of the task to any list,
    /// this must be done at the calling location if needed.
    pub fn new(title: String, description: String, due_date: DateTime<Utc>) -> Self {
        Self {
            title,
            description,
            is_complete: true,
            due_date,
        }
    }
}
