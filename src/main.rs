/*
 *   Copyright (c) 2022 R3BL LLC
 *   All rights reserved.
 *
 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
 */

// https://github.com/rust-lang/rust-clippy
// https://rust-lang.github.io/rust-clippy/master/index.html
#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

use r3bl_rs_utils::*;

// Attach sources.
mod ex_app_no_layout;
mod ex_app_with_layout;
mod ex_editor;
mod ex_lolcat;

// Use things from sources.
use ex_app_no_layout::*;
use ex_app_with_layout::*;
use reedline::*;

const HELP_MSG: &str = "\
Type a number to run corresponding example:
  1. App with no layout ❌
  2. App with layout ✅
  3. lolcat 🦜
  4. Text editor 📜
or type Ctrl+C / Ctrl+D / 'x' to exit";

#[tokio::main]
async fn main() -> CommonResult<()> {
  throws!({
    println!("{}", HELP_MSG);
    let maybe_user_selection_string = get_user_selection_from_terminal();
    if let Some(user_selection) = maybe_user_selection_string {
      run_user_selected_example(user_selection).await?;
    }
  })
}

/// This is a single threaded blocking function. The R3BL examples are all async and non-blocking.
fn get_user_selection_from_terminal() -> Option<String> {
  let mut line_editor = Reedline::create();
  let prompt = DefaultPrompt::default();

  loop {
    let maybe_signal = &line_editor.read_line(&prompt);
    if let Ok(Signal::Success(user_input_str)) = maybe_signal {
      match user_input_str.as_str() {
        code @ ("1" | "2" | "3" | "4") => return Some(code.into()),
        "x" => break,
        _ => println!("Unknown command: {}", user_input_str),
      }
    } else if let Ok(Signal::CtrlC) | Ok(Signal::CtrlD) = maybe_signal {
      break;
    }
  }

  None
}

async fn run_user_selected_example(selection: String) -> CommonResult<()> {
  throws!({
    if !selection.is_empty() {
      match selection.as_ref() {
        "1" => throws!(ex_app_no_layout::run_app().await?),
        "2" => throws!(ex_app_with_layout::run_app().await?),
        "3" => throws!(ex_lolcat::run_app().await?),
        "4" => todo!("TODO: implement editor ex!"),
        _ => unimplemented!(),
      }
    }
  })
}
