#[allow(warnings)]
pub mod t_type;
pub mod t_macro;

use std::collections::HashMap;

use chrono::{Utc, DateTime};
use t_type::{FuncMap, VariableMap, TVal};
use regex::Regex;

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

fn get_variable<'a>(key: &'a str, var_map: &'a VariableMap) -> (Option<&'a TVal>, bool) {
    let maybe_tval = var_map.get(key);
    if maybe_tval.is_some() {
        return (maybe_tval, true);
    }
    let maybe_tval: Option<&TVal> = if key.contains('.') && !key.contains(' ') {
        key.split('.').fold(None, |old_maybe_tval, current_key| {
            var_map
                .get(current_key)
                .or_else(|| old_maybe_tval.and_then(|t_val| t_val.get_hashmap(current_key)))
        })
    } else {
        None
    };
    return (maybe_tval, maybe_tval.is_some());
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

fn typing(reg: &Regex, parametres: String, var_map: &VariableMap, typing: Option<String>) -> Vec<TVal> {

    let mut vec_typing: Vec<TVal> = vec![];

    if typing.is_none() {
        for group_param in find_all_group(reg, &parametres) {

            println!("{:?}", group_param);


            if group_param.contains_key("str_double") && group_param["str_double"] != "" {
                vec_typing.push(TVal::Str(group_param["str_double"].to_string()));
                print!("double");

            } else if group_param.contains_key("str_single") && group_param["str_single"].to_string() != "" {
                vec_typing.push(TVal::Str(group_param["str_single"].to_string()));
                print!("single");


            } else if group_param.contains_key("str_back") && group_param["str_back"] != "" {
                vec_typing.push(TVal::Str(group_param["str_back"].to_string()));
                print!("back");


            } else if group_param.contains_key("bool") && group_param["bool"].to_string() != "" {

                let b: bool;

                match group_param["bool"].as_ref() {
                    "True" => b = true,
                    "False" => b = false,
                    _ => panic!("nique"),
                }

                vec_typing.push(TVal::Bool(b))

            } else if group_param.contains_key("number") && group_param["number"].to_string() != "" {

                let number = group_param["number"].to_string();

                if !number.contains(".") {
                    vec_typing.push(TVal::Int(number.parse::<i32>().unwrap()))
                } else {
                    vec_typing.push(TVal::Float(number.parse::<f64>().unwrap()))
                }

            } else if group_param.contains_key("variable") && group_param["variable"].to_string() != "" {
                let value_variable = get_variable(group_param["variable"].as_str(), var_map).0;

                if !value_variable.is_none() {
                    vec_typing.push(TVal::Str(value_variable.unwrap().get_to_string()))
                }

            }
        }

    } else if typing.as_ref().unwrap() == "int" {
        vec_typing.push(TVal::Int(parametres.parse::<i32>().unwrap()))
        
    } else if typing.as_ref().unwrap() == "float" {
        vec_typing.push(TVal::Float(parametres.parse::<f64>().unwrap()))

    } else if typing.as_ref().unwrap() == "str" {
        vec_typing.push(TVal::Str(parametres))
    
    } else if typing.as_ref().unwrap() == "bool" {
        vec_typing.push(TVal::Bool(parametres.parse::<bool>().unwrap()))
    
    }

    return vec_typing;
}

impl TemplateStr {

    pub fn new(variable_map: Option<VariableMap>, function_vec: Option<FuncMap>) -> TemplateStr {

        let mut vmap: VariableMap = VariableMap::new();
        let mut fvec: FuncMap = FuncMap::new();

        if !variable_map.is_none() {
            vmap = variable_map.unwrap();
        }

        if !function_vec.is_none() {
            fvec = function_vec.unwrap();
        }

        return TemplateStr {
            variable_map: vmap,
            function_map: fvec,
            reg_variable: Regex::new(r#"(?P<match>\{\{\$(?P<key>[^{{$}}]+)}})"#).unwrap(),
            reg_function: Regex::new(r#"(?P<match>\{\{@(?P<function>[^{@}\s]+) ?(?P<key>[^{@}]+)?}})"#).unwrap(),
            reg_condition: Regex::new(r#"(?P<match>\{\{#(?P<compValue1>[^{#}]+) (?P<compSymbol>[=!<>][=]?) (?P<compValue2>[^{#}]+): (?P<resultValue1>[^{}]+) || (?P<resultValue2>[^{}]+)}})"#).unwrap(),
            reg_switch: Regex::new(r#"(?P<match>\{\{?(?:(?P<key>[^{?}:]+)|(?P<keyTyped>[^{?}]+):(?P<type>str|int|float)); (?P<val>(?:[^{}]+)=(?:[^{}]+)), default=(?P<default>[^{}]+)}})"#).unwrap(),
            reg_typing: Regex::new(r#""(?P<str_double>[^"]+)"|'(?P<str_single>[^']+)'|`(?P<str_back>[^`]+)`|<b:(?P<bool>True|False)>|<n:(?P<number>[0-9_.]+)>|(?P<variable>[^<>' ]+)"#).unwrap(),
        }
    }

    pub fn parse(&mut self, mut text: String) -> String {

        text = self.parse_variable(text);
        text = self.parse_function(text);
        text = self.parse_condition(text);
        text = self.parse_switch(text);

        return text
    }

    pub fn parse_variable(&mut self, text: String) -> String {

        
        if !self.has_variable(text.to_string()) { return text.to_string() };
        let mut text_ed = text.to_string();

        for v in find_all_group(&self.reg_variable, &text) {
            let mut key= "None";
            if v.contains_key("key") { key = &v["key"]; };
            let match_text = &v["match"];

            let (value, mut _ok) = get_variable(key, &self.variable_map);

            let replace_val: String;

            if !value.is_none() {
                replace_val = value.unwrap().get_to_string();

            } else {
                replace_val = "None".to_string()
            }

            text_ed = text_ed.replace(match_text, replace_val.as_str());
            
        };

        return text_ed.to_string();
    }

    pub fn parse_function(&self, text: String) -> String{

        if !self.has_function(text.to_string()) { return text.to_string() }
        let mut text_ed = text.to_string();
        let mut replace_val: String = "None".to_string();

        for v in find_all_group(&self.reg_function, &text) {
            let mut key= "None";
            if v.contains_key("key") { key = &v["key"]; };
            let match_text = &v["match"];
            let function_name = v["function"].as_str();

            let (value, ok) = get_variable(key, &self.variable_map);

            if ok && !value.is_none() {
                replace_val = value.unwrap().get_to_string();
            }

            let now: DateTime<Utc> = Utc::now();

            match function_name {
                "uppercase" => { text_ed = text_ed.replace(match_text, replace_val.to_uppercase().as_str()) },
                "uppercaseFirst" => { 
                    let mut vec_letter: Vec<char> = replace_val.chars().collect();
                    vec_letter[0] = vec_letter[0].to_uppercase().nth(0).unwrap();
                    replace_val = vec_letter.into_iter().collect();
                    text_ed = text_ed.replace(match_text, replace_val.as_str())
                },
                "lowercase" => { text_ed = text_ed.replace(match_text, replace_val.to_lowercase().as_str()) },
                // "casefold" => { text_ed = text_ed.replace(match_text, replace_val.to_string().as_str()) },
                "swapcase" => { text_ed = text_ed.replace(match_text, swap_case(&replace_val).as_str()) },
                "time" => { text_ed = text_ed.replace(match_text, now.format("%H:%M:%S").to_string().as_str()) },
                "date" => { text_ed = text_ed.replace(match_text, now.format("%d/%m/%Y").to_string().as_str()) },
                "dateTime" => { text_ed = text_ed.replace(match_text, now.format("%d/%m/%Y %H:%M:%S").to_string().as_str()) },
                _ => {

                    if check_exist_fn(&self.function_map, function_name.to_string()) {

                        let mut result_text_fn: String = "None".to_string();
                        let custom_function = self.function_map[function_name];

                        if key != "" {
                            println!("{:?}", key);

                            result_text_fn = custom_function(typing(&self.reg_typing, key.to_string(), &self.variable_map, None));

                        } else {
                            result_text_fn = custom_function(vec![]);

                        }
                        text_ed = text_ed.replace(match_text, &result_text_fn);

                    } else {
                        text_ed = format!("NoFunction : {}", function_name);
                        
                    }
                },
            }
        };
        return text_ed
    }

    pub fn parse_condition(&self, mut text: String) -> String {
        text = text;


        return text
    }

    pub fn parse_switch(&self, mut text: String) -> String {
        text = text;


        return text
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