use std::any::Any;
use std::vec::Vec;
use std::collections::HashMap;

pub type VariableMap = HashMap<String, TVal>;

// TODO Type à revoire
pub type Func = fn (list: Vec<TVal>) -> String;
pub type FuncMap = HashMap<String, Func>;

#[derive(Clone, Debug, PartialEq)]
pub enum TVal {
    Str(String),
    Int(i32),
    Float(f64),
    Bool(bool),
    Hashmap(VariableMap),
    // VarMap(VariableMap),
}

impl TVal {
    pub fn get_hashmap(&self, key: &str) -> Option<&TVal> {
        match self {
            TVal::Hashmap(map) => map.get(key),
            _ => None,
        }
    }

    pub fn get_to_string(&self) -> String {
        match self {
            TVal::Str(s) => s.to_string(),
            TVal::Int(i) => i.to_string(),
            TVal::Float(f) => f.to_string(),
            TVal::Bool(b) => b.to_string(),
            TVal::Hashmap(_) => panic!("This key is Hashmap"),
            _ => panic!("Not a Str"),
        }
    }
}