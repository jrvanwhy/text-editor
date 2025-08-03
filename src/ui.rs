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

use crate::{Mode, State};
use ratatui::Frame;
use ratatui::{
	layout::{Constraint, Layout},
	text::Line,
};

pub fn render(frame: &mut Frame, state: &mut State) {
	let [_tabs, _textarea, statusline] =
		Layout::vertical([Constraint::Length(1), Constraint::Fill(1), Constraint::Length(1)])
			.areas(frame.area());
	let message = Line::from(&*state.message);
	let message_width: u16 = message.width().try_into().unwrap();
	frame.render_widget(message, statusline);
	state.cursor_position = match state.mode {
		Mode::Command => None,
		Mode::Prompt => {
			let mut pos = statusline.as_position();
			pos.x += message_width;
			Some(pos)
		}
	};
}
