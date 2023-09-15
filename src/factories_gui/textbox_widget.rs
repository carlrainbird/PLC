use fltk::input::Input as FltkInput;
use fltk::prelude::*;

// Define a custom trait for textbox creation
pub trait TextboxWidget {
    fn create_textbox(&self) -> FltkInput;
}

// Implement the trait for the TextboxFactory
pub struct TextboxFactory;

impl TextboxWidget for TextboxFactory {
    fn create_textbox(&self) -> FltkInput {
        // Create and configure an Input (Textbox) widget and return it directly
        FltkInput::new(10, 10, 200, 30, "Text")
    }
}
