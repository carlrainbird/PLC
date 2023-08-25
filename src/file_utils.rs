// file_utils.rs

use serde_yaml;
use std::fs::File;
use std::io::prelude::*;

use crate::rung_functions::RungFunctions;

pub fn read_rung_logic_from_yaml(file_path: &str) -> Result<Vec<RungFunctions>, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let rung_logic: Vec<RungFunctions> = serde_yaml::from_str(&contents)?;

    Ok(rung_logic)
}
