// Copyright 2020 The Wingspan Authors.
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
#![warn(clippy::all, rust_2018_idioms)]

// All of this is so naive.
// https://github.com/linebender/druid/blob/v0.6.0/druid/src/widget/textbox.rs
// https://github.com/linebender/druid/blob/v0.6.0/druid/src/text/text_input.rs

use druid::{widget::prelude::*, MouseEvent};

use textedit::EditableText;

mod paint;

/// Hidden state goes here, application state goes on the widget
///
/// I'm not quite sure what goes where
#[derive(Default, Clone, Copy)]
pub struct TextArea {
    vscroll: f64,
}

impl TextArea {
    pub fn new() -> Self {
        Self { vscroll: 0. }
    }
}

impl Widget<EditableText> for TextArea {
    /// Recieve an event from the OS
    /// Most likely a keypress
    /// Here is where we can edit the rope
    fn event(
        &mut self,
        ctx: &mut EventCtx<'_, '_>,
        event: &Event,
        data: &mut EditableText,
        _env: &Env,
    ) {
        // Fuck it, we'll figure out the focus later
        ctx.request_focus();

        match event {
            Event::KeyDown(key_event) => {
                use druid::keyboard_types::Key::*;
                match &key_event.key {
                    ArrowLeft => data.left(),
                    ArrowRight => data.right(),

                    Delete | Backspace => data.delete(),

                    // TODO: Use a better number than 10.0
                    PageDown => {
                        self.vscroll += 10.0;
                    }
                    PageUp => {
                        let down = self.vscroll - 10.0;
                        // f64 isn't Ord
                        self.vscroll = if down < 0. { 0. } else { down };
                    }

                    // No CRLF, fight me
                    Enter => data.insert('\n'),
                    Character(chars) => {
                        //TODO: Surely there is a better way
                        for i in chars.chars() {
                            data.insert(i);
                        }
                    }

                    _ => {}
                }
                ctx.request_paint()
            }
            Event::Wheel(MouseEvent { wheel_delta, .. }) => {
                // TODO: Make this feel native (acceleration, etc)
                let vscroll = self.vscroll + wheel_delta.y;
                self.vscroll = if vscroll < 0. { 0. } else { vscroll };
                ctx.request_paint();
            }
            _ => {}
        }
    }

    // TODO: Should we be doing something here?
    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx<'_, '_>,
        event: &LifeCycle,
        _data: &EditableText,
        _env: &Env,
    ) {
        // LifeCycle isn't Eq
        if let LifeCycle::WidgetAdded = event {
            ctx.register_for_focus();
        }
    }

    // Do internal state stuff.
    fn update(
        &mut self,
        ctx: &mut UpdateCtx<'_, '_>,
        _old_data: &EditableText,
        _data: &EditableText,
        _env: &Env,
    ) {
        //TODO: Is this right?
        ctx.request_paint();
    }

    // Get the size of the widget.
    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx<'_, '_>,
        bc: &BoxConstraints,
        _data: &EditableText,
        _env: &Env,
    ) -> Size {
        bc.max()
    }

    // Rendering is in a seperate file for consiseness.
    fn paint(
        &mut self,
        ctx: &mut PaintCtx<'_, '_, '_>,
        data: &EditableText,
        env: &Env,
    ) {
        self.paint_internal(ctx, data, env);
    }
}

#[cfg(test)]
mod tests {
    //TODO: How to test with druid
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
