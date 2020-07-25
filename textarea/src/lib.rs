// All of this is so naive.
// https://github.com/linebender/druid/blob/d84b8c50f55a28282f1e69ef51c651e70d83f9c3/druid/src/widget/textbox.rs

use druid::{
    piet::{FontBuilder, PietText, PietTextLayout, Text, TextLayoutBuilder},
    theme,
    widget::prelude::*,
    KeyEvent, Point, KeyModifiers,
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
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EditableText, _env: &Env) {
        // Fuck it
        ctx.request_focus();

        if let Event::KeyDown(KeyEvent {
            key_code,
            is_repeat: false,
            mods: KeyModifiers {
                shift,
                ..
            },
            ..
        }) = event
        {
            let shift = *shift;
            use druid::KeyCode::*;

            //TODO: clean up
            match key_code {
                ArrowLeft => data.left(),
                ArrowRight => data.right(),

                Delete | Backspace => data.delete(),

                // No CRLF, fight me
                Return => data.insert('\n'),
                Space => data.insert(' '),

                // Numberics
                // TODO: !@#$%^&*(){}[], in a way independent of keyboard layout
                Key0 => data.insert('0'),
                Key1 => data.insert('1'),
                Key2 => data.insert('2'),
                Key3 => data.insert('3'),
                Key4 => data.insert('4'),
                Key5 => data.insert('5'),
                Key6 => data.insert('6'),
                Key7 => data.insert('7'),
                Key8 => data.insert( '8'),
                Key9 => data.insert('9'),

                KeyQ => data.insert(if shift {'Q'} else {'q'}),
                KeyW => data.insert(if shift {'W'} else {'w'}),
                KeyE => data.insert(if shift {'E'} else {'e'}),
                KeyR => data.insert(if shift {'R'} else {'r'}),
                KeyT => data.insert(if shift {'T'} else {'t'}),
                KeyY => data.insert(if shift {'Y'} else {'y'}),
                KeyU => data.insert(if shift {'U'} else {'u'}),
                KeyI => data.insert(if shift {'I'} else {'i'}),
                KeyO => data.insert(if shift {'O'} else {'o'}),
                KeyP => data.insert(if shift {'P'} else {'p'}),
                // Middle row
                KeyA => data.insert(if shift {'A'} else {'a'}),
                KeyS => data.insert(if shift {'S'} else {'s'}),
                KeyD => data.insert(if shift {'D'} else {'d'}),
                KeyF => data.insert(if shift {'F'} else {'f'}),
                KeyG => data.insert(if shift {'G'} else {'g'}),
                KeyH => data.insert(if shift {'H'} else {'h'}),
                KeyJ => data.insert(if shift {'J'} else {'j'}),
                KeyK => data.insert(if shift {'K'} else {'k'}),
                KeyL => data.insert(if shift {'L'} else {'l'}),
                // Bottom Row
                KeyZ => data.insert(if shift {'Z'} else {'z'}),
                KeyX => data.insert(if shift {'X'} else {'x'}),
                KeyC => data.insert(if shift {'C'} else {'c'}),
                KeyV => data.insert(if shift {'V'} else {'v'}),
                KeyB => data.insert(if shift {'B'} else {'b'}),
                KeyN => data.insert(if shift {'N'} else {'n'}),
                KeyM => data.insert(if shift {'M'} else {'m'}),



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
