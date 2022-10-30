#[allow(warnings)]
pub mod t_type;
pub mod t_macro;
pub mod reg;
pub mod error;

use std::collections::HashMap;

use chrono::{Utc, DateTime};
use error::TmplError;
use t_type::{FuncMap, VariableMap, TVal};
use regex::Regex;
use reg::{REG_VARIABLE, REG_FUNCTION, REG_CONDITION, REG_SWITCH, REG_BOOL, REG_STR, REG_INT, REG_FLOAT, REG_VAR, REG_LIST};

#[derive(Clone, Debug)]
pub struct TemplateStr {
    variable_map: VariableMap,
    function_map: FuncMap,
    reg_variable: Regex,
    reg_function: Regex,
    reg_condition: Regex,
    reg_switch: Regex,
    reg_typing: Regex,
}

fn get_variable<'a>(key: &'a str, var_map: &'a VariableMap, index: Option<usize>) -> Result<Option<&'a TVal>, TmplError> {
    let mut maybe_tval: Option<&TVal> = var_map.get(key);

    if key.contains('.') && !key.contains(' ') {

        maybe_tval = key.split('.').fold(None, |old_maybe_tval, current_key| {
            var_map.get(current_key).or_else(|| old_maybe_tval.and_then(|t_val| t_val.get_hashmap(current_key)))
        });
    };

    if index.is_some() && maybe_tval.unwrap().get_type() == "Vec" {

        if maybe_tval.unwrap().get_vec_len() <= index.unwrap() {
            return  Err(TmplError::IndexOutOfRange { key: (key.to_string()), index: (index.unwrap()) });
        }

        maybe_tval = maybe_tval.unwrap().get_vec_item(index.unwrap());

    } else if index.is_some() {
        return Err(TmplError::NotAArray { key: (key.to_string()) });
    }

    if maybe_tval.is_none() {
        return Err(TmplError::NotFoundVariable { key: (key.to_string()) });
    };

    return Ok(maybe_tval)


}

fn find_all_group(reg: &Regex, str: &String) -> Vec<HashMap<String, String>> {

    return reg.captures_iter(str).map(|caps| {
        reg.capture_names().map(|o| o.and_then(|n| Some((n.to_string(), caps.name(n)?.as_str().to_string())))).flatten().collect()
    }).collect();
}

fn check_exist_fn(function_map: &FuncMap, name_function: String) -> bool {

    for func in function_map {

        // println!("{}", get_name_fn(*func));

        if func.0.to_string() == name_function {
            return true ;
        }
    }

    return false;
}

fn swap_case(text: &String) -> String {

    let vec_letter: Vec<char> = text.chars().collect();
    let mut new_vec_tetter: Vec<String> = vec!["a".to_string(); vec_letter.len()];

    for (index, l) in vec_letter.iter().enumerate() {
        if l.is_alphabetic() && l.is_uppercase() {
            new_vec_tetter[index] = l.to_lowercase().to_string();
        } else if l.is_alphabetic() && l.is_lowercase() {
            new_vec_tetter[index] = l.to_uppercase().to_string();
        } else {
            new_vec_tetter[index] = l.to_string();
        }
    }

    return new_vec_tetter.join("").to_string();

}

fn convert_tval_to_float(value1: &TVal, value2: &TVal) -> (f64, f64) {

    let value1_f: f64;
    let value2_f: f64;

    match value1 {
        TVal::Int(i) => {value1_f = *i as f64}
        TVal::Str(i) => {value1_f = i.len() as f64},
        TVal::Float(i) => {value1_f = *i},
        TVal::Bool(i) => {value1_f = *i as i32 as f64},
        _ => todo!(),
    }

    match value2 {
        TVal::Int(i) => {value2_f = *i as f64}
        TVal::Str(i) => {value2_f = i.len() as f64},
        TVal::Float(i) => {value2_f = *i},
        TVal::Bool(i) => {value2_f = *i as i32 as f64},
        _ => todo!(),
    }

    return (value1_f, value2_f);

}

