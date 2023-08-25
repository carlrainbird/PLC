// xic.rs

use serde::{Deserialize, Serialize};
use crate::rung_functions::LadderComponent;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct XIC {
    pub tag_name: String,
    pub data: bool,
    pub en: bool,
    pub eno: bool,
}

impl LadderComponent for XIC {
    fn push_it(&self) {
        println!("Processing XIC: {:?}", self);
        self.execute_function();
    }
    fn execute_function(&self) {
        // Your XIC logic here
    }
}
