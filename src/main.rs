// Copyright 2025 Ryan Van Why
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod buffer;
mod command;
mod input;
mod line;
mod model;
mod prompt;
mod rope;
mod tab;
mod terminal;
mod ui;

use buffer::Buffer;
use input::Event;
use line::Line;
use model::{Mode, Model};
use prompt::Prompt;
use rope::Rope;
use std::env;
use tab::Tab;

fn main() {
	let (_terminal_cleanup, mut terminal) = terminal::init();
	let mut model = Model::init();
	// TODO: Once multiple tab support is safe (buffers are appropriately
	// deduplicated), change this to initialize one tab for every command line
	// argument.
	model.new_tab(env::args().skip(1).next());
	while !model.tabs.is_empty() {
		let _ = terminal.draw(|frame| ui::render(frame, &mut model)).expect("terminal draw failed");
		match model.render_cursor_position {
			None => terminal.hide_cursor().unwrap(),
			Some(position) => {
				terminal.set_cursor_position(position).unwrap();
				terminal.show_cursor().unwrap();
			}
		};
		let event = input::next();
		match (event, model.mode) {
			(Event::Key(key_event), Mode::Command) => command::on_key(&mut model, key_event),
			(Event::Paste(paste), Mode::Command) => command::on_paste(&mut model, paste),
			(Event::Key(key_event), Mode::Prompt) => prompt::on_key(&mut model, key_event),
			(Event::Paste(paste), Mode::Prompt) => prompt::on_paste(&mut model, paste),
			(Event::Resize, _) => continue,
		}
	}
}
