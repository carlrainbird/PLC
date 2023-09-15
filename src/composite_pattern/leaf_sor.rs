//component_pattern/leaf_sor.rs
use serde::Deserialize;
use crate::composite_pattern::Component;
//use crate::factories_gui::{ButtonFactory, ButtonWidget};
use super::component::Mode;
use super::component::PowerFlow; 

#[derive(Debug, Clone, Deserialize)] // Derive Clone for your enum
pub struct SOR {
    pub tag_name: String,
    pub data: bool,
}

pub struct StartOfRung{
    sor: SOR,
    powerflow: PowerFlow,
}

impl StartOfRung{
    fn new()->Self{
        
    }    
}


impl Component for StartOfRung {
    fn mode_select(&mut self, mode: Mode) {
        match mode {
            Mode::Init => {
                println!("Init for SOR: {:?}", self.sor);
                todo!();
            },
            Mode::Pause => {
                println!("Logic for SOR: {:?}", self.sor);
                // Add more logic here if needed
                self.sor.eno = true;
                todo!();
            },
            Mode::Step => {
                println!("Single step SOR: {:?}", self.sor);
                // Add more logic here if needed
                todo!();
            },
            Mode::Run => {
                println!("Run for SOR: {:?}", self.sor);
                // Add more logic here if needed
                todo!();
            },
            Mode::Stop => {
                println!("Stop for SOR: {:?}", self.sor);
                // Add more logic
                todo!();
            },
            Mode::Edit => {
                println!("Edit for SOR: {:?}", self.sor);
                // Add more logic here if needed
                todo!();
            },
        }
    }
}