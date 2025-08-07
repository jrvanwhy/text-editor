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

use crate::Rope;

pub struct Buffer {
	current_snapshot: usize,
	history: Vec<Rope>,
}

impl Buffer {
	pub fn new(name: Option<&str>) -> Buffer {
		Buffer { current_snapshot: 0, history: vec![Rope::new(name)] }
	}

	pub fn current_snapshot(&self) -> &Rope {
		&self.history[self.current_snapshot]
	}

	pub fn line_number_column_width(&self) -> usize {
		self.history[self.current_snapshot].num_lines().ilog10() as usize + 3
	}
}
