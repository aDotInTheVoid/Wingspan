use druid::{AppLauncher, LocalizedString, WindowDesc};
use std::{fs::File, io::BufReader, error::Error};
use textarea::TextArea;
use textedit::EditableText;

const WINDOW_TITLE: LocalizedString<EditableText> =
    LocalizedString::new("Hello World!");

fn main() -> Result<(), Box<dyn Error>>{
    let main = WindowDesc::new(TextArea::new)
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    let data = EditableText::from_reader(BufReader::new(File::open(
        "Cargo.toml",
    )?))?;

    AppLauncher::with_window(main)
        .launch(data)
        .expect("Failed to launch application");

    Ok(())
}
