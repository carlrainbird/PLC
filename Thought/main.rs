use composite_pattern::component::PowerFlow;
use composite_pattern::leaf_eor::EndOfRung;
use composite_pattern::leaf_sor::StartOfRung;
use composite_pattern::leaf_xic::ExamineIfClosed;
use factories_gui::{ButtonFactory, ButtonWidget};
use fltk::{app, window::Window, prelude::*};
use crate::parse::read_components_from_yaml;
use crate::composite_pattern::*;
mod layout_manager;
mod parse;
mod composite_pattern;
mod factories_gui;
mod memory_manager;

fn main() {
    // File name
    let file_path1 = "RungLogic.yaml";

    let rung_order = read_components_from_yaml(file_path1)
        .expect(&format!("Failed to read or parse {}", file_path1));


    let file_path2 = "Memory.yaml";
        let memory = read_components_from_yaml(file_path2)
        .expect(&format!("Failed to read or parse {}", file_path2));

    let app = app::App::default();
    let mut win = Window::default()
        .with_size(1500, 1000)
        .with_label("Your Window Wangarooi");

    let mut xpos = 0;
    let mut ypos;

    for component in &rung_order {
        xpos = xpos + 200;
        ypos = 100;
        match component {
            ComponentType::SOR(sor_data) => {
                // Create a builder for SOR
                let sor_builder = StartOfRung::new();
                // Use builder methods to set properties
                sor_builder
                    .identifier() // set a unique identifier
                    .tag_name(sor_data.tag_name.clone()) // Set the tag name from YAML
                    .data(false) // Set the data
                    .powerflow(PowerFlow {
                        enable_input: false,
                        enable_output: false,
                     })
                    .build(); // Build the SOR component
                //printdata(sor_data);
                // Create a button with an image
                let button_factory = ButtonFactory;
                let mut button_widget = button_factory.create_button();
                
                button_factory.set_position(&mut button_widget, xpos, ypos);
                button_factory.set_label(&mut button_widget, &sor_data.tag_name.clone());
                button_factory.attach_image(&mut button_widget, "C:/Developer/PLC/src/images/scoobe-outro.svg");
            }

            ComponentType::EOR(eor_data) => {
                // Create a builder for EOR
                let eor_builder = EndOfRung::new();

                // Use builder methods to set properties
                     eor_builder
                    .identifier() // set a unique identifier
                    .tag_name(eor_data.tag_name.clone()) // Set the tag name from YAML
                    .data(true) // Set the data
                    .powerflow(PowerFlow {
                        enable_input: false,
                        enable_output: false,
                     }) 
                    .build(); // Build the EOR component
            
                // Create a button with an image
                let button_factory = ButtonFactory;
                let mut button_widget = button_factory.create_button();
                button_factory.set_position(&mut button_widget, xpos, ypos);
                button_factory.set_label(&mut button_widget, &eor_data.tag_name.clone());
                button_factory.attach_image(&mut button_widget, "C:/Developer/PLC/src/images/ew.svg");
            }

            ComponentType::XIC(xic_data) => {
                // Create a builder for XIC
                let xic_builder = ExamineIfClosed::new();
                // Use builder methods to set properties
                xic_builder
                    .identifier() // set a unique identifier
                    .tag_name(xic_data.tag_name.clone()) // Set the tag name from YAML
                    .data(true) // Set the data
                    .powerflow(PowerFlow {
                        enable_input: false,
                        enable_output: false,
                     })
                    .build(); // Build the XIC component
                //printdata(xic_data);

                // Create a button with an image
                let button_factory = ButtonFactory;
                let mut button_widget = button_factory.create_button();
                button_factory.set_position(&mut button_widget, xpos, ypos);
                button_factory.set_label(&mut button_widget, &xic_data.tag_name.clone());
                button_factory.attach_image(&mut button_widget, "C:/Developer/PLC/src/images/arrow.svg");
            }
        }
    }

    win.end();
    win.show();

    app.run().unwrap();
}
