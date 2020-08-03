use druid::{AppLauncher, LocalizedString, WindowDesc};
use textarea::TextArea;
use textedit::EditableText;

const WINDOW_TITLE: LocalizedString<EditableText> =
    LocalizedString::new("Hello World!");

fn lines(n: usize) -> String {
    let mut buff = String::with_capacity(n * 3);
    for i in 1..=n {
        buff.push_str(&i.to_string());
        buff.push_str("\n");
    }
    buff
}

fn main() {
    let main = WindowDesc::new(TextArea::new)
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    let data = EditableText::new_text(&lines(100));

    AppLauncher::with_window(main)
        .launch(data)
        .expect("Failed to launch application");
}
