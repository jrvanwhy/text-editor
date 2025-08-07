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

use crate::Line;
use std::fs::File;
use std::io::{BufRead as _, BufReader};

// TODO: This is not a rope yet.
#[derive(Clone)]
pub struct Rope {
	lines: Vec<Line>,
}

impl Rope {
	pub fn new(filename: Option<&str>) -> Rope {
		Rope {
			lines: (|| {
				Some(
					BufReader::new(File::open(filename?).ok()?)
						.lines()
						.map(|l| Line::new(l.unwrap()))
						.collect::<Vec<_>>(),
				)
			})()
			.unwrap_or_else(|| vec![Line::new("".into())]),
		}
	}

	pub fn line(&self, idx: usize) -> &Line {
		&self.lines[idx]
	}

	pub fn num_lines(&self) -> usize {
		self.lines.len()
	}
}
