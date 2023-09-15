//component_pattern/leaf_eor.rs
use serde::Deserialize;  
use crate::composite_pattern::Component;
//use crate::factories_gui::{ButtonFactory, ButtonWidget};
use super::component::Mode;
use super::component::PowerFlow; 

// the serde data
#[derive(Debug, Clone, Deserialize)] // Derive Clone for your enum
pub struct EOR {
    pub tag_name: String,
    pub data: bool,
}
pub struct EndOfRung{
    eor : EOR,  // serde data
    powerflow: PowerFlow,
}

impl EndOfRung{
    fn new()->Self{
        
    }
}

impl Component for EndOfRung {
    fn mode_select(&mut self, mode: Mode) {
        match mode {
            Mode::Init => {                
                println!("Init for EOR: {:?}", self.eor);
                todo!();
            },

            Mode::Pause => {
                println!("Logic for EOR: {:?}", self.eor);
                // Add more logic here if needed
                todo!();
            },

            Mode::Step => {
                println!("Single step EOR: {:?}", self.eor);
                // Add more logic here if needed
                todo!();
            },

            Mode::Run => {
                println!("Run for EOR: {:?}", self.eor);
                // Add more logic here if needed
                todo!();
            },

            Mode::Stop => {
                println!("Stop for EOR: {:?}", self.eor);
                // Add more logic

                todo!();
            },

            Mode::Edit => {
                println!("Edit for EOR: {:?}", self.eor);
                // Add more logic here if needed
                todo!();
            },

        }
    }
}
fn _abc(){

}