//handle the memory for the tag
enum DataType{
    datatype(bool),
    datatype(int),
    datatype(String),
    datatype(float),
}

struct Tag {
    tag: String,
    alias: String,
    DataType: datatype,
    location: String,
    scope: String,
    comment: String,
}



//Beauty0767737115