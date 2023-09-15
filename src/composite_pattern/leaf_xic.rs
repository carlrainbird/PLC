//component_pattern/leaf_xic.rs
use serde::Deserialize;
use crate::composite_pattern::Component;
//use crate::factories_gui::{ButtonFactory, ButtonWidget};
use super::component::Mode;
use super::component::PowerFlow; 

// data from the .yaml file that hold the order 
// of execution and PLC instructions
#[derive(Debug, Clone, Deserialize)] // Derive Clone for your enum
pub struct XIC {
    pub tag_name: String,
    pub data: bool,
}

pub struct ExamineIfClosed{
    xic: XIC,
    powerflow: PowerFlow,
    //widget: ??
}

impl ExamineIfClosed{
    fn init(mut xic: XIC, mut examine_if_closed: ExamineIfClosed  ){
        examine_if_closed.xic.tag_name = xic.tag_name; 
        //etc
    }
    fn new()->Self{
    // todo
    }
}

impl Component for ExamineIfClosed {
    fn mode_select(&mut self, mode: Mode) {
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
