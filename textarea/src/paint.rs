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
use std::cmp::min;

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
        //=================================================================
        // Preamble
        //=================================================================

        let global_rope = data.rope();

        // Pull some theme stuff
        let font_size = env.get(theme::TEXT_SIZE_NORMAL);
        let background_color = env.get(theme::BACKGROUND_LIGHT);
        let text_color = env.get(theme::LABEL_COLOR);

        // First we paint the background
        // This is so we can add padding later, see druid::widget::TextBox for
        // more
        let clip_rect = ctx.size().to_rect();
        ctx.fill(clip_rect, &background_color);

        // Move into a save, and render most of the stuff inside of it.
        ctx.with_save(|rc| {
            rc.clip(clip_rect);
            // TODO: Log an error if we use the default.
            // This in the top to top spacing
            let line_spacing =
                self.get_line_spacing(&mut rc.text(), env).unwrap_or(19.0);

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

            //=================================================================
            // Partial Rendering Calculatings
            //=================================================================

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
            // TODO: Figure out where this 0.8 comes from
            let text_height = font_size * 0.8;
            let text_pos = Point::new(0.0, text_height - local_vscroll);

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
                self.get_layout(&mut rc.text(), &local_rope.to_string(), env);

            self.paint_curser(
                rc,
                env,
                data,
                global_rope,
                text_start_idx,
                lines_to_remove,
                &text_layout,
                line_spacing,
                local_vscroll,
                font_size,
                local_rope.len_bytes(),
            );

            // Draw the text
            rc.draw_text(&text_layout, text_pos, &text_color);
        });
    }

    // Returns Some(()) if the curser was rendered, otherwise None
    // TODO: Fewer args. It uses a load of stuff from paint_internal, but needs
    // to be seperate to do the ? sugar.
    #[allow(clippy::too_many_arguments)]
    fn paint_curser(
        &mut self,
        ctx: &mut PaintCtx,
        env: &Env,
        data: &EditableText,
        global_rope: &ropey::Rope,
        text_start_idx: usize,
        lines_to_remove: usize,
        text_layout: &PietTextLayout,
        line_spacing: f64,
        local_vscroll: f64,
        font_size: f64,
        local_len: usize,
    ) -> Option<()> {
        let cursor_color = env.get(theme::CURSOR_COLOR);

        // Bytewise index of the curser position in the local rope
        // If this checked_sub returns None, it means the curser is above the
        // local rope.
        let text_byte_idx = global_rope
            .char_to_byte(data.curser())
            .checked_sub(global_rope.char_to_byte(text_start_idx))?;

        // TODO: Check before call, calculate text_byte_idx in caller
        // TODO: why?
        if local_len < text_byte_idx {
            return None;
        }

        // The line number in the local rope.
        let local_lineno =
            global_rope.char_to_line(data.curser()) - lines_to_remove;
        // If the curser is below whats onsren, dont render it.
        if local_lineno >= text_layout.line_count() {
            return None;
        }
        let local_lineno = local_lineno as f64;

        // Now we get the position of the curser in the text
        let curser_pos = text_layout.hit_test_text_position(text_byte_idx);

        // https://github.com/linebender/druid/issues/1105
        // pos.y isn't platform independent, so we use this instead.
        let topy = local_lineno * line_spacing - local_vscroll;
        let bottomy = topy + font_size;

        // TODO: hot to fall back if pos==None
        // that'll happen if the rope is empty, and maybe other reasons
        if let Some(pos) = curser_pos {
            let mut x = pos.point.x;

            // If we're on a newline, the x is from the previous line
            // TODO: Don't index into the global rope
            if global_rope.chars_at(data.curser().saturating_sub(1)).next()
                == Some('\n')
            {
                x = 1.0;
            }

            // Create the curser line
            let top = Point::new(x, topy);
            let bottom = Point::new(x, bottomy);
            let line = Line::new(top, bottom);

            // Draw the curser
            // TODO: Make width configurable
            ctx.stroke(line, &cursor_color, 1.0);

            return Some(());
        }
        None
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
