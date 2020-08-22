mod paint;
mod widget;

use druid::{AppLauncher, WindowDesc};
use widget::EditWidget;
use wingspan_buffer::Buffer;

fn main() {
    let window = WindowDesc::new(EditWidget::default);
    let state = Buffer::new_text(include_str!("main.rs"));
    AppLauncher::with_window(window)
        .launch(state)
        .expect("Failed to launch");
}
