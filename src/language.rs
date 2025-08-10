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

use std::path::Path;
use std::sync::LazyLock;
use tree_sitter::Parser;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Language {
	Rust,
}

impl Language {
	pub fn for_path(path: &Path) -> Option<Language> {
		let extension = path.extension()?;
		if extension == "rs" { return Some(Language::Rust); }
		None
	}

	pub fn parser(self) -> &'static Parser {
		macro_rules! static_parser {
			($name:ident, $c:ident) => {
				static $name: LazyLock<Parser> = LazyLock::new(|| {
					let mut parser = Parser::new();
					parser.set_language(&$c::LANGUAGE.into()).unwrap();
					parser
				});
			}
		}
		static_parser!(RUST, tree_sitter_rust);
		match self {
			Language::Rust => &*RUST,
		}
	}
}
