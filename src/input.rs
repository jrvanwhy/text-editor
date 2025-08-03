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

use ratatui::crossterm;
use ratatui::crossterm::event::{self, KeyEvent};

pub enum Event {
	Key(KeyEvent),
	Paste(String),
	Resize,
}

pub fn next() -> Event {
	loop {
		use crossterm::event::Event as CrosstermEvent;
		return match event::read().expect("stdin event read failed") {
			CrosstermEvent::FocusGained | CrosstermEvent::FocusLost | CrosstermEvent::Mouse(_) => {
				continue;
			}
			CrosstermEvent::Key(key_event) => Event::Key(key_event),
			CrosstermEvent::Paste(string) => Event::Paste(string.replace('\r', "\n")),
			CrosstermEvent::Resize(_, _) => Event::Resize,
		};
	}
}
