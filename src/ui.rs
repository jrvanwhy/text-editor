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
use ratatui::text::{Line, Span, Text};
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

	let tab = &mut model.tabs[model.current_tab];
	tab.adjust_window(textarea.width.into(), textarea.height.into());
	let (start_line, start_column) = tab.window_location();
	let number_col_width = tab.buffer().line_number_column_width();
	let current_snapshot = tab.buffer().current_snapshot();
	let mut render_text = Text::default();
	for i in start_line..(start_line + textarea.height as usize) {
		if i >= current_snapshot.num_lines() {
			render_text.push_line(Line::styled("~", Color::Blue));
			continue;
		}
		let mut line = Line::default();
		line.push_span(Span::styled(format!(" {:number_col_width$} ", i + 1), Color::Gray));
		let indentation_bytes = current_snapshot.line(i).indentation_bytes();
		line.push_span(Span::styled(
			current_snapshot.line(i).contents()[..indentation_bytes].replace('\t', "    "),
			(Color::White, Color::DarkGray),
		));
		line.push_span(Span::raw(
			current_snapshot.line(i).contents()[indentation_bytes..].replace('\t', "    "),
		));
		render_text.push_line(line);
	}
	frame.render_widget(render_text, textarea);

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
