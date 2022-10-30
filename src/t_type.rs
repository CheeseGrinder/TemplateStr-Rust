use std::any::{Any, TypeId};
use std::vec::{Vec, self};
use std::collections::HashMap;

pub type VariableMap = HashMap<String, TVal>;

// TODO Type à revoire
pub type TmplFunc = fn (array: Vec<TVal>) -> String;
pub type FuncMap = HashMap<String, TmplFunc>;

#[derive(Clone, Debug, PartialEq)]
pub enum TVal {
    Str(String),
    Int(i32),
    Float(f64),
    Bool(bool),
    Vec(Vec<TVal>),
    Hashmap(VariableMap),
    // VarMap(VariableMap),
}

impl TVal{
    pub fn get_hashmap(&self, key: &str) -> Option<&TVal> {
        match self {
            TVal::Hashmap(map) => return map.get(key),
            _ => return None,
        }
    }

    pub fn get_vec_item(&self, index: usize) -> Option<&TVal> {
        match self {
            TVal::Vec(vec) => return Some(&vec[index]),
            _ => return None,
        }
    }

    pub fn get_vec_len(&self) -> usize {
        match self {
            TVal::Vec(vec) => return vec.len(),
            _ => panic!("Not a Vec"),
        }
    }

    /// RETURN TYPE :
    /// - Str
    /// - Int
    /// - Float
    /// - Bool
    /// - Vec
    /// - Hasmap
    pub fn get_type(&self) -> &str {
        match self {
            TVal::Str(_) => return "Str",
            TVal::Int(_) => return "Int",
            TVal::Float(_) => return "Float",
            TVal::Bool(_) => return "Bool",
            TVal::Vec(_) => return "Vec",
            TVal::Hashmap(_) => return "Hasmap",
            _ => return "",
        }
    }

    pub fn get_to_string(&self) -> String {
        match self {
            TVal::Str(s) => return s.to_string(),
            TVal::Int(i) => return i.to_string(),
            TVal::Float(f) => return f.to_string(),
            TVal::Bool(b) => return b.to_string(),
            TVal::Vec(_) => panic!("This key is Vec"),
            TVal::Hashmap(_) => panic!("This key is Hashmap"),
            _ => panic!("Not a Str"),
        }
    }
}