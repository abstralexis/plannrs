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

use color_eyre::Result;

mod app;
mod task;

fn main() -> Result<()> {
    color_eyre::install()?;

    ratatui::run(|terminal| app::App::default().run(terminal))?;

    Ok(())
}
