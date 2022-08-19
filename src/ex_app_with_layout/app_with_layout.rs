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
 *   Unless required by applicable law or agreed &to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
 */

use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;
use crossterm::event::*;
use r3bl_rs_utils::*;
use tokio::sync::RwLock;

use crate::*;

// Constants for the ids.
const CONTAINER_ID: &str = "container";
const COL_1_ID: &str = "col_1";
const COL_2_ID: &str = "col_2";

/// Async trait object that implements the [TWApp] trait.
#[derive(Default)]
pub struct AppWithLayout {
  pub component_registry: ComponentRegistry<AppWithLayoutState, AppWithLayoutAction>,
  pub has_focus: HasFocus,
}

#[async_trait]
impl TWApp<AppWithLayoutState, AppWithLayoutAction> for AppWithLayout {
  async fn app_handle_event(
    &mut self, input_event: &TWInputEvent, _state: &AppWithLayoutState,
    _shared_store: &SharedStore<AppWithLayoutState, AppWithLayoutAction>, _terminal_size: Size,
  ) -> CommonResult<EventPropagation> {
    throws_with_return!({
      // Try to handle left and right arrow key input events & return if handled.
      if let Continuation::Return = self.handle_left_right_input_to_switch_focus(input_event) {
        return Ok(EventPropagation::ConsumedRerender);
      }

      // If component has focus, then route input_event to it. Return its propagation enum.
      if let Some(shared_component_has_focus) =
        self.component_registry.get_has_focus(&self.has_focus)
      {
        let result_event_propagation = shared_component_has_focus
          .write()
          .await
          .handle_event(input_event, _state, _shared_store)
          .await?;
        return Ok(result_event_propagation);
      };

      // input_event not handled, propagate it.
      EventPropagation::Propagate
    });
  }

  async fn app_render(
    &mut self, state: &AppWithLayoutState,
    shared_store: &SharedStore<AppWithLayoutState, AppWithLayoutAction>, window_size: Size,
  ) -> CommonResult<TWCommandQueue> {
    throws_with_return!({
      self.create_components_populate_registry_init_focus().await;
      let mut tw_surface = TWSurface {
        stylesheet: style_helpers::create_stylesheet()?,
        ..TWSurface::default()
      };
      tw_surface.surface_start(TWSurfaceProps {
        pos: (0, 0).into(),
        size: (window_size.cols, window_size.rows - 1).into(), // Leave row at bottom for message.
      })?;
      self
        .create_main_container(&mut tw_surface, state, shared_store)
        .await?;
      tw_surface.surface_end()?;

      status_bar_helpers::render(&mut tw_surface.render_buffer, window_size);

      tw_surface.render_buffer
    });
  }
}

impl AppWithLayout {
  fn handle_left_right_input_to_switch_focus(
    &mut self, input_event: &TWInputEvent,
  ) -> Continuation {
    let mut event_consumed = false;

    // Handle Left, Right to switch focus between columns.
    if let TWInputEvent::NonDisplayableKeypress(keypress) = input_event {
      match keypress {
        Keypress {
          maybe_modifier_keys: None,
          non_modifier_key: NonModifierKey::Special(SpecialKey::Left),
        } => {
          event_consumed = true;
          self.switch_focus(KeyCode::Left);
          debug_log_has_focus(
            stringify!(AppWithLayout::app_handle_event).into(),
            &self.has_focus,
          );
        }
        Keypress {
          maybe_modifier_keys: None,
          non_modifier_key: NonModifierKey::Special(SpecialKey::Right),
        } => {
          event_consumed = true;
          self.switch_focus(KeyCode::Right);
          debug_log_has_focus(
            stringify!(AppWithLayout::app_handle_event).into(),
            &self.has_focus,
          );
        }
        _ => {}
      }
    }

    if event_consumed {
      Continuation::Return
    } else {
      Continuation::Continue
    }
  }

  fn switch_focus(&mut self, code: KeyCode) {
    if let Some(_id) = self.has_focus.get_id() {
      if code == KeyCode::Left {
        self.has_focus.set_id(COL_1_ID)
      } else {
        self.has_focus.set_id(COL_2_ID)
      }
    } else {
      log_no_err!(ERROR, "No focus id has been set, and it should be set!");
    }
  }

