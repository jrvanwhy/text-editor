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

use crate::{Mode, Model, command};
use ratatui::crossterm::event::{KeyCode, KeyEvent};

pub type Prompt = String;

pub fn on_key(model: &mut Model, key_event: KeyEvent) {
	match key_event.code {
		KeyCode::Backspace => {
			if model.prompt.pop().is_none() {
				model.message = String::new();
				command::start(model);
			} else {
				update_message(model);
			}
		}
		KeyCode::Enter => execute_cmd(model),
		KeyCode::Esc => {
			model.message = String::new();
			command::start(model);
		}
		KeyCode::Char(c) => {
			model.prompt.push(c);
			update_message(model);
		}
		_ => {}
	}
}

pub fn on_paste(_model: &mut Model, _paste: String) {}

pub fn start(model: &mut Model) {
	model.mode = Mode::Prompt;
	model.prompt = String::new();
	update_message(model);
}

// -----------------------------------------------------------------------------
// Implementation details below.
// -----------------------------------------------------------------------------

fn execute_cmd(model: &mut Model) {
	match &*model.prompt {
		"q" | "q!" => {
			model.tabs.remove(model.current_tab);
			if model.current_tab >= model.tabs.len() && model.current_tab > 0 {
				model.current_tab = model.tabs.len() - 1;
			}
		}
		"qall" | "qall!" => model.tabs = vec![],
		"tabn" => model.current_tab = (model.current_tab + 1) % model.tabs.len(),
		"tabp" => {
			model.current_tab = match model.current_tab.checked_sub(1) {
				None => model.tabs.len() - 1,
				Some(n) => n,
			}
		}
		_ => {}
	}
	command::start(model);
}

fn update_message(model: &mut Model) {
	model.message = format!(":{}", model.prompt);
}
