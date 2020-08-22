mod paint;
mod widget;

use druid::{AppLauncher, WindowDesc};
use std::{fs::File, io::BufReader};
use widget::EditWidget;
use wingspan_buffer::Buffer;

fn main() {
    let window = WindowDesc::new(EditWidget::default);
    let state = Buffer::from_reader(BufReader::new(
        File::open(std::env::args().nth(1).unwrap()).unwrap(),
    ))
    .unwrap();

    AppLauncher::with_window(window)
        .launch(state)
        .expect("Failed to launch");
}
