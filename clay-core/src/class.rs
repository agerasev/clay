/// An interface in OpenCL code.
pub trait Class {
    /// Class name (e.g. `shape`)
    fn name() -> String;
    /// List of methods of the class.
    fn methods() -> Vec<String>;
}

/// An implementation of a class in OpenCL.
pub trait Instance<C: Class> {
    // Class of an instance.
    //type Class: Class = C;
    
    /// Associated OpenCL code that contains necessary function definition.
    fn source() -> String;
    /// Name of the instance of the class (e.g. `sphere` as instance of class `shape`).
    fn inst_name() -> String;
}
