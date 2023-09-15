//component_pattern/leaf_xic.rs
use crate::composite_pattern::Component;
//use crate::factories_gui::{ButtonFactory, ButtonWidget};
use super::component::Mode; 

pub struct Rung {
    rung_parts: vec![Component],
}

impl Rung{
    fn init(){
        
    }
    fn new()->Self{
    // todo
    }
    fn add_leaf(component:dyn Component){
        //add it to the rung in the correct order for execution
    }
    fn remove_leaf(component: dyn Component){
        //remove it from the rung in the correct order for execution
    }
}

impl Component for Rung {
    fn operating_mode_select(&mut self, mode: Mode) {
        match mode {
            Mode::Init => {
                println!("Logic for XIC: {:?}", self.xic);
                todo!();
            },
            Mode::Pause => {
                println!("Logic for XIC: {:?}", self.xic);
                // Add more logic here if needed
                self.xic.eno = true;
                todo!();
            },
            Mode::Step => {
                println!("Single step XIC: {:?}", self.xic);
                // Add more logic here if needed
                todo!();
            },
            Mode::Run => {
                println!("Run for XIC: {:?}", self.xic);
                // Add more logic here if needed
                todo!();
            },
            Mode::Stop => {
                println!("Stop for XIC: {:?}", self.xic);
                // Add more logic
                todo!();
            },
            Mode::Edit => {
                println!("Edit for XIC: {:?}", self.xic);
                // Add more logic here if needed
                todo!();
            },
        }
    }
}