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

use crate::Buffer;
use std::rc::Rc;

pub struct Tab {
	buffer: Rc<Buffer>,
	cursor_column: usize, // 0-indexed
	cursor_line: usize,   // 0-indexed
	title: String,        // Title displayed at the top of the tab in the UI.
	window_column: usize, // Top column visible in the window, 0-indexed.
	window_line: usize,   // Top line visible in the window, 0-indexed.
}

impl Tab {
	/// Adjusts the current window position to keep the cursor within it, given
	/// the current window size.
	pub fn adjust_window(&mut self, width: usize, height: usize) {
		self.window_line = adjust_range(self.window_line, height, self.cursor_line);
		self.window_column = adjust_range(
			self.window_column,
			width - self.buffer.line_number_column_width(),
			self.cursor_column,
		);
	}

	pub fn new(name: Option<String>) -> Tab {
		Tab {
			buffer: Buffer::new().into(),
			cursor_column: 0,
			cursor_line: 0,
			title: name.unwrap_or_else(|| "[No Name]".into()),
			window_column: 0,
			window_line: 0,
		}
	}

	pub fn title(&self) -> &str {
		&self.title
	}
}

// -----------------------------------------------------------------------------
// Implementation details below.
// -----------------------------------------------------------------------------

// Adjusts a range with the given size to contain the given position. The range
// is specified and returned by returning the start of the range. The range and
// position are expected to be 0-indexed.
fn adjust_range(range_start: usize, length: usize, must_contain: usize) -> usize {
	if range_start > must_contain {
		return must_contain;
	}
	if range_start + length <= must_contain {
		return must_contain + 1 - length;
	}
	return range_start;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_adjust_range() {
		assert_eq!(adjust_range(0, 1, 1), 1);
		assert_eq!(adjust_range(1, 1, 0), 0);
		assert_eq!(adjust_range(0, 1, 0), 0);
		assert_eq!(adjust_range(0, 2, 2), 1);
		assert_eq!(adjust_range(0, 2, 3), 2);
		assert_eq!(adjust_range(1, 2, 0), 0);
		assert_eq!(adjust_range(1, 2, 1), 1);
	}
}
