use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq)]
pub enum TmplError {
    // Variable
    NotFoundVariable{key: String},
    NotAArray{key: String},
    IndexOutOfRange{key: String, index: usize},
    // Function
    NotFoundFunction{function_name: String},
    // CustomFunctionReturnString{function_name: String},
    //Condition
    BadComparator{comparator: String},
}

impl Display for TmplError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let err_s = match self {
            // Variable
            TmplError::NotFoundVariable { key } => format!("[key '{key}' not exist]"),
            TmplError::NotAArray {key} => format!("[key '{key}' is not a Array]"),
            TmplError::IndexOutOfRange { key, index } => format!("[index '{key}[{index}]' out of range]"),
            // Function
            TmplError::NotFoundFunction {function_name} => format!("[Function : {function_name} not exist]"),
            // TmplError::CustomFunctionReturnString {function_name}=> format!("[Function : {function_name} must return a string]"),
            // Condition
            TmplError::BadComparator {comparator}=> format!("[{comparator} is not valid comparator]"),
        };
        return write!(f, "{err_s}")
    }
}