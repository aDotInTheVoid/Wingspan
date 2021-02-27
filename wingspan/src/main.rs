mod paint;
mod widget;

use druid::widget::{Flex, Scroll, TextBox};
use druid::{AppLauncher, Data, Lens, WidgetExt, WindowDesc};
use widget::EditWidget;
use wingspan_buffer::Buffer;

const LONG_TEXT: &str = include_str!("../../assets/long_log.txt");

fn main() {
    #[derive(Clone, Data, Lens)]
    struct Appstate {
        left: String,
        right: Buffer,
    }

    // let my_editor = EditWidget::default().lens(Appstate::my_buf);
    let left = Scroll::new(TextBox::new().expand_width())
        .vertical()
        .lens(Appstate::left);
    let right = EditWidget::default().lens(Appstate::right).expand_width();

    let app = || {
        Flex::row()
            .with_flex_child(left, 1.0)
            .with_flex_child(right, 1.0)
    };

    let window = WindowDesc::new(app);

    let state = Appstate {
        left: String::from(LONG_TEXT),
        right: Buffer::new_text(LONG_TEXT),
    };

    AppLauncher::with_window(window)
        .launch(state)
        .expect("Failed to launch");
}
