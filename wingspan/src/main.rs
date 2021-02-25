mod paint;
mod widget;

use druid::widget::{Flex, TextBox};
use druid::{AppLauncher, Data, Lens, WidgetExt, WindowDesc};
use widget::EditWidget;
use wingspan_buffer::Buffer;

fn main() {
    #[derive(Clone, Data, Lens)]
    struct Appstate {
        left: String,
        right: Buffer,
    }

    // let my_editor = EditWidget::default().lens(Appstate::my_buf);
    let left = TextBox::new().lens(Appstate::left).expand_width();
    let right = EditWidget::default().lens(Appstate::right).expand_width();

    let app = || {
        Flex::row()
            .with_flex_child(left, 1.0)
            .with_flex_child(right, 1.0)
    };

    let window = WindowDesc::new(app);

    let state = Appstate {
        left: String::from(include_str!("main.rs")),
        right: Buffer::new_text(include_str!("main.rs")),
    };

    AppLauncher::with_window(window)
        .launch(state)
        .expect("Failed to launch");
}
