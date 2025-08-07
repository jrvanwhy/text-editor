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

use std::rc::Rc;

#[derive(Clone)]
pub struct Line {
	contents: Rc<str>,
	indent_level: usize,
}

impl Line {
	pub fn contents(&self) -> &str {
		&self.contents
	}

	// TODO: Re-do line loading with one that understands syntax and can get
	// indent_level correct.
	pub fn new(contents: String) -> Line {
		Line {
			indent_level: contents.chars().take_while(|&c| c == '\t').count(),
			contents: contents.into(),
		}
	}

	pub fn indentation_bytes(&self) -> usize {
		let mut bytes = 0;
		let mut col = 0;
		for c in self.contents.chars() {
			col += match c {
				' ' => 1,
				'\t' => 4,
				_ => break,
			};
			bytes += c.len_utf8();
			if col >= self.indent_level * 4 {
				break;
			}
		}
		bytes
	}
}
