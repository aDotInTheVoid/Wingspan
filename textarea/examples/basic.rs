use color_eyre::eyre::Result;
use druid::{AppLauncher, LocalizedString, WindowDesc};
use std::io;
use std::io::prelude::*;
use std::{error::Error, fs::File, io::BufReader};
use textarea::TextArea;
use textedit::EditableText;

const WINDOW_TITLE: LocalizedString<EditableText> =
    LocalizedString::new("Hello World!");

fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;

    let main = WindowDesc::new(TextArea::new)
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    print!("Which file to open: ");
    io::stdout().flush()?;
    let mut file_string = String::new();
    io::stdin().read_line(&mut file_string)?;

    let data = EditableText::from_reader(BufReader::new(File::open(
        file_string.trim(),
    )?))?;

    AppLauncher::with_window(main)
        .launch(data)
        .expect("Failed to launch application");

    Ok(())
}
