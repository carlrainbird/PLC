trait LadderComponent{
    fn evaluate(&self)->bool;
    //fn push it
}

#[derive(Debug, Serialize, Deserialize)]
struct ExIfClsd{
    tag: String,  //todo
    en: bool,
    eno: bool,
}

impl LadderComponent for ExIfClsd{
    // if self.tag == true...
    fn evaluate(&self)->bool{
        if self.en {
            // if self.tag == true... todo
            self.eno = true
        }
        else{
            self.eno = false  // the tag would be false at this point
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TmrOn{
    tag: String,
    en: bool,
    eno: bool,
    setpoint: int,
    acc: int,
    dn: bool,
    tt: bool,
}
// place all functions in seperate file  todo
impl LadderComponent for TON{
    fn evaluate(&self)->bool{
       if self.en { 
            if self.setpoint >= self.acc {
            self.dn = true;
            self.tt = false;
            }
            else{
            self.dn = false;
            self.tt = true;
            }
        
        self.eno = true   // the timer has been checked carry on
        }
        else{
            self.dn = false;
            self.tt = false;
            self.acc = 0;
            self.eno = false
        }
    }
}

struct Rung{
    components: Vec<Box<dyn LadderComponent>>,
}

impl LadderComponent for Rung{
    fn evaluate(&self)->bool{
        self
        .components
        .iter()
        .all(|components|components.evaluate())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
enum RungFunctions {
    XIC(ExIfClsd),
    TON(TmrOn),
}

// Define processing functions for each enum variant
fn process_xic(tag: String, en: bool, eno: bool) {
    let data = GenericData { tag, en, eno };
    println!("Processing XIC: {:?}", data);
}

fn main() {
    // Read the YAML file
    let mut file = File::open("PLC.yaml").expect("Failed to open the file.");
    let mut contents = String::new();
    file
    .read_to_string(&mut contents)
    .expect("Failed to read the file.");

    // Deserialize the YAML data into a vector of RungFunctions enum variants
    let rung_logic: Vec<RungFunctions> =
        serde_yaml::from_str(&contents)
        .expect("Failed to parse YAML.");

    // Process each enum variant and call the corresponding processing function
    for rung_function in &rung_logic {
        match rung_function {
            RungFunctions::XIC(data) => Rung{components: vec![Box::new(data.tag.clone(), data.en, data.eno)]},
            RungFunctions::TON(data) => process_xic(data.tag.clone(), data.en, data.eno),
        }
    }

}
