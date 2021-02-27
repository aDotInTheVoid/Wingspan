use druid::widget::prelude::*;
use wingspan_buffer::Buffer;

#[derive(Default, Clone, Copy)]
pub struct EditWidget {
    pub vscroll: f64,
}

impl Widget<Buffer> for EditWidget {
    /// Recieve an event from the OS
    /// Most likely a keypress
    /// Here is where we can edit the rope
    fn event(
        &mut self,
        ctx: &mut EventCtx<'_, '_>,
        event: &Event,
        data: &mut Buffer,
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
                    ArrowUp => data.up(),
                    ArrowDown => data.down(),

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
            Event::Wheel(druid::MouseEvent { wheel_delta, .. }) => {
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
        _data: &Buffer,
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
        _old_data: &Buffer,
        _data: &Buffer,
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
        _data: &Buffer,
        _env: &Env,
    ) -> Size {
        bc.max()
    }

    // Rendering is in a seperate file for consiseness.
    fn paint(
        &mut self,
        ctx: &mut PaintCtx<'_, '_, '_>,
        data: &Buffer,
        env: &Env,
    ) {
        self.paint_internal(ctx, data, env);
    }
}
