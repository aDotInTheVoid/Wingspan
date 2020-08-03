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
        let background_color = env.get(theme::BACKGROUND_LIGHT);
        let text_color = env.get(theme::LABEL_COLOR);
        let cursor_color = env.get(theme::CURSOR_COLOR);

        // First we paint the background
        // This is so we can add padding later, see druid::widget::TextBox for
        // more
        let clip_rect = ctx.size().to_rect();
        ctx.fill(clip_rect, &background_color);

        // Inside the lambda we make thing's relative to where text is, and not
        // padding I think?
        ctx.with_save(|rc| {
            // Here we re-adjust the coord system to ignore padding, see above
            // I think?
            rc.clip(clip_rect);

            // These next 2 lines are a mistery to me.
            // TODO: Figure out where this 0.8 comes from
            // font_size is the number of "pixels" from top to bottom,
            // so that makes sense.
            let text_height = font_size * 0.8;

            // 0 indexed from top line number. We can probably cache this,
            // but this should be quite fast (O(log n)).
            let lineno: f64 = data.rope().char_to_line(data.curser()) as f64;
            // TODO: scrolling
            // TODO: what is this
            let text_pos = Point::new(0.0, text_height - self.vscroll);

            // The default works for JetBrains Mono.
            // TODO: sould we log if we have to use the default (ie
            // get_line_spacing has failed)
            let line_spacing =
                self.get_line_spacing(&mut rc.text(), env).unwrap_or(19.0);
            let topy = lineno * line_spacing;
            let bottomy = topy + font_size;

            // Number of lines to remove from the top.
            // Round down, so lines partialy in display are still rendered
            let lines_to_remove = (self.vscroll / line_spacing).floor();
            // Number of lines to keep.
            // Here we round up, for the same reason

            let num_lines = (rc.size().height / line_spacing).ceil();
            // Check nothing is castastrophicly wrong, and then cast to an int.
            debug_assert!(lines_to_remove >= 0.0);
            let lines_to_remove = lines_to_remove as u64;
            debug_assert!(num_lines >= 0.0);
            let num_lines = num_lines as u64;

            dbg!((num_lines, lines_to_remove));

            // Next we generate the `text_layout`, which is the text + the
            // formatting (I think)
            //
            // TODO: Only pull the bit of the
            // rope that's on screen. This is super wastefull, as we
            // do a full copy out of the rope to render but we need
            // to convert to string before we sent it into druid so
            // what we should do it see what needs to be painted (ie
            // is onscrean) and only copy that.
            let text_layout =
                self.get_layout(&mut rc.text(), &data.to_string(), env);

            // Now we start trying to draw the curser it is relative to the
            // text, and that's easier and more robust to do using
            // the text, rather than assuming monospace and winging
            // it.

            // When the text layout doesn't have the whole rope, this will need
            // to change, but here we convert from ropey'r char based system to
            // druid's bytes based system.
            let text_byte_idx = data.rope().char_to_byte(data.curser());

            // Now we get the position of the curser in the text
            let pos = text_layout.hit_test_text_position(text_byte_idx);

            // TODO: hot to fall back if pos==None
            // that'll happen if the rope is empty, and maybe other reasons
            if let Some(pos) = pos {
                let mut x = pos.point.x;

                // https://github.com/linebender/druid/issues/1105
                // means we can't trust pos.y, so this is the work around

                // If we are just past a newline, the position thinks we're on
                // the line above, so misreports. Here we abjust
                // it. y is fine as it is calculated
                // seperayly from rope data, which gets this right.
                if data.rope().chars_at(data.curser().saturating_sub(1)).next()
                    == Some('\n')
                {
                    x = 1.0;
                }

                // Create the curser line
                let top = Point::new(x, topy - self.vscroll);
                let bottom = Point::new(x, bottomy - self.vscroll);
                let line = Line::new(top, bottom);

                // Draw the curser
                rc.stroke(line, &cursor_color, 1.0)
            }
            // Draw the text
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

    // Line spacing is the difference between the top of one line and the top of
    // another. Their's probably a better way to get it, but this works for
    // now.
    fn get_line_spacing(
        &self,
        piet_text: &mut PietText,
        env: &Env,
    ) -> Option<f64> {
        let layout = self.get_layout(piet_text, "12\n45", env);
        let top = layout.hit_test_text_position(1)?.point.y;
        let bottom = layout.hit_test_text_position(4)?.point.y;
        Some(bottom - top)
    }
}
