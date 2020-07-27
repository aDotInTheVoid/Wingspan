// All of this is so naive.
// https://github.com/linebender/druid/blob/v0.6.0/druid/src/widget/textbox.rs
// https://github.com/linebender/druid/blob/v0.6.0/druid/src/text/text_input.rs

use druid::{
    kurbo::Line,
    piet::{FontBuilder, PietText, PietTextLayout, Text, TextLayout, TextLayoutBuilder},
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
        let font_size = env.get(theme::TEXT_SIZE_NORMAL);
        // TODO: caching of both the format and the layout
        let font = piet_text
            // While druid uses the "toy" api, this is fine
            .new_font_by_name("JetBrains Mono", font_size)
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
        let cursor_color = env.get(theme::CURSOR_COLOR);

        let is_focused = ctx.is_focused();

        let _border_color = if is_focused {
            env.get(theme::PRIMARY_LIGHT)
        } else {
            env.get(theme::BORDER_DARK)
        };

        // Paint the background
        let clip_rect = ctx.size().to_rect();
        ctx.fill(clip_rect, &background_color);

        // Core text happens in the lambda, outside is for aux.
        ctx.with_save(|rc| {
            // For padding stuff, I think
            rc.clip(clip_rect);

            // TODO: Figure out where this 0.8 comes from
            // font_size is the number of "pixels" from top to bottom,
            // so that makes sense.
            let text_height = font_size * 0.8;

            // TODO: scrolling
            // TODO: what is this
            let text_pos = Point::new(0.0, text_height);

            // TODO: Only pull the bit of the rope that's on screen.
            // This is super wastefull, as we do a full copy out of the rope to render
            // but we need to convert to string before we sent it into druid
            // so what we should do it see what needs to be painted (ie is onscrean)
            // and only copy that.
            let text_layout = self.get_layout(&mut rc.text(), &data.to_string(), env);

            // This offset calc could be nicer if we assume a monospace font.
            // also the position is relative the the text in text_layout, not the global rope,
            // so when we implent partial views, this will need to change
            let text_byte_idx = data.rope().char_to_byte(data.curser());

            // 0 indexed from top line number. We can probably cache this, but this should
            // be quite fast (O(log n)).
            let lineno: f64 = data.rope().char_to_line(data.curser()) as f64;

            // This gives us the x, but for y, it's not platform independent,
            // linux gives from the top, but macOS is on the bottom.
            // Also we need to do edge cases around the \n that this get's wrong
            // so we only can trust pos.x
            let pos = text_layout.hit_test_text_position(text_byte_idx);

            // Curser rendering
            // TODO: hot to fall back if pos==None
            // that'll happen if the rope is empty, and maybe other reasons
            if let Some(pos) = pos {
                // pos is either top left, or bottom left, platfrom dependent
                // Therefor we only use pos for x
                let mut x = pos.point.x;

                // Font size: 15
                // Text height: 12
                // Line | Bottom y | Top y
                // -----|----------|------
                // 0    | 0        | 15
                // 1    | 19       | 34
                // 2    | 38       | 53
                // on macOS with JetBrains mono

                // Calculate the top and bottom y coords
                // TODO: figure out where this number is from
                // I got it by debuging  pos.point.y (see table above)
                // This doesn't work if you cange the font to arial
                let magic_number = 4.0;
                let line_spacing = font_size + magic_number;
                let topy = lineno * line_spacing;
                let bottomy = ((lineno + 1.0) * line_spacing) - magic_number;

                // If we are just past a newline, the position thinks we're on the line
                // above, so misreports. Here we abjust it. y is fine as it is calculated
                // seperayly from rope data, which gets this right.
                if data.rope().chars_at(data.curser().saturating_sub(1)).next() == Some('\n') {
                    x = 1.0;
                }
                let top = Point::new(x, topy);
                let bottom = Point::new(x, bottomy);
                let line = Line::new(top, bottom);

                rc.stroke(line, &cursor_color, 1.0)
            }

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
