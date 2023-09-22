use rand::Rng;
use serde::Deserialize;
use crate::composite_pattern::Component;
use super::component::PowerFlow;

// Define the EOR struct with serde data
#[derive(Debug, Clone, Deserialize)] // Derive Clone for your enum
pub struct XIC {
    pub tag_name: String,
}

// Define the EndOfRung component
#[derive(Debug, Clone)]
pub struct ExamineIfClosed {
    pub identifier: Option<i128>,   // A unique identifier for an instance of this structure
    pub tag_name: Option<String>,
    data: Option<bool>,
    powerflow: Option<PowerFlow>,
    // Add other fields as needed
}

impl ExamineIfClosed {
    pub fn new() -> ExamineIfClosed {
        ExamineIfClosed {
            identifier: None,  // Set default values or handle them as needed
            tag_name: None,
            data: None,
            powerflow: None,  // Initialize as None  // Set default values or handle them as needed
        }
    }

    pub fn identifier(mut self) -> Self {
        // Generate a random i128 identifier
        let mut rng = rand::thread_rng();
        let identifier = rng.gen::<i128>();
        self.identifier = Some(identifier);
        self
    }
    
    pub fn tag_name(mut self,tag_name: String ) -> Self {
        self.tag_name = Some(tag_name);
        self
    }

    pub fn data(mut self, data: bool) -> Self {
        self.data = Some(data);
        self
    }

    pub fn powerflow(mut self, powerflow: PowerFlow) -> Self {
        self.powerflow = Some(powerflow);
        self
    }

    pub fn build(self) -> ExamineIfClosed {
        ExamineIfClosed {
            identifier: self.identifier,
            tag_name: self.tag_name,
            data: self.data,
            powerflow: self.powerflow,
        }
    }

    pub fn printdata(self){
        print!("identifier: {:?} 
        tag_name {:?} 
        data {:?} 
        powerflow {:?}",
        self.identifier, 
        self.tag_name, 
        self.data, 
        self.powerflow);

    }
}


// Implement the Component trait for EndOfRung
impl Component for ExamineIfClosed {
    fn logic(&self) {
        // This logic function will be called when you iterate through components
        // Implement any shared logic for all components here if needed
    }
}