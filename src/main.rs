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

mod command;
mod input;
mod prompt;
mod tab;
mod terminal;
mod ui;

use input::Event;
use prompt::Prompt;
use ratatui::layout::Position;
use std::env;
use tab::Tab;

struct State {
	cursor_position: Option<Position>,
	message: String,
	mode: Mode,
	prompt: Prompt,
	tabs: Vec<Tab>,
	current_tab: usize,
}

#[derive(Clone, Copy)]
enum Mode {
	Command,
	Prompt,
}

fn main() {
	let (_terminal_cleanup, mut terminal) = terminal::init();
	let cmdline_tabs: Vec<_> = env::args().skip(1).map(Tab::new_with_filename).collect();
	let mut state = State {
		cursor_position: None,
		message: String::new(),
		mode: Mode::Command,
		prompt: Prompt::default(),
		tabs: match cmdline_tabs.is_empty() {
			false => cmdline_tabs,
			true => vec![Tab::new_unnamed()],
		},
		current_tab: 0,
	};
	while !state.tabs.is_empty() {
		let _ = terminal.draw(|frame| ui::render(frame, &mut state)).expect("terminal draw failed");
		match state.cursor_position {
			None => terminal.hide_cursor().unwrap(),
			Some(position) => {
				terminal.set_cursor_position(position).unwrap();
				terminal.show_cursor().unwrap();
			}
		};
		let event = input::next();
		match (event, state.mode) {
			(Event::Key(key_event), Mode::Command) => command::on_key(&mut state, key_event),
			(Event::Paste(paste), Mode::Command) => command::on_paste(&mut state, paste),
			(Event::Key(key_event), Mode::Prompt) => prompt::on_key(&mut state, key_event),
			(Event::Paste(paste), Mode::Prompt) => prompt::on_paste(&mut state, paste),
			(Event::Resize, _) => continue,
		}
	}
}
