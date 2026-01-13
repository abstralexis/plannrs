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

//! Module for the task struct and various operations upon a task.

use serde::{Deserialize, Serialize};
use toml;

/// The tasks that may be registered in lists within the application. All fields are public,
/// but this is because each `Task` is designed to be largely ephemeral in nature - operated
/// on briefly then saved back into the tasks file.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub is_complete: bool,
    /// The due date is stored as `toml::value::Datetime`, because `chrono::Datetime<chrono::Utc>`
    /// does not `impl Serialize + Deserialize`.
    ///
    /// **See:** https://docs.rs/toml/latest/toml/value/struct.Datetime.html
    pub due_date: Option<toml::value::Datetime>,
}
impl Task {
    /// Creates a task object. This is **not** registration of the task to any list,
    /// this must be done at the calling location if needed.
    pub fn new(
        title: String,
        description: String,
        due_date: Option<toml::value::Datetime>,
    ) -> Self {
        Self {
            title,
            description,
            is_complete: true,
            due_date,
        }
    }
}
