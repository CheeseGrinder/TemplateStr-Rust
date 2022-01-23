use std::collections::HashMap;
use std::any::Any;
use std::vec::Vec;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

type VariableMap = HashMap<String, Box<dyn Any>>;
type Func = fn (list: Vec<Box<dyn Any>>) -> String;
type FuncList = Vec<Func>;


static ref reg_variable: Regex = Regex::new("(?P<match>{{$(?P<key>[^{{$}}]+)}})").unwrap();
static ref reg_function: Regex = Regex::new("(?P<match>{{@(?P<function>[^{@}\\s]+) ?(?P<key>[^{@}]+)?}})").unwrap();
static ref reg_condition: Regex = Regex::new("(?P<match>{{#(?P<compValue1>[^{#}]+) (?P<compSymbol>[=!<>][=]?) (?P<compValue2>[^{#}]+): (?P<resultValue1>[^{}]+) || (?P<resultValue2>[^{}]+)}})").unwrap();
static ref reg_switch: Regex = Regex::new("(?P<match>{{?(?:(?P<key>[^{?}:]+)|(?P<keyTyped>[^{?}]+):(?P<type>str|int|float)); (?P<val>(?:[^{}]+)=(?:[^{}]+)), default=(?P<default>[^{}]+)}})").unwrap();
static ref reg_typing: Regex = Regex::new("\"(?P<str_double>[^\"]+)\"|\'(?P<str_single>[^\']+)\'|\x60(?P<str_back>[^\x60]+)\x60|<b:(?P<bool>True|False)>|<n:(?P<number>[0-9_.]+)>|(?P<variable>[^<>\' ]+)").unwrap();


pub struct TemplateStr {
    variable_map: VariableMap,
    function_list: FuncList,

}


impl TemplateStr {


    pub fn new(&self, var_map: VariableMap, func_list: FuncList) -> TemplateStr {


        return TemplateStr {
            variable_map: var_map,
            function_list: func_list,
        }
    }

    pub fn parse(&self, mut text: String) -> String {

        text = self.parse_variable(text);
        text = self.parse_function(text);
        text = self.parse_condition(text);
        text = self.parse_switch(text);

        return text
    }

    pub fn parse_variable(&self, mut text: String) -> String {
        
        return text
    }

    pub fn parse_function(&self, mut text: String) -> String {

        return text
    }

    pub fn parse_condition(&self, mut text: String) -> String {

        return text
    }

    pub fn parse_switch(&self, mut text: String) -> String {

        return text
    }

    pub fn has_variable(&self, mut text: String) -> bool {

        return true
    }
}