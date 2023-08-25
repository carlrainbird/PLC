// rung_functions.rs

use serde::{Serialize, Deserialize};
use crate::xic::XIC;
use crate::ton::TON;

#[derive(Debug, Serialize, Deserialize)]
pub enum RungFunctions {
    XIC(XIC),
    TON(TON),
}

pub trait LadderComponent {
    fn push_it(&self);
    fn execute_function(&self);
}

pub fn read_rung_logic_from_yaml(file_path: &str) -> Result<Vec<RungFunctions>, Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open(file_path)
        .map_err(|err| format!("Failed to open {}: {}", file_path, err))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|err| format!("Failed to read {}: {}", file_path, err))?;

    let rung_logic: Vec<RungFunctions> =
        serde_yaml::from_str(&contents)
        .map_err(|err| format!("Failed to parse YAML: {}", err))?;

    Ok(rung_logic)
}

pub fn process_rung_logic(rung_logic: Vec<RungFunctions>) -> Vec<Box<dyn Fn()>> {
    let mut function_struct_vec: Vec<Box<dyn Fn()>> = Vec::new();

    // Process each enum variant and push the corresponding closure
    for rung_function in rung_logic {
        match rung_function {
            RungFunctions::XIC(data) => {
                let struct_pushed = move || {
                    data.push_it();
                };
                function_struct_vec.push(Box::new(struct_pushed));
            }
            RungFunctions::TON(data) => {
                let struct_pushed = move || {
                    data.push_it();
                };
                function_struct_vec.push(Box::new(struct_pushed));
            }
        }
    }

    function_struct_vec
}
