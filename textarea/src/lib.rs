// All of this is so naive.
// https://github.com/linebender/druid/blob/v0.6.0/druid/src/widget/textbox.rs
// https://github.com/linebender/druid/blob/v0.6.0/druid/src/text/text_input.rs

use druid::{widget::prelude::*, KeyEvent};

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
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut EditableText,
        _env: &Env,
    ) {
        // Fuck it, we'll figure out the focus later
        ctx.request_focus();

        // This can be simplified with #![feature(bindings_after_at)] but this
        // should compile on stable
        if let Event::KeyDown(key_event) = event {
            let KeyEvent {
                key_code,
                /* We can also pull out is repeat here, but the platfoms seem
                 * to do an ok job of handling this */
                ..
            } = key_event;

            use druid::KeyCode::*;

            match key_code {
                ArrowLeft => data.left(),
                ArrowRight => data.right(),

                Delete | Backspace => data.delete(),

                // TODO: Use a more
                PageDown => {
                    self.vscroll += 10.0;
                    ctx.request_paint();
                }
                PageUp => {
                    let down = self.vscroll - 10.0;
                    // f64 isn't Ord
                    self.vscroll = if down < 0. { 0. } else { down };
                    ctx.request_paint();
                }

                // No CRLF, fight me
                Return => data.insert('\n'),

                // https://github.com/linebender/druid/blob/v0.6.0/druid/src/text/text_input.rs
                key_code if key_code.is_printable() => {
                    if let Some(txt) = key_event.text() {
                        //TODO: see if their is a nicer rope way to do this
                        for i in txt.chars() {
                            data.insert(i);
                        }
                    }
                }

                _ => {}
            }
        }
    }

    // TODO: Should we be doing something here?
    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
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
        ctx: &mut UpdateCtx,
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
        _ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &EditableText,
        _env: &Env,
    ) -> Size {
        bc.max()
    }

    // Rendering is in a seperate file for consiseness.
    fn paint(&mut self, ctx: &mut PaintCtx, data: &EditableText, env: &Env) {
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
