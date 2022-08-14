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

use async_trait::async_trait;
use crossterm::event::*;

use crate::*;

/// Async trait object that implements the [Render] trait.
#[derive(Default, Debug, Clone, Copy)]
pub struct AppNoLayout {
  pub lolcat: Lolcat,
}

#[async_trait]
impl TWApp<AppNoLayoutState, AppNoLayoutAction> for AppNoLayout {
  async fn app_render(
    &mut self, state: &AppNoLayoutState,
    _shared_store: &SharedStore<AppNoLayoutState, AppNoLayoutAction>, window_size: Size,
  ) -> CommonResult<TWCommandQueue> {
    throws_with_return!({
      let content = format!("{}", state);

      let content_size = content.len() as UnitType;
      let col: UnitType = window_size.cols / 2 - content_size / 2;
      let row: UnitType = window_size.rows / 2;

      let colored_content = colorize_using_lolcat!(&mut self.lolcat, "{}", state);

      let mut queue = tw_command_queue!(
        TWCommand::ClearScreen,
        TWCommand::ResetColor,
        TWCommand::MoveCursorPositionAbs((col, row).into()),
        TWCommand::PrintWithAttributes(colored_content, None),
        TWCommand::ResetColor
      );

      status_bar_helpers::create_status_bar_message(&mut queue, window_size);

      call_if_true!(DEBUG, {
        log_no_err!(
          INFO,
          "⛵ AppNoLayout::render -> size, state: {} {}",
          window_size,
          state
        );
        log_no_err!(INFO, "⛵ AppNoLayout::render -> queue: {:?}", queue);
      });
      queue
    });
  }

  async fn app_handle_event(
    &mut self, input_event: &TWInputEvent, _state: &AppNoLayoutState,
    shared_store: &SharedStore<AppNoLayoutState, AppNoLayoutAction>, _terminal_size: Size,
  ) -> CommonResult<EventPropagation> {
    throws_with_return!({
      call_if_true!(
        DEBUG,
        log_no_err!(
          INFO,
          "⛵ AppNoLayout::handle_event -> input_event: {}",
          input_event
        )
      );

      let mut event_consumed = false;

      if let TWInputEvent::DisplayableKeypress(typed_char) = input_event {
        match typed_char {
          '+' => {
            spawn_and_consume_event!(event_consumed, shared_store, AppNoLayoutAction::AddPop(1));
            call_if_true!(
              DEBUG,
              log_no_err!(
                INFO,
                "⛵ AppNoLayout::handle_event -> + -> dispatch_spawn: {}",
                AppNoLayoutAction::AddPop(1)
              )
            );
          }
          '-' => {
            spawn_and_consume_event!(event_consumed, shared_store, AppNoLayoutAction::SubPop(1));
            call_if_true!(
              DEBUG,
              log_no_err!(
                INFO,
                "⛵ AppNoLayout::handle_event -> - -> dispatch_spawn: {}",
                AppNoLayoutAction::SubPop(1)
              )
            );
          }
          _ => {}
        }
      }

      if let TWInputEvent::NonDisplayableKeypress(key_event) = input_event {
        match key_event {
          KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
          } => {
            spawn_and_consume_event!(event_consumed, shared_store, AppNoLayoutAction::AddPop(1));
            call_if_true!(
              DEBUG,
              log_no_err!(
                INFO,
                "⛵ AppNoLayout::handle_event -> Up -> dispatch_spawn: {}",
                AppNoLayoutAction::AddPop(1)
              )
            );
          }
          KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
          } => {
            spawn_and_consume_event!(event_consumed, shared_store, AppNoLayoutAction::SubPop(1));
            call_if_true!(
              DEBUG,
              log_no_err!(
                INFO,
                "⛵ AppNoLayout::handle_event -> Down -> dispatch_spawn: {}",
                AppNoLayoutAction::SubPop(1)
              )
            );
          }
          _ => {}
        }
      }

      if event_consumed {
        EventPropagation::Consumed
      } else {
        EventPropagation::Propagate
      }
    });
  }
}

mod status_bar_helpers {
  use r3bl_rs_utils::*;

  /// Shows helpful messages at the bottom row of the screen.
  pub fn create_status_bar_message(queue: &mut TWCommandQueue, size: Size) {
    let st_vec = styled_texts! {
      styled_text! { "Hints:",            style!(attrib: [dim])       },
      styled_text! { " Ctrl+q: Exit ⛔ ", style!(attrib: [bold])      },
      styled_text! { " … ",               style!(attrib: [dim])       },
      styled_text! { " ↑ / + : inc ",     style!(attrib: [underline]) },
      styled_text! { " … ",               style!(attrib: [dim])       },
      styled_text! { " ↓ / - : dec ",     style!(attrib: [underline]) }
    };

    let display_width = st_vec.unicode_string().display_width;
    let col_center: UnitType = (size.cols / 2) - (display_width / 2);
    let row_bottom: UnitType = size.rows - 1;
    let center: Position = (col_center, row_bottom).into();

    *queue += TWCommand::MoveCursorPositionAbs(center);
    *queue += st_vec.render();
  }
}