fn typing<'a, 'b>(reg: &'a Regex, parametres: String, var_map: &'b VariableMap, type_: Option<String>) -> Result<Vec<TVal>, TmplError> {

    let mut vec_typing: Vec<TVal> = vec![];
        
    if type_.is_none() {
        for group_param in find_all_group(reg, &parametres) {

            println!("{:?}", group_param);

            if group_param.contains_key("str_double") && group_param["str_double"] != "" {
                
                vec_typing.push(TVal::Str(group_param["str_double"].to_string()));

            } else if group_param.contains_key("str_single") && group_param["str_single"].to_string() != "" {
                
                vec_typing.push(TVal::Str(group_param["str_single"].to_string()));

            } else if group_param.contains_key("str_back") && group_param["str_back"] != "" {
                
                vec_typing.push(TVal::Str(group_param["str_back"].to_string()));

            } else if group_param.contains_key("bool") && group_param["bool"].to_string() != "" {
                
                let bool: bool;

                match group_param["bool"].to_lowercase().as_ref() {
                    "true" => bool = true,
                    "false" => bool = false,
                    _ => panic!("Typing ?"),
                }

                vec_typing.push(TVal::Bool(bool))

            } else if group_param.contains_key("int") && group_param["int"].to_string() != "" {

                let int = group_param["int"].to_string();
                vec_typing.push(TVal::Int(int.parse::<i32>().unwrap()));

            } else if group_param.contains_key("float") && group_param["float"].to_string() != "" {

                let float = group_param["float"].to_string();
                vec_typing.push(TVal::Float(float.parse::<f64>().unwrap()));

            } else if group_param.contains_key("variable") && group_param["variable"].to_string() != "" {

                let value_variable: Option<&TVal>;

                if group_param.contains_key("index") && group_param["index"].to_string() != "" {

                    value_variable = get_variable(group_param["variable"].as_str(), var_map, Some(group_param["index"].to_string().parse::<usize>().unwrap()))?;

                } else {
                    value_variable = get_variable(group_param["variable"].as_str(), var_map, None)?;
                }
                
                if !value_variable.is_none() {
                    vec_typing.push(value_variable.unwrap().clone());
                }
            } else if group_param.contains_key("list") && group_param["list"].to_string() != "" {
                vec_typing.push(TVal::Vec(typing(reg, group_param["list"].to_string(), var_map, None)?))
            }
        }
    } else if type_.as_ref().unwrap() == "int" {
        vec_typing.push(TVal::Int(parametres.parse::<i32>().unwrap()))
    } else if type_.as_ref().unwrap() == "float" {
        vec_typing.push(TVal::Float(parametres.parse::<f64>().unwrap()))
    } else if type_.as_ref().unwrap() == "str" {
        vec_typing.push(TVal::Str(parametres))
    } else if type_.as_ref().unwrap() == "bool" {
        vec_typing.push(TVal::Bool(parametres.parse::<bool>().unwrap()))
    }
    
    

    return Ok(vec_typing);
}

impl TemplateStr {

    pub fn new<'a>(variable_map: VariableMap, function_vec: Option<FuncMap>) -> TemplateStr {

        let vmap = variable_map;
        let fvec = function_vec.unwrap_or(FuncMap::new());

