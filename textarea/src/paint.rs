/// Code for painting the [`TextArea`].
///
/// This is the part that is concerned with pushing pixels to the screen.
/// It's called via the `paint` method in `druid::Widget`
use crate::TextArea;

use textedit::EditableText;

use druid::{
    kurbo::Line,
    piet::{
        FontBuilder, PietText, PietTextLayout, Text, TextLayout,
        TextLayoutBuilder,
    },
    theme, Env, PaintCtx, Point, RenderContext,
};

impl TextArea {
    /// Main entry point to the paint system.
    ///
    /// The function signature is that of `druid::Widget::paint`, put it's
    /// placed in a seperate function so the lib.rs can be smaller.
    pub(crate) fn paint_internal(
        &mut self,
        ctx: &mut PaintCtx,
        data: &EditableText,
        env: &Env,
    ) {
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
            // This is super wastefull, as we do a full copy out of the rope to
            // render but we need to convert to string before we
            // sent it into druid so what we should do it see what
            // needs to be painted (ie is onscrean) and only copy
            // that.
            let text_layout =
                self.get_layout(&mut rc.text(), &data.to_string(), env);

            // This offset calc could be nicer if we assume a monospace font.
            // also the position is relative the the text in text_layout, not
            // the global rope, so when we implent partial views,
            // this will need to change
            let text_byte_idx = data.rope().char_to_byte(data.curser());

            // 0 indexed from top line number. We can probably cache this, but
            // this should be quite fast (O(log n)).
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

                // The default works for JetBrains Mono.
                // TODO: sould we log if we have to use the default (ie
                // get_magic has failed)
                let magic_number =
                    self.get_magic(&mut rc.text(), env).unwrap_or(4.0);
                let line_spacing = font_size + magic_number;
                let topy = lineno * line_spacing;
                let bottomy = ((lineno + 1.0) * line_spacing) - magic_number;

                // If we are just past a newline, the position thinks we're on
                // the line above, so misreports. Here we abjust
                // it. y is fine as it is calculated
                // seperayly from rope data, which gets this right.
                if data.rope().chars_at(data.curser().saturating_sub(1)).next()
                    == Some('\n')
                {
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

    fn get_layout(
        &self,
        piet_text: &mut PietText,
        text: &str,
        env: &Env,
    ) -> PietTextLayout {
        //TODO: allow customisable fonts and sizes
        let font_size = env.get(theme::TEXT_SIZE_NORMAL);
        let font = env.get(theme::FONT_NAME);

        // TODO: caching of both the format and the layout
        let font = piet_text
            // While druid uses the "toy" api, this is fine
            .new_font_by_name(font, font_size)
            .build()
            .unwrap();

        piet_text
            .new_text_layout(&font, &text.to_string(), std::f64::INFINITY)
            .build()
            .unwrap()
    }

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
    fn get_magic(&self, piet_text: &mut PietText, env: &Env) -> Option<f64> {
        let layout = self.get_layout(piet_text, "12\n45", env);
        let top = layout.hit_test_text_position(1)?.point.y;
        let bottom = layout.hit_test_text_position(4)?.point.y;
        let font_size = env.get(theme::TEXT_SIZE_NORMAL);
        Some(bottom - top - font_size)
    }
}
