// Copyright 2020 The Wingspan Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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

    let path = match std::env::args().nth(1) {
        Some(t) => t,
        None => {
            print!("Which file to open: ");
            io::stdout().flush()?;
            let mut file_string = String::new();
            io::stdin().read_line(&mut file_string)?;
            file_string
        }
    };

    let data = EditableText::from_reader(BufReader::new(File::open(path)?))?;

    AppLauncher::with_window(main)
        .launch(data)
        .expect("Failed to launch application");

    Ok(())
}