  async fn create_components_populate_registry_init_focus(&mut self) {
    let _component = ColumnRenderComponent::default();
    let shared_component_r1 = Arc::new(RwLock::new(_component));
    let shared_component_r2 = shared_component_r1.clone();

    // Construct COL_1_ID.
    if self.component_registry.id_does_not_exist(COL_1_ID) {
      self.component_registry.put(COL_1_ID, shared_component_r1);
    }

    // Construct COL_2_ID.
    if self.component_registry.id_does_not_exist(COL_2_ID) {
      self.component_registry.put(COL_2_ID, shared_component_r2);
    }

    // Init has focus.
    if self.has_focus.get_id().is_none() {
      self.has_focus.set_id(COL_1_ID);
    }
  }

  /// Main container CONTAINER_ID.
  async fn create_main_container<'a>(
    &mut self, tw_surface: &mut TWSurface, state: &'a AppWithLayoutState,
    shared_store: &'a SharedStore<AppWithLayoutState, AppWithLayoutAction>,
  ) -> CommonResult<()> {
    throws!({
      tw_surface.box_start(TWBoxProps {
        id: CONTAINER_ID.into(),
        dir: Direction::Horizontal,
        req_size: (100, 100).try_into()?,
        ..Default::default()
      })?;
      self
        .create_left_col(tw_surface, state, shared_store)
        .await?;
      self
        .create_right_col(tw_surface, state, shared_store)
        .await?;
      tw_surface.box_end()?;
    });
  }

  /// Left column COL_1_ID.
  async fn create_left_col<'a>(
    &mut self, tw_surface: &mut TWSurface, state: &'a AppWithLayoutState,
    shared_store: &'a SharedStore<AppWithLayoutState, AppWithLayoutAction>,
  ) -> CommonResult<()> {
    throws!({
      // REFACTOR: use make_box macro here to replace 3 statements below.
      // box_start! {
      //   in: tw_surface,
      //   COL_1_ID,
      //   Direction::Vertical,
      //   (50, 100).try_into()?,
      //   ["style1", "style2"]
      // };
      // render_component! {
      //   in: tw_surface,
      //   from: self.component_registry,
      //   id: COL_1_ID,
      //   has_focus: self.has_focus,
      //   state: state,
      //   shared_store: shared_store
      // };
      // tw_surface.box_end()?;
      make_box! {
        in:    tw_surface,
        id:    COL_1_ID,
        dir:   Direction::Vertical,
        size:  (50, 100).try_into()?,
        style: ["style1", "style2"],
        render: {
          from:         self.component_registry,
          component_id: COL_1_ID,
          has_focus:    self.has_focus,
          state:        state,
          shared_store: shared_store
        }
      }
    });
  }

  // REFACTOR: replace all the code below w/ new macros from create_left_col
  /// Right column COL_2_ID.
  async fn create_right_col(
    &mut self, tw_surface: &mut TWSurface, state: &AppWithLayoutState,
    shared_store: &SharedStore<AppWithLayoutState, AppWithLayoutAction>,
  ) -> CommonResult<()> {
    throws!({
      tw_surface.box_start(TWBoxProps {
        styles: tw_surface.stylesheet.find_styles_by_ids(vec!["style2"]),
        id: COL_2_ID.to_string(),
        dir: Direction::Vertical,
        req_size: (50, 100).try_into()?,
      })?;

      // OPTIMIZE: macro?
      if let Some(shared_component) = self.component_registry.get(COL_2_ID) {
        let current_box = tw_surface.current_box()?;
        let queue = shared_component
          .write()
          .await
          .render(&self.has_focus, current_box, state, shared_store)
          .await?;
        tw_surface.render_buffer += queue;
      }

      tw_surface.box_end()?;
    });
  }
}

mod app_with_layout_helpers {
  use super::*;

  impl Debug for AppWithLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("AppWithLayout")
        .field("component_registry", &self.component_registry)
        .field("state_manage_focus_data", &self.has_focus)
        .finish()
    }
  }
}

mod style_helpers {
  use super::*;

  pub fn create_stylesheet() -> CommonResult<Stylesheet> {
    throws_with_return!({
      stylesheet! {
        style! {
          id: style1
          margin: 1
          color_bg: TWColor::Rgb { r: 55, g: 55, b: 100 }
        },
        style! {
          id: style2
          margin: 1
          color_bg: TWColor::Rgb { r: 55, g: 55, b: 248 }
        }
      }
    })
  }
}

mod status_bar_helpers {
  use super::*;

  /// Shows helpful messages at the bottom row of the screen.
  pub fn render(queue: &mut TWCommandQueue, size: Size) {
    let st_vec = styled_texts! {
      styled_text! { "Hints:",            style!(attrib: [dim])       },
      styled_text! { " x : Exit ⛔ ",     style!(attrib: [bold])      },
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
