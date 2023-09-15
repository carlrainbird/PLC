use composite_pattern::leaf_eor::EndOfRung;
use factories_gui::{ButtonWidget,TextboxFactory,TextboxWidget};
use fltk::{app,window::Window,prelude::*};
use crate::parse::read_components_from_yaml;
use crate::composite_pattern::*;
use crate::factories_gui::ButtonFactory; // Import the button factory // Import the image factory
use crate::layout_manager::Position;
mod layout_manager;
mod parse;
mod composite_pattern;
mod factories_gui;

fn main() {

    //let abc = layout_manager::coordinate::set_position(10,10);

    // File name
    let file_path = "RungLogic.yaml";
    
    let component_instances = read_components_from_yaml(file_path)
        .expect(&format!("Failed to read or parse {}", file_path));

    let app = app::App::default();
    let mut win = Window::default()
        .with_size(1000, 1000)
        .with_label("Your Window Wangarooi");
let mut xpos = 0;
let mut ypos = 0; 


    for component in &component_instances {
        xpos = xpos + 200;
        ypos = 100;
        match component {
            ComponentType::SOR(_sor) => {
                // Create a button with an image
                let button_factory = ButtonFactory;                
                let mut button_widget = button_factory.create_button();
                button_factory.set_position(&mut button_widget, xpos, ypos);
                button_factory.set_label(&mut button_widget, "Custom Label");
                button_factory.attach_image(&mut button_widget, "C:/Developer/PLC/src/images/scoobe-outro.svg")
            }

            ComponentType::EOR(_eor) => {

                just deal with the function builder pattern ??
                // Create a button with an image
                //let com = EndOfRung;
                let button_factory = ButtonFactory;                
                let mut button_widget = button_factory.create_button();
                button_factory.set_position(&mut button_widget, xpos, ypos);
                button_factory.set_label(&mut button_widget, "Custom Label");
                button_factory.attach_image(&mut button_widget, "C:/Developer/PLC/src/images/ew.svg")
            }

            ComponentType::XIC(_xic) => {
                // Create a button with an image
                let button_factory = ButtonFactory;                
                let mut button_widget = button_factory.create_button();
                button_factory.set_position(&mut button_widget, xpos, ypos);
                button_factory.set_label(&mut button_widget, "Custom Label");
                button_factory.attach_image(&mut button_widget, "C:/Developer/PLC/src/images/arrow.svg")            
            }
        }
    }

    win.end();
    win.show();

    app.run().unwrap();
}
