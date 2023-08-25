// the question is should this be stored in a database or in a seperate 
// file.
struct XIO {
    alias: String,      // sometimes double description is needed
    datatype: String,   // in this case its a bool
    location: String,   // the location of the bool memory
    scope: String,      // does the memory have scope
    comment: String,    // ok 
    tag: String,        // this is a key identifier
    en: bool,
    eno: bool,
}

struct SOR {
    alias: String,     // s
    datatype: String,  // in this case its a bool
    location: String,  // the location of the rung
    scope: String,     // what file location
    comment: String,   // this is the rung comment
    en: bool,          // start the power flow off
    eno: bool,         //
}



