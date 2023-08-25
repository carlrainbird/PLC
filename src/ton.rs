// ton.rs

use serde::{Deserialize, Serialize};
use crate::rung_functions::LadderComponent;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TON {
    pub tag_name: String,
    pub en: bool,
    pub eno: bool,
    pub setpoint: i32,
    pub acc: i32,
    pub dn: bool,
    pub tt: bool,
}

impl LadderComponent for TON {
    fn push_it(&self) {
        println!("Processing TON: {:?}", self);
        self.execute_function();
    }
    fn execute_function(&self) {
        // Your TON logic here
    }
}
