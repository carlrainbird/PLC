use serde::Deserialize;
use crate::composite_pattern::SOR;
use crate::composite_pattern::EOR;
use crate::composite_pattern::XIC;

// Define the Component trait/rules
pub trait Component {
    fn mode_select(&mut self,mode: Mode);
    //fn widget();
}

pub struct PowerFlow{
    enable_input: bool,
    enable_output: bool,
}

 pub enum Mode{
    Init,
    Pause,
    Step,
    Stop,
    Run,
    Edit,
 }
// Define the ComponentType enum
#[derive(Debug, Clone, Deserialize)] // Derive Clone for your enum
pub enum ComponentType {
    SOR(SOR),
    EOR(EOR),
    XIC(XIC),
}
