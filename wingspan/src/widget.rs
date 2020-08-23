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

#[cfg(test)]
mod test {
    use super::*;
    use druid::{
        keyboard_types::Key, tests::harness::Harness, KeyEvent, WidgetExt,
    };
    use std::{
        fs::File,
        io::{self, BufReader, Read},
        path::PathBuf,
    };
    #[test]
    fn default_state_is_empty() {
        let widget = EditWidget { vscroll: 0.0 }.with_id(WidgetId::next());

        Harness::create_simple(Buffer::new(), widget, |harness| {
            harness.send_initial_events();
            assert_eq!(harness.data().to_string(), "");
        });
    }

    #[test]
    fn open_sonnet_18() -> io::Result<()> {
        let root = EditWidget { vscroll: 0.0 }.with_id(WidgetId::next());
        let data = Buffer::from_reader(BufReader::new(File::open(
            "./texts/sonnet18.txt",
        )?))?;
        Harness::create_simple(data, root, |harness| {
            harness.send_initial_events();
            assert_eq!(
                harness.data().to_string(),
                include_str!("../texts/sonnet18.txt")
            )
        });
        Ok(())
    }

    #[test]
    fn open_macbeth() -> io::Result<()> {
        let root = EditWidget { vscroll: 0.0 }.with_id(WidgetId::next());
        let data = Buffer::from_reader(BufReader::new(File::open(
            "./texts/macbeth.txt",
        )?))?;
        Harness::create_simple(data, root, |harness| {
            harness.send_initial_events();
            assert_eq!(
                harness.data().to_string(),
                include_str!("../texts/macbeth.txt")
            )
        });
        Ok(())
    }

    #[test]
    fn open_shakespeare() -> io::Result<()> {
        let root = EditWidget { vscroll: 0.0 }.with_id(WidgetId::next());
        let data = Buffer::from_reader(BufReader::new(File::open(
            "./texts/shakespeare.txt",
        )?))?;
        Harness::create_simple(data, root, |harness| {
            harness.send_initial_events();
            harness.paint();
            assert_eq!(
                harness.data().to_string(),
                include_str!("../texts/shakespeare.txt")
            )
        });
        Ok(())
    }
    #[test]
    fn sonnet_img() -> io::Result<()> {
        test_image_open("sonnet1", "./texts/sonnet18.txt", |_| {})
    }
    #[test]
    fn sonnet_img_move() -> io::Result<()> {
        test_image_open("sonnet2", "./texts/sonnet18.txt", |harness| {
            let mut ke: KeyEvent = Default::default();
            ke.key = Key::Character("a".to_owned());
            harness.event(Event::KeyDown(ke));
        })
    }

    #[test]
    fn shakespeare_scroll() -> io::Result<()> {
        Ok(test_image_check(
            "shakespere_scroll",
            |wid| {
                wid.vscroll = 100.0;
            },
            Buffer::from_reader(BufReader::new(File::open(
                "./texts/shakespeare.txt",
            )?))?,
            |harness| {
                let mut ke: KeyEvent = Default::default();
                ke.key = Key::ArrowRight;
                for _ in 0..300 {
                    harness.event(Event::KeyDown(ke.clone()))
                }
                ke.key = Key::Backspace;
                for _ in 0..10 {
                    harness.event(Event::KeyDown(ke.clone()))
                }
                ke.key = Key::Character("x".to_owned());
                for _ in 0..10 {
                    harness.event(Event::KeyDown(ke.clone()))
                }
            },
        ))
    }

    fn test_image_open(
        name: &str,
        text_file: &str,
        harness_events: impl FnMut(&mut Harness<Buffer>),
    ) -> io::Result<()> {
        test_image_check(
            name,
            |_| {},
            Buffer::from_reader(BufReader::new(File::open(text_file)?))?,
            harness_events,
        );
        Ok(())
    }

    fn test_image_check(
        name: &str,
        mut widget_setup: impl FnMut(&mut EditWidget),
        data: Buffer,
        mut harness_events: impl FnMut(&mut Harness<Buffer>),
    ) {
        let mut root = Default::default();
        widget_setup(&mut root);
        Harness::create_with_render(
            data,
            root,
            Size::new(400.0, 400.0),
            |harness| {
                harness.send_initial_events();
                harness.just_layout();
                harness_events(harness);
                harness.paint()
            },
            |render| {
                let mut name = name.to_owned();
                name.push_str(".png");
                let mut file = PathBuf::from("./test_images");
                file.push(std::env::consts::OS);
                file.push(name);
                if std::env::var("WINGSPAN_UPDATE_IMAGES") == Ok("1".to_owned())
                {
                    // Save the png
                    render.into_png(file).unwrap();
                } else {
                    let mut true_image = File::open(file).unwrap();
                    let false_image = tempfile::NamedTempFile::new().unwrap();
                    render.into_png(false_image.path()).unwrap();
                    let mut true_bytes = Vec::new();
                    true_image.read_to_end(&mut true_bytes).unwrap();
                    let mut false_bytes = Vec::new();
                    false_image
                        .as_file()
                        .read_to_end(&mut false_bytes)
                        .unwrap();
                    assert_eq!(true_bytes, false_bytes);
                }
            },
        )
    }
}
