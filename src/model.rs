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

//! The model from the traditional model-view-controller design. In short: this
//! is the data structure storing the main state of the editor.

use crate::{Prompt, Tab};
use ratatui::layout::Position;

pub struct Model {
	pub message: String,
	pub mode: Mode,
	pub prompt: Prompt,
	pub render_cursor_position: Option<Position>,
	pub tabs: Vec<Tab>,
	// If tabs is empty (which only happens during editor startup and shutdown),
	// current_tab will be 0.
	pub current_tab: usize,
}

impl Model {
	pub fn init() -> Model {
		Model {
			message: String::new(),
			mode: Mode::Command,
			prompt: Prompt::default(),
			render_cursor_position: None,
			tabs: vec![],
			current_tab: 0,
		}
	}

	pub fn new_tab(&mut self, name: Option<String>) {
		// TODO: Deduplicate buffers if the same file is opened multiple times.
		if !self.tabs.is_empty() {
			self.current_tab += 1;
		}
		self.tabs.insert(self.current_tab, Tab::new(name))
	}
}

#[derive(Clone, Copy)]
pub enum Mode {
	Command,
	Prompt,
}
