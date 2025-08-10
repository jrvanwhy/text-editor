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

use crate::Language;
use ropey::Rope;
use std::fs::read_to_string;
use std::io::ErrorKind;
use std::path::Path;
use tree_sitter::{Parser, Tree};

#[derive(Clone)]
pub struct FileSnapshot {
	/// Each line is stored in the following sequence:
	///     <indent_level>|<line contents>
	/// where <indent_level> is the indentation level encoded as hexadecimal.
	rope: Rope,

	// tree-sitter parse tree and the language it was parsed with.
	tree: Option<Tree>,
	tree_language: Option<Language>,
}

impl FileSnapshot {
	pub fn new(filename: Option<&Path>, parser: Option<&Parser>, language: Option<Language>) -> FileSnapshot {
		let contents = match filename.map(read_to_string) {
			None => None,
			Some(Err(error)) if error.kind() == ErrorKind::NotFound => None,
			Some(Err(error)) => panic!("I/O error: {}", error),
			Some(Ok(contents)) => Some(contents),
		};
		let tree = parser.map(|p| p.parse(contents.as_deref().unwrap_or("\n"), None).unwrap());
	}
}
