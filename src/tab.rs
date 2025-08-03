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

pub struct Tab {
	/// The title displayed at the top of the tab in the UI.
	title: String,
}

impl Tab {
	pub fn new_unnamed() -> Tab {
		Tab { title: "[No Name]".into() }
	}

	pub fn new_with_filename(name: String) -> Tab {
		Tab { title: name }
	}

	pub fn title(&self) -> &str {
		&self.title
	}
}
