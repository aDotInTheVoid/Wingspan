use druid::widget::{Button, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Hello World!");

//mod ropey_text_box;

#[derive(Clone, Data, Lens)]
struct AppState {
    _data: String,
}

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = AppState {
        _data: String::with_capacity(10),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<AppState> {
    // A label for the editor, so it isn't just the textbox
    let label = Label::new("This is part of the editr");
    // a textbox that modifies `name`.
    let editor_box = TextBox::new()
        .with_placeholder("Who are we greeting?")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(AppState::_data);

    // Create the overall editor window
    let editor = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(editor_box);

    // Placeholder file tree for new
    let file_tree = Flex::column()
        .with_child(Button::new("A File"))
        .with_child(Button::new("Another FIle"));

    // Probably where we list the files
    let top_bar = Flex::row()
        .with_child(Button::new("A file"))
        .with_child(Button::new("Another File"));

    // Other random info, just want to be sure it's in the layout
    let bottom_bar = Flex::row()
        .with_spacer(10.0)
        .with_child(Button::new("Some bottom button"))
        .with_flex_spacer(10.0)
        .with_child(Button::new("Another bottom button"))
        .with_spacer(10.0);

    let layout: Flex<AppState> = Flex::column()
        // Top bit is flex, as it fills all space.
        // Bottom bar is fixed width, so not flex
        .with_flex_child(
            // The main view has the files on the left, and the editor on the
            // right. Files are fixed, editor is flex
            Flex::row().with_child(file_tree).with_flex_child(
                Flex::column()
                    .with_child(top_bar.align_left())
                    .with_flex_child(editor, 1.0)
                    .expand_width(),
                1.0,
            ),
            //.fix_height()
            1.0,
        )
        .with_child(bottom_bar);

    layout.debug_paint_layout()
}
