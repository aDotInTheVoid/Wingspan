use druid::{
    kurbo::Line,
    piet::{
        FontFamily, FontWeight, PietText, PietTextLayout, Text, TextAttribute,
        TextLayout, TextLayoutBuilder,
    },
    Env, PaintCtx, Point, RenderContext,
};
use std::cmp::min;

use crate::widget::EditWidget;
use wingspan_buffer::Buffer;

impl EditWidget {
    pub(crate) fn paint_internal(
        &mut self,
        ctx: &mut PaintCtx<'_, '_, '_>,
        data: &Buffer,
        _env: &Env,
    ) {
        let global_rope = data.rope();

        let font_size = 14.0;
        let background_color = druid::piet::Color::BLACK;
        let cursor_color = druid::piet::Color::WHITE;

        let clip_rect = ctx.size().to_rect();
        ctx.fill(clip_rect, &background_color);

        ctx.with_save(|rc| {
            rc.clip(clip_rect);

            let line_spacing = self.get_line_spacing(&mut rc.text());

            // The maximum number of lines that can on the screen.
            // We round up, as we want to include lines partialy on the screen
            let max_lines_onscreen = (rc.size().height / line_spacing).ceil();
            debug_assert!(max_lines_onscreen >= 0.0);
            let max_lines_onscreen = max_lines_onscreen as usize;

            // Ensure their is text on screen
            match global_rope.len_lines().checked_sub(max_lines_onscreen) {
                Some(lines_above_fold) => {
                    let pix_above_fold = line_spacing * lines_above_fold as f64;
                    // Float isn't Ord, can't use min.
                    if self.vscroll > pix_above_fold {
                        self.vscroll = pix_above_fold
                    }
                }
                // More lines onscrean than in the global rope, so dont scroll.
                None => {
                    self.vscroll = 0.0;
                }
            }

            // Number of lines to remove from the top.
            // Round down, so lines partialy in display are still rendered
            let lines_to_remove = (self.vscroll / line_spacing).floor();
            // Check nothing is castastrophicly wrong, and then cast to an int.
            debug_assert!(lines_to_remove >= 0.0);
            let lines_to_remove = lines_to_remove as usize;

            // self.vscroll is the amount of scrolling done overall.
            // local_vscroll is how far up we need to move the text.
            let pixels_removed = lines_to_remove as f64 * line_spacing;
            let local_vscroll = self.vscroll - pixels_removed;
            let text_pos = Point::new(0.0, 0. - local_vscroll);

            // Extract the onscrean rope
            let text_start_idx = global_rope.line_to_char(lines_to_remove);
            let text_end_idx = global_rope.line_to_char(min(
                lines_to_remove + max_lines_onscreen,
                global_rope.len_lines(),
            ));
            let local_rope = global_rope.slice(text_start_idx..text_end_idx);

            // Next we generate the `text_layout`, which is the text + the
            // formatting (I think)
            let text_layout =
                self.get_layout(&mut rc.text(), &local_rope.to_string());

            // Bytewise index of the curser position in the local rope
            // If this checked_sub returns None, it means the curser is above
            // the local rope.
            if let Some(text_byte_idx) = global_rope
                .char_to_byte(data.curser())
                .checked_sub(global_rope.char_to_byte(text_start_idx))
            {
                let local_len = local_rope.len_bytes();

                // The line number in the local rope.
                let local_lineno =
                    global_rope.char_to_line(data.curser()) - lines_to_remove;

                // Check the curser is onscreen
                if local_len >= text_byte_idx
                    && local_lineno < text_layout.line_count()
                {
                    // Now we get the position of the curser in the text
                    let pos = text_layout.hit_test_text_position(text_byte_idx);

                    let local_lineno = local_lineno as f64;

                    // https://github.com/linebender/druid/issues/1105
                    // pos.y isn't platform independent, so we use this
                    // instead.
                    let top_y = local_lineno * line_spacing - local_vscroll;
                    let bottom_y = top_y + font_size;
                    let mut x = pos.point.x;

                    // If we're on a newline, the x is from the previous
                    // line. TODO: Index the local rope instead
                    if global_rope
                        .chars_at(data.curser().saturating_sub(1))
                        .next()
                        == Some('\n')
                    {
                        x = 1.0;
                    }

                    let top = Point::new(x, top_y);
                    let bottom = Point::new(x, bottom_y);
                    let line = Line::new(top, bottom);
                    // TODO: Make width configurable
                    rc.stroke(line, &cursor_color, 1.0);
                }
            }
            rc.draw_text(&text_layout, text_pos);
        });
    }

    fn get_layout(
        &self,
        piet_text: &mut PietText,
        text: &str,
    ) -> PietTextLayout {
        let font_size = 14.0;
        let default_colors = druid::piet::Color::WHITE;
        let font = FontFamily::MONOSPACE;

        piet_text
            .new_text_layout(text.to_string())
            .font(font, font_size)
            .default_attribute(TextAttribute::TextColor(default_colors))
            .default_attribute(TextAttribute::Weight(FontWeight::MEDIUM))
            .build()
            .unwrap()
    }

    // Line spacing is the difference between the top of one line and the top of
    // another. Their's probably a better way to get it, but this works for
    // now.
    fn get_line_spacing(&self, piet_text: &mut PietText) -> f64 {
        let layout = self.get_layout(piet_text, "12\n45");
        let top = layout.hit_test_text_position(1).point.y;
        let bottom = layout.hit_test_text_position(4).point.y;
        bottom - top
    }
}
