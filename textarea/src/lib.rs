// All of this is so naive.
// https://github.com/linebender/druid/blob/v0.6.0/druid/src/widget/textbox.rs
// https://github.com/linebender/druid/blob/v0.6.0/druid/src/text/text_input.rs

use druid::{
    piet::{FontBuilder, PietText, PietTextLayout, Text, TextLayoutBuilder},
    theme,
    widget::prelude::*,
    KeyEvent, Point,
};

use textedit::EditableText;

/// Hidden state goes here, application state goes on the widget
///
/// I'm not quite sure what goes where
#[derive(Default, Clone, Copy)]
pub struct TextArea;

impl TextArea {
    pub fn new() -> Self {
        Self
    }

    /// Calculate the PietTextLayout from the given text, font, and font size
    fn get_layout(&self, piet_text: &mut PietText, text: &str, env: &Env) -> PietTextLayout {
        let font_name = env.get(theme::FONT_NAME);
        let font_size = env.get(theme::TEXT_SIZE_NORMAL);
        // TODO: caching of both the format and the layout
        let font = piet_text
            .new_font_by_name(font_name, font_size)
            .build()
            .unwrap();

        piet_text
            .new_text_layout(&font, &text.to_string(), std::f64::INFINITY)
            .build()
            .unwrap()
    }
}

impl Widget<EditableText> for TextArea {
    /// Recieve an event from the OS
    /// Most likely a keypress
    /// Here is where we can edit the rope
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EditableText, _env: &Env) {
        // Fuck it, we'll figure out the focus later
        ctx.request_focus();

        // This can be simplified with #![feature(bindings_after_at)] but this
        // should compile on stable
        if let Event::KeyDown(key_event) = event {
            if let KeyEvent {
                key_code,
                // This causes problems on windows, but that's not my problem
                // This should be fixed in druid, as if this library trys to do
                // compat features, it will die
                is_repeat: false,
                ..
            } = key_event
            {
                use druid::KeyCode::*;

                match key_code {
                    ArrowLeft => data.left(),
                    ArrowRight => data.right(),

                    Delete | Backspace => data.delete(),

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
    }

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

    // Here we need to render the text
    fn paint(&mut self, ctx: &mut PaintCtx, data: &EditableText, env: &Env) {
        // Pull some theme stuff
        let font_size = env.get(theme::TEXT_SIZE_NORMAL);
        let _height = env.get(theme::BORDERED_WIDGET_HEIGHT);
        let background_color = env.get(theme::BACKGROUND_LIGHT);
        let _selection_color = env.get(theme::SELECTION_COLOR);
        let text_color = env.get(theme::LABEL_COLOR);
        let _placeholder_color = env.get(theme::PLACEHOLDER_COLOR);
        let _cursor_color = env.get(theme::CURSOR_COLOR);

        let is_focused = ctx.is_focused();

        let _border_color = if is_focused {
            env.get(theme::PRIMARY_LIGHT)
        } else {
            env.get(theme::BORDER_DARK)
        };

        // Paint the background
        let clip_rect = ctx.size().to_rect();

        ctx.fill(clip_rect, &background_color);

        ctx.with_save(|rc| {
            rc.clip(clip_rect);

            //TODO: Only pull the bit of the rope that's on screen.
            let text_height = font_size * 0.8;
            let text_layout = self.get_layout(&mut rc.text(), &data.to_string(), env);
            let text_pos = Point::new(0.0, text_height);
            rc.draw_text(&text_layout, text_pos, &text_color);
        });
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
