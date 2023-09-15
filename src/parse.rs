use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use serde_yaml;

use crate::composite_pattern::ComponentType;

pub fn read_components_from_yaml(file_path: &str) -> Result<Vec<ComponentType>, Box<dyn Error>> {
    let mut file = File::open(file_path)
        .map_err(|err| format!("Failed to open {}: {}", file_path, err))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|err| format!("Failed to read {}: {}", file_path, err))?;

    let component_instances: Vec<ComponentType> =
        serde_yaml::from_str(&contents)
        .map_err(|err| format!("Failed to parse YAML: {}", err))?;

    Ok(component_instances)
}
