// All of this is so naive.
// https://github.com/linebender/druid/blob/d84b8c50f55a28282f1e69ef51c651e70d83f9c3/druid/src/widget/textbox.rs
// 

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
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EditableText, env: &Env) {
        // Fuck it
        ctx.request_focus();

        if let Event::KeyDown(KeyEvent {
            key_code,
            is_repeat: false,
            ..
        }) = event
        {
            use druid::KeyCode::*;

            

            match key_code {
                ArrowLeft => data.left(),
                ArrowRight => data.right(),

                Delete| Backspace => data.delete(),

                Key0 => data.insert('0'),
                Key1 => data.insert('1'),
                Key2 => data.insert('2'),
                Key3 => data.insert('3'),
                Key4 => data.insert('4'),
                Key5 => data.insert('5'),
                Key6 => data.insert('6'),
                Key7 => data.insert('7'),
                Key8 => data.insert('8'),
                Key9 => data.insert('9'),
                KeyQ => data.insert('Q'),
                KeyW => data.insert('W'),
                KeyE => data.insert('E'),
                KeyR => data.insert('R'),
                KeyT => data.insert('T'),
                KeyY => data.insert('Y'),
                KeyU => data.insert('U'),
                KeyI => data.insert('I'),
                KeyO => data.insert('O'),
                KeyP => data.insert('P'),
                KeyA => data.insert('A'),
                KeyS => data.insert('S'),
                KeyD => data.insert('D'),
                KeyF => data.insert('F'),
                KeyG => data.insert('G'),
                KeyH => data.insert('H'),
                KeyJ => data.insert('J'),
                KeyK => data.insert('K'),
                KeyL => data.insert('L'),
                KeyZ => data.insert('Z'),
                KeyX => data.insert('X'),
                KeyC => data.insert('C'),
                KeyV => data.insert('V'),
                KeyB => data.insert('B'),
                KeyN => data.insert('N'),
                KeyM => data.insert('M'),


                // No CRLF, fight me
                Return => data.insert('\n'),

                _ => {}
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
        let height = env.get(theme::BORDERED_WIDGET_HEIGHT);
        let background_color = env.get(theme::BACKGROUND_LIGHT);
        let selection_color = env.get(theme::SELECTION_COLOR);
        let text_color = env.get(theme::LABEL_COLOR);
        let placeholder_color = env.get(theme::PLACEHOLDER_COLOR);
        let cursor_color = env.get(theme::CURSOR_COLOR);

        let is_focused = ctx.is_focused();

        let border_color = if is_focused {
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
