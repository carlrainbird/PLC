// main.rs


mod rung_functions;
mod xic;
mod ton;

use rung_functions::{read_rung_logic_from_yaml, process_rung_logic};

fn main() {
    // Read the YAML file using the interface function from rung_functions module
    let file_path = "RungLogic.yaml";
    let rung_logic = read_rung_logic_from_yaml(file_path)
        .expect(&format!("Failed to read or parse {}", file_path));

    // Process rung logic using the interface function
    let function_struct_vec = process_rung_logic(rung_logic);

    // Save function struct in the vector
    for struct_pushed in &function_struct_vec {
        struct_pushed();
    }
}
