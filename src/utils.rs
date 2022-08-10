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

use r3bl_rs_utils::*;

/// Appends commands to the queue that display a 'quit' hint at the center, bottom.
pub fn append_quit_msg_center_bottom(queue: &mut TWCommandQueue, size: Size) {
  let message: String = "Press Ctrl + q to exit!".into();
  let col_center: UnitType = size.cols / 2 - convert_to_base_unit!(message.len()) / 2;
  let row_bottom: UnitType = size.rows - 1;
  tw_command_queue!(
   queue push
    TWCommand::MoveCursorPositionAbs((col_center,row_bottom).into()),
    TWCommand::PrintWithAttributes(message, None)
  );
}
