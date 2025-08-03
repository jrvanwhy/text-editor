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

use crate::{Mode, State, command};
use ratatui::crossterm::event::{KeyCode, KeyEvent};

pub type Prompt = String;

pub fn on_key(state: &mut State, key_event: KeyEvent) {
	match key_event.code {
		KeyCode::Backspace => {
			if state.prompt.pop().is_none() {
				state.message = String::new();
				command::start(state);
			} else {
				update_message(state);
			}
		}
		KeyCode::Enter => execute_cmd(state),
		KeyCode::Esc => {
			state.message = String::new();
			command::start(state);
		}
		KeyCode::Char(c) => {
			state.prompt.push(c);
			update_message(state);
		}
		_ => {}
	}
}

pub fn on_paste(_state: &mut State, _paste: String) {}

pub fn start(state: &mut State) {
	state.mode = Mode::Prompt;
	state.prompt = String::new();
	update_message(state);
}

// -----------------------------------------------------------------------------
// Implementation details below.
// -----------------------------------------------------------------------------

fn execute_cmd(state: &mut State) {
	match &*state.prompt {
		"q" | "q!" => {
			state.tabs.remove(state.current_tab);
			if state.current_tab >= state.tabs.len() && state.current_tab > 0 {
				state.current_tab = state.tabs.len() - 1;
			}
		}
		"tabn" => state.current_tab = (state.current_tab + 1) % state.tabs.len(),
		"tabp" => {
			state.current_tab = match state.current_tab.checked_sub(1) {
				None => state.tabs.len() - 1,
				Some(n) => n,
			}
		}
		_ => {}
	}
	command::start(state);
}

fn update_message(state: &mut State) {
	state.message = format!(":{}", state.prompt);
}
