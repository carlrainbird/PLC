use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct GenericData {
    tag: String,
    alias: String,
    datatype: String,
    location: String,
    scope: String,
    comment: String,
    en: bool,
    eno: bool,
}

#[derive(Debug, Serialize, Deserialize)]
enum RungFunctions {
    SOR (GenericData),
    XIC (GenericData),
    XIO (GenericData),
    SOOR (GenericData),
    Out (GenericData),
    EOOR (GenericData),
    OREVAL (GenericData),
    EOR (GenericData),
}

fn main() {
    // Read the YAML file
    let mut file = File::open("PLC.yaml")
                 .expect("Failed to open the file.");
    
    let mut contents = String::new();
    file
    .read_to_string(&mut contents).expect("Failed to read the file.");

    // Deserialize the YAML data into a vector of RungFunctions enum variants
    let rung_logic: Vec<RungFunctions> = serde_yaml::from_str(&contents).expect("Failed to parse YAML.");

    // Serialize the RungLogic enum variants back into YAML format
    let serialized_yaml = serde_yaml::to_string(&rung_logic)
                        .expect("Failed to serialize YAML.");

    // Write the serialized YAML back to a file named PLCwrite.yaml
    let mut output_file = File::create("PLCwrite.yaml")
    .expect("Failed to create output file.");
    
    output_file
        .write_all(serialized_yaml.as_bytes())
        .expect("Failed to write to output file.");

    // Append the enum variants to a single vector
    let mut rung_functions: Vec<RungFunctions> = Vec::new();
    rung_functions
    .extend(rung_logic);

    // Serialize the vector back into YAML format
    let rung_functions_yaml = serde_yaml::to_string(&rung_functions)
    .expect("Failed to serialize YAML.");

    // Write the serialized vector to a file named PLC.txt
    let mut output_file = File::create("PLC.txt")
           .expect("Failed to create output file.");
        
        output_file
        .write_all(rung_functions_yaml
        .as_bytes())
        .expect("Failed to write to output file.");
}
