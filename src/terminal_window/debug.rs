/*
 *   Copyright (c) 2022 Nazmul Idris
 *   All rights reserved.

 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at

 *   http://www.apache.org/licenses/LICENSE-2.0

 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
*/

use crate::*;
use std::fmt::{Debug, Display, Formatter};

impl Debug for TerminalWindow {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("TerminalWindow")
      .field("event_stream", &DebugDisplay::Ok)
      .field("terminal_size", &self.terminal_size)
      .finish()
  }
}

/// Pretty print the [DebugDisplay] (its impl of [Debug] trait), for [TerminalWindow]'s `event_stream`.
enum DebugDisplay {
  Ok,
}

impl Debug for DebugDisplay {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Ok => write!(f, "✅"),
    }
  }
}

/// For [ToString].
impl Display for TerminalWindow {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}