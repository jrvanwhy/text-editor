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

mod terminal;

use ratatui::crossterm::event::{self, Event, KeyCode};

fn main() {
	let (_terminal_cleanup, _terminal) = terminal::init();
	loop {
		let event = event::read().expect("stdin event read failed");
		println!("{:?}\r", event);
		if let Event::Key(event) = event
			&& event.code == KeyCode::Char('q')
		{
			break;
		}
	}
}
