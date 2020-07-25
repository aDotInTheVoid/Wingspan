use textarea::TextArea;
use textedit::EditableText;
use druid::{LocalizedString, WindowDesc, Data, Lens, AppLauncher};


const WINDOW_TITLE: LocalizedString<EditableText> = LocalizedString::new("Hello World!");



fn main() {
    let main = WindowDesc::new(TextArea::new)
    .title(WINDOW_TITLE)
    .window_size((400.0, 400.0));

    let data = EditableText::new();

    AppLauncher::with_window(main)
        .launch(data)
        .expect("Failed to launch application");
}