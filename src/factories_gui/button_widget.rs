use fltk::button::Button as FltkButton;
use fltk::enums::Color;
use fltk::image::SvgImage;
use fltk::prelude::*;

// Define a custom trait for button creation
pub trait ButtonWidget {
    fn create_button(&self) -> FltkButton;
    fn set_position(&self, button: &mut FltkButton, x: i32, y: i32);
    fn set_label(&self, button: &mut FltkButton, label: &str);
    fn attach_image(&self, button: &mut FltkButton, image_path: &str);
}

pub struct ButtonFactory;

impl ButtonWidget for ButtonFactory {
    fn create_button(&self) -> FltkButton {
        // Create a default Button widget
        let mut button = FltkButton::new(10, 10, 80, 30, "Button");
        button.set_color(Color::from_u32(0xCCCCCC));
        button
    }
    // must be set according to the previous component
    fn set_position(&self, button: &mut FltkButton, x: i32, y: i32) {
        // Set the position of the button
        button.set_pos(x, y);
    }
    // according to tag
    fn set_label(&self, button: &mut FltkButton, label: &str) {
        // Set the label of the button
        button.set_label(label);
    }
    // according to type of component
    fn attach_image(&self, button: &mut FltkButton, image_path: &str) {                      
        // Load the image from the specified path
        let mut image = SvgImage::load(image_path).unwrap();
        image.scale(200, 200, true, true);

        // Attach the image to the button
        button.set_image(Some(image));

    }
}
