use serde::Deserialize;
use crate::composite_pattern::SOR;
use crate::composite_pattern::EOR;
use crate::composite_pattern::XIC;

// Define the Component trait
pub trait Component {
    fn logic(& self){}
        // Default implementation for logic, which does nothing.
        // Leaf components can override this.
    
}
#[derive(Debug, Clone)] 
pub struct PowerFlow{
    pub enable_input: bool,
    pub enable_output: bool,
}

// Define the ComponentType enum
#[derive(Debug, Clone, Deserialize)] // Derive Clone for your enum
pub enum ComponentType {
    SOR(SOR),
    EOR(EOR),
    XIC(XIC),
}
