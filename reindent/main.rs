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

//! A program for converting files between my personal preferred indentation
//! format (tabs) and the project's indentation format (read from editorconfig
//! with a default fallbacks per-language).
//!
//! This was written for my own personal use -- I don't expect anyone else to
//! have a use for this. If you are interested, please reach out! I could give
//! it a better interface and publish it to crates.io if it's useful. Lacking
//! another user, I plan to retire this once my text editor is working.

use std::io::{Read as _, stdin};

// Notes:
// Need to fsync temporary file before rename.

fn main() {
	let mut input = vec![];
	if let Err(error) = stdin().lock().read_to_end(&mut input) {
		eprintln!("Failed to read stdin: {error}");
		return;
	}
}
