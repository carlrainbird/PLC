use serde::Deserialize;
use serde_yaml;

#[derive(Debug, Clone, Deserialize)] 
pub struct RungStart{
    pub timedatestamp: i128, //  # a unique identifier
    pub alias: String,
    pub datatype: bool,
    pub data: bool,
    pub comment: String,
    pub file: String,
}

#[derive(Debug, Clone, Deserialize)] 
pub struct RungEnd{
    pub timedatestamp: i128, //  # a unique identifier
    pub alias: String,
    pub datatype: bool,
    pub data: bool,
    pub comment: String,
    pub file: String,
}

#[derive(Debug, Clone, Deserialize)] 
pub struct Boolean{
    pub timedatestamp: i128,
    pub alias: String,
    pub datatype: bool,
    pub data: bool,
    pub comment: String,
}
/*/
#[derive(Debug, Clone, Deserialize)] 
pub enum DataType{
    EndOfRung,
    StartOfRung,
    Boolean,

}*/