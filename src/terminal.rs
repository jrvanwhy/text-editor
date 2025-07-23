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

//! Terminal-configuration logic (e.g. raw mode setup).

use ratatui::crossterm::cursor::Show;
use ratatui::crossterm::event::{DisableBracketedPaste, EnableBracketedPaste};
use ratatui::crossterm::execute;
use std::{io::stdout, panic, process::abort};

pub type Terminal = ratatui::DefaultTerminal;

/// Initializes the terminal. Returns the ratatui terminal and a cleanup handle
/// that performs other miscellaneous cleanups. Note that this should be called
/// early (and exactly once), as it installs a panic handler.
pub fn init() -> (CleanupHandle, Terminal) {
	let old_hook = panic::take_hook();
	panic::set_hook(Box::new(move |panic_hook_info| {
		// Normally, ratatui's Terminal cleans up the cursor display state on
		// drop. However, we can't drop the Terminal on panic (as other code
		// owns it), so instead manually show the cursor.
		if let Err(error) = execute!(stdout(), Show) {
			eprintln!("Failed to show cursor: {}", error);
		}
		common_cleanup();
		old_hook(panic_hook_info);
		println!("Editor cannot continue with terminal in normal mode; aborting.");
		abort();
	}));
	execute!(stdout(), EnableBracketedPaste).expect("failed to enable bracketed paste");
	(CleanupHandle { _private: () }, ratatui::init())
}

/// Dropping this resets the terminal to normal mode (i.e. drop before
/// exiting).
pub struct CleanupHandle {
	_private: (),
}

impl Drop for CleanupHandle {
	fn drop(&mut self) {
		ratatui::restore();
		common_cleanup();
	}
}

// -----------------------------------------------------------------------------
// Implementation details below.
// -----------------------------------------------------------------------------

/// Performs cleanup common to the "CleanupHandle dropped" and panic cases.
fn common_cleanup() {
	execute!(stdout(), DisableBracketedPaste).expect("failed to disable bracketed paste");
}