        return TemplateStr {
            variable_map: vmap,
            function_map: fvec,
            reg_variable: Regex::new(REG_VARIABLE).unwrap(),
            reg_function: Regex::new(REG_FUNCTION).unwrap(),
            reg_condition: Regex::new(REG_CONDITION).unwrap(),
            reg_switch: Regex::new(REG_SWITCH).unwrap(),
            reg_typing: Regex::new(format!("{}|{}|{}|{}|{}|{}", REG_STR, REG_BOOL, REG_INT, REG_FLOAT, REG_VAR, REG_LIST).as_str()).unwrap(),
        }
    }

    pub fn parse(&self, text: String) -> Result<String, TmplError> {

        let mut text_ed = text.to_string();

        while self.has_one(text_ed.to_string()) {

            text_ed = self.parse_variable(text_ed)?;
            text_ed = self.parse_function(text_ed)?;
            text_ed = self.parse_condition(text_ed)?;
            text_ed = self.parse_switch(text_ed)?;
        }

        return Ok(text_ed)
    }

    pub fn parse_variable(&self, text: String) -> Result<String, TmplError> {

        if !self.has_variable(text.to_string()) { return Ok(text.to_string()) };
        let mut text_ed = text.to_string();

        while self.has_variable(text_ed.to_string()) {
            
            for v in find_all_group(&self.reg_variable, &text_ed) {
                let mut index: Option<usize> = None;
                let mut key= "None";

                if v.contains_key("variable") { key = &v["variable"]; };
                if v.contains_key("index") { index = Some(v["index"].to_string().parse::<usize>().unwrap()); };

                let match_text = &v["match"];
    
                let value = get_variable(key, &self.variable_map, index)?;
    
                let replace_val: String;
    
                if !value.is_none() {
                    replace_val = value.unwrap().get_to_string();
    
                } else {
                    replace_val = "None".to_string()
                }
    
                text_ed = text_ed.replace(match_text, replace_val.as_str());
                
            }
        }
        return Ok(text_ed.to_string());
    }

    pub fn parse_function(&self, text: String) -> Result<String, TmplError>{

        if !self.has_function(text.to_string()) { return Ok(text.to_string()) }
        let mut text_ed = text.to_string();

        while self.has_function(text_ed.to_string()) {
                
            for v in find_all_group(&self.reg_function, &text_ed) {
                let mut parameters= "None";

                if v.contains_key("parameters") { parameters = &v["parameters"]; };

                let match_text = &v["match"];
                let function_name = v["functionName"].as_str();

                let value = || -> Result<Option<&TVal>, TmplError> {
                    return get_variable(parameters, &self.variable_map, None);
                };
    
                let now: DateTime<Utc> = Utc::now();
    
                match function_name {
                    "uppercase" => { text_ed = text_ed.replace(match_text, value()?.unwrap().get_to_string().to_uppercase().as_str()) },
                    "uppercaseFirst" => { 
                        let mut vec_letter: Vec<char> = value()?.unwrap().get_to_string().chars().collect();

                        vec_letter[0] = vec_letter[0].to_uppercase().nth(0).unwrap();
                        let r: String = vec_letter.into_iter().collect();

                        text_ed = text_ed.replace(match_text, r.as_str())
                    },
                    "lowercase" => { text_ed = text_ed.replace(match_text, value()?.unwrap().get_to_string().to_lowercase().as_str()) },
                    // "casefold" => { text_ed = text_ed.replace(match_text, replace_val.to_string().as_str()) },
                    "swapcase" => { text_ed = text_ed.replace(match_text, swap_case(&value()?.unwrap().get_to_string()).as_str()) },
                    "time" => { text_ed = text_ed.replace(match_text, now.format("%H:%M:%S").to_string().as_str()) },
                    "date" => { text_ed = text_ed.replace(match_text, now.format("%d/%m/%Y").to_string().as_str()) },
                    "dateTime" => { text_ed = text_ed.replace(match_text, now.format("%d/%m/%Y %H:%M:%S").to_string().as_str()) },
                    _ => {
    
                        if check_exist_fn(&self.function_map, function_name.to_string()) {
    
                            let result_text_fn: String;
                            let custom_function = self.function_map[function_name];
    
                            if parameters != "None" {
    
                                result_text_fn = custom_function(typing(&self.reg_typing, parameters.to_string(), &self.variable_map, None)?);
    
                            } else {
                                result_text_fn = custom_function(vec![]);
    
                            }
                            text_ed = text_ed.replace(match_text, &result_text_fn);
    
                        } else {
                            return Err(TmplError::NotFoundFunction { function_name: (function_name.to_string()) });
                        }
                    },
                }
            }
        }
        return Ok(text_ed)
    }

    pub fn parse_condition(&self, text: String) -> Result<String, TmplError> {

        if !self.has_condition(text.to_string()) { return Ok(text.to_string()) };
        let mut text_ed = text.to_string();

        while self.has_condition(text_ed.to_string()) {
            
            for v in find_all_group(&self.reg_condition, &text_ed) {
                
                let match_text = &v["match"];
                let condition_value1 = &v["conditionValue1"];
                let condition_value2 = &v["conditionValue2"];
                let condition_symbol = &v["conditionSymbol"];
                let true_value = &v["trueValue"];
                let false_value = &v["falseValue"];

                let vecteur_typing = typing(&self.reg_typing, format!("{} {}", condition_value1.to_string(), condition_value2.to_string()), &self.variable_map, None)?;
                
                let v = || convert_tval_to_float(&vecteur_typing[0], &vecteur_typing[1]);
                
                if condition_symbol == "==" {
                    text_ed = text_ed.replace(match_text, ternary!(vecteur_typing[0] == vecteur_typing[1] => true_value; false_value))
                } else if condition_symbol == "!="{
                    text_ed = text_ed.replace(match_text, ternary!(vecteur_typing[0] != vecteur_typing[1] => true_value; false_value))
                } else if condition_symbol == "<="{
                    text_ed = text_ed.replace(match_text, ternary!(v().0 <= v().1 => true_value; false_value))
                } else if condition_symbol == ">="{
                    text_ed = text_ed.replace(match_text, ternary!(v().0 >= v().1 => true_value; false_value))
                } else if condition_symbol == "<"{
                    text_ed = text_ed.replace(match_text, ternary!(v().0 < v().1 => true_value; false_value))
                } else if condition_symbol == ">"{
                    text_ed = text_ed.replace(match_text, ternary!(v().0 > v().1 => true_value; false_value))
                } else {
                    return Err(TmplError::BadComparator { comparator: (condition_symbol.to_string()) });
                }
            }
        }

        return Ok(text_ed)
    }

    pub fn parse_switch(&self, text: String) -> Result<String, TmplError> {
        if !self.has_switch(text.to_string()) { return Ok(text.to_string()) };
        let mut text_ed = text.to_string();

        while self.has_switch(text_ed.to_string()) {
            
            for v in find_all_group(&self.reg_switch, &text_ed) {
    
                let match_text = &v["match"];
                // let mut key= "None";
    
    
                let mut map_temp: HashMap<String, String> = HashMap::new();
                let mut result: String = "".to_string();
    
                for n in v["values"].split(", ") {
                    let key_value: Vec<&str> = n.split("::").collect();
                    map_temp.insert(key_value[0].to_string(), key_value[1].to_string());
                }

                if !v.contains_key("type") {
                    for (key, value) in map_temp {
                        if key == self.variable_map[&v["variable"]].get_to_string() {
                            result = value;
                            break;
                        } else {
                            result = v["defaultValue"].to_string()
                        }
                    }
                } else {
                    let key_var = &v["variable"];
                    let type_var = &v["type"];
    
                    for (key, value) in map_temp {
                        let val_var = get_variable(&key_var, &self.variable_map, None)?.unwrap();
    
                        if val_var == &typing(&self.reg_typing, key, &self.variable_map, Some(type_var.to_string()))?[0] {
                            result = value;
                            break;
                        } else {
                            result = v["defaultValue"].to_string()
                        }
                    }
                }
    
                text_ed = text_ed.replace(match_text, &result);
            }
        }
        return Ok(text_ed)
    }

    pub fn has_one(&self, text: String) -> bool {

        let t = text.as_str();

        if self.has_variable(t.to_string()) || self.has_function(t.to_string()) || self.has_condition(t.to_string()) || self.has_switch(t.to_string()) {
            return true;
        }
        return false;

    }

    pub fn has_variable(&self, text: String) -> bool {

        return self.reg_variable.is_match(&text)
    }
    
    pub fn has_function(&self, text: String) -> bool {

        return self.reg_function.is_match(&text);
    }

    pub fn has_condition(&self, text: String) -> bool {

        return self.reg_condition.is_match(&text);
    }

    pub fn has_switch(&self, text: String) -> bool {

        return self.reg_switch.is_match(&text);
    }
}