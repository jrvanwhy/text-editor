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

use crate::{Mode, Model};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::Tabs;

pub fn render(frame: &mut Frame, model: &mut Model) {
	let [tabs, textarea, statusline] =
		Layout::vertical([Constraint::Length(1), Constraint::Fill(1), Constraint::Length(1)])
			.areas(frame.area());

	frame.render_widget(
		Tabs::new(model.tabs.iter().map(|t| format!(" {} ", t.title())))
			.divider("")
			.highlight_style(
				Style::from((Color::White, Color::Black)).remove_modifier(Modifier::UNDERLINED),
			)
			.padding("", "")
			.select(model.current_tab)
			.style((Color::White, Color::Gray, Modifier::UNDERLINED)),
		tabs,
	);

	model.tabs[model.current_tab].adjust_window(textarea.width.into(), textarea.height.into());

	let message = Line::from(&*model.message);
	let message_width: u16 = message.width().try_into().unwrap();
	frame.render_widget(message, statusline);
	model.render_cursor_position = match model.mode {
		Mode::Command => None,
		Mode::Prompt => {
			let mut pos = statusline.as_position();
			pos.x += message_width;
			Some(pos)
		}
	};
}
