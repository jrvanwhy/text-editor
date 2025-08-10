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

use crate::{FileSnapshot, Language};
use std::collections::BTreeMap;
use std::path::PathBuf;
use tree_sitter::Parser;

pub struct Buffer {
	// The version of the file currently being edited (indexes into `history`).
	current_snapshot: usize,
	// This is basically a Vec<FileSnapshot>, but with O(log(n)) insertion time
	// (rather than amortized O(1)). The keys are incrementing number starting
	// at 0 (i.e. they would match the indexes into a Vec).
	history: BTreeMap<usize, FileSnapshot>,
	language: Option<Language>,
	parser: Option<&'static Parser>,
}

impl Buffer {
	pub fn new(path: Option<PathBuf>) -> Buffer {
		let language = path.as_deref().and_then(Language::for_path);
		let parser = language.map(Language::parser);
		Buffer { current_snapshot: 0, history: [(0, FileSnapshot::new(path.as_deref(), parser, language))].into(), language, parser }
	}

	pub fn current_snapshot(&self) -> &FileSnapshot {
		&self.history[&self.current_snapshot]
	}

	pub fn line_number_column_width(&self) -> usize {
		todo!()
		//self.history[self.current_snapshot].lines().ilog10() as usize + 1
	}
}
