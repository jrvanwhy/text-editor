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

// TODO: This is not a rope yet.
#[derive(Clone)]
pub struct Rope {
	lines: Vec<Rc<String>>,
}

impl Rope {
	pub fn new() -> Rope {
		Rope { lines: vec![Rc::new("".into())] }
	}

	pub fn lines(&self) -> usize {
		self.lines.len()
	}
}
