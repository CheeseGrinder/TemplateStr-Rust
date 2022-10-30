use template_str::{TemplateStr, error::TmplError, t_type::{VariableMap as VMap, TVal, FuncMap}, varmap, list_func, vecTval};
use chrono::Utc;

struct GlobalVar{
    var_map: VMap,
    list_func: FuncMap,
}

fn test(_: Vec<TVal>) -> String {
    return "Test custom Function".to_string();
}

fn test_type(array: Vec<TVal>) -> String {

    let mut result: String = "ok".to_string();

    let a = &array[0];
    let b = &array[1];
    let c = &array[2];
    let d = &array[3];
    let e = &array[4];
    let f = &array[5];
    let g = &array[6];
    let h = &array[7];
    let i = &array[8];
    let j = &array[9];

    // println!("a: {:?}", a);
    // println!("b: {:?}", b);
    // println!("c: {:?}", c);
    // println!("d: {:?}", d);
    // println!("e: {:?}", e);
    // println!("f: {:?}", f);
    // println!("g: {:?}", g);
    // println!("h: {:?}", h);
    // println!("i: {:?}", i);
    // println!("j: {:?}", j);

    // A
    if a.get_type() != "Str" {
        result = format!("{}|{}", result, "type a != Str")
    } else {
        if a.clone() != TVal::Str("text".to_string()) {
            result = format!("{}|{}", result, "a != 'text'")
        }
    }

    // B
    if b.get_type() != "Str" {
        result = format!("{}|{}", result, "type b != Str")
    } else {
        if b.clone() != TVal::Str("text".to_string()) {
            result = format!("{}|{}", result, "b != 'text'")
        }
    }

    // C
    if c.get_type() != "Str" {
        result = format!("{}|{}", result, "type c != Str")
    } else {
        if c.clone() != TVal::Str("text".to_string()) {
            result = format!("{}|{}", result, "c != 'text'")
        }
    }

    // D
    if d.get_type() != "Bool" {
        result = format!("{}|{}", result, "type d != Bool")
    } else {
        if d.clone() != TVal::Bool(true) {
            result = format!("{}|{}", result, "d != true")
        }
    }

    // E
    if e.get_type() != "Int" {
        result = format!("{}|{}", result, "type e != Int")
    } else {
        if e.clone() != TVal::Int(123) {
            result = format!("{}|{}", result, "e != 123")
        }
    }

    // F
    if f.get_type() != "Float" {
        result = format!("{}|{}", result, "type f != Float")
    } else {
        if f.clone() != TVal::Float(123.4) {
            result = format!("{}|{}", result, "f != 123.4")
        }
    }

    // G
    if g.get_type() != "Int" {
        result = format!("{}|{}", result, "type g != Int")
    } else {
        if g.clone() != TVal::Int(32) {
            result = format!("{}|{}", result, "g != 32")
        }
    }

    // H
    if h.get_type() != "Int" {
        result = format!("{}|{}", result, "type h != Int")
    } else {
        if h.clone() != TVal::Int(42) {
            result = format!("{}|{}", result, "h != 42")
        }
    }

    // I
    if i.get_type() != "Vec" {
        result = format!("{}|{}", result, "type i != Vec")
    } else {
        if i.get_vec_item(1).unwrap().clone() != TVal::Int(56) {
            result = format!("{}|{}", result, "i != 56")
        }
    }

    // J
    if j.get_type() != "Str" {
        result = format!("{}|{}", result, "type j != Str")
    } else {
        if j.clone() != TVal::Str("Map in Map".to_string()) {
            result = format!("{}|{}", result, "j != 56")
        }
    }

    return result;
}

impl GlobalVar {
    fn new() -> GlobalVar {
        return GlobalVar {
            list_func: list_func![test, test_type],
            var_map: varmap!{
                "Build" => "Succes",
                "var" => "int",
                "var2" => "str",
                "str" => "Jame",
                "int" => 32,
                "float" => 4.2,
                "bool" => true,
                "lower" => "azerty",
                "upper" => "AZERTY",
                "swap" => "AzErTy",
                // "cfold" => "grüßen",
                "Map" => varmap!{
                    "value" => "Map in Map",
                },
                "MasterMap" => varmap!{
                    "SecondMap" => varmap!{
                        "value" => "Map in Map in Map",
                    },
                },
                "Vec" => vecTval![
                    "test", 
                    42
                ],
            },
        }
    }
}

#[cfg(test)]
mod all {
    use super::*;

    #[test]
    fn test_all() {
        let var = GlobalVar::new();

        let test_all_1 = vec!["Name is @{uppercase; str}, ${int} years old. Map: ${Map.value}. my keyboard; #{lower == 'azerty'; azerty | qwerty}, ?{lower; azerty::yes, AZERTY::no, _::anyway}",
        "Name is JAME, 32 years old. Map: Map in Map. my keyboard; azerty, yes"];
        let test_all_2 = vec!["test var in var ${${var}}", "test var in var 32"];
        let test_all_3 = vec!["test if in if #{lower == 'azerty2'; azerty | #{lower == 'querty'; yes | no}}","test if in if no"];
        let test_all_4 = vec!["test switch in switch ?{str; Jame::?{Build; Succes::#0, Failed::#1, _::#default}, Tony::#1, Marco::#2, _::#default}", "test switch in switch #0"];
        let test_all_5 = vec!["test wtf ?{str; Jame::?{int/${var}; 32::#0, 36::#1, _::#default}, Tony::#1, Marco::#2, _::#default2}", "test wtf #0"];

        let parser = TemplateStr::new(var.var_map, Some(var.list_func));

        assert_eq!(parser.parse(test_all_1[0].to_string()).unwrap(), test_all_1[1], "test_all_1");
        assert_eq!(parser.parse(test_all_2[0].to_string()).unwrap(), test_all_2[1], "test_all_2");
        assert_eq!(parser.parse(test_all_3[0].to_string()).unwrap(), test_all_3[1], "test_all_3");
        assert_eq!(parser.parse(test_all_4[0].to_string()).unwrap(), test_all_4[1], "test_all_4");
        assert_eq!(parser.parse(test_all_5[0].to_string()).unwrap(), test_all_5[1], "test_all_5");
    }

    #[test]
    fn test_all_error() {
        let var = GlobalVar::new();

        let parser = TemplateStr::new(var.var_map, None);
        assert_eq!(parser.parse("test func in func @{lowercase; @{uppercase; str}}".to_string()), Err(TmplError::NotFoundVariable { key: ("JAME".to_string()) }));

    }
}

#[cfg(test)]
mod variable {
    use super::*;

    #[test]
    fn test_variable() {

        let var = GlobalVar::new();

        let text_1 = vec!["var bool = ${bool} and name = ${str}", "var bool = true and name = Jame"];
        let text_2 = vec!["${Map.value}", "Map in Map"];
        let text_3 = vec!["${MasterMap.SecondMap.value}", "Map in Map in Map"];
        let text_6 = vec!["${${var2}}", "Jame"];

        let parser = TemplateStr::new(var.var_map, None);
        
        assert_eq!(parser.parse_variable(text_1[0].to_string()).unwrap(), text_1[1], "text_1");
        assert_eq!(parser.parse_variable(text_2[0].to_string()).unwrap(), text_2[1], "text_2");
        assert_eq!(parser.parse_variable(text_3[0].to_string()).unwrap(), text_3[1], "text_3");
        assert_eq!(parser.parse_variable(text_6[0].to_string()).unwrap(), text_6[1], "text_6");
    }

    #[test]
    fn test_variable_error() {
        let var = GlobalVar::new();

        let parser = TemplateStr::new(var.var_map, None);
        assert_eq!(parser.parse_variable("${word}".to_string()), Err(TmplError::NotFoundVariable { key: ("word".to_string()) }));
        assert_eq!(parser.parse_variable("${Map.SecondMap.value}".to_string()), Err(TmplError::NotFoundVariable { key: ("Map.SecondMap.value".to_string()) }));
        assert_eq!(parser.parse_variable("${Vec[5]}".to_string()), Err(TmplError::IndexOutOfRange { key: ("Vec".to_string()), index: (5) }));
        assert_eq!(parser.parse_variable("${int[5]}".to_string()), Err(TmplError::NotAArray { key: ("int".to_string()) }));
    }
}

#[cfg(test)]
mod function {

    use super::*;

    #[test]
    fn test_intern_function() {
        let var = GlobalVar::new();

        let uppercase = vec!["@{uppercase; lower}", "AZERTY"];
        let uppercase_first = vec!["@{uppercaseFirst; lower}", "Azerty"];
        let lowercase = vec!["@{lowercase; upper}", "azerty"];
        // let casefold = vec!["@{casefold cfold}", "grüssen"];
        let swapcase = vec!["@{swapcase; swap}", "aZeRtY"];
        let time = "@{time}";
        let date = "@{date}";
        let date_time = "@{dateTime}";
        let function_in_function = vec!["@{uppercase; @{lowercase; var2}}", "JAME"];

        let parser = TemplateStr::new(var.var_map, None);

        assert_eq!(parser.parse_function(uppercase[0].to_string()).unwrap(), uppercase[1], "uppercase");
        assert_eq!(parser.parse_function(uppercase_first[0].to_string()).unwrap(), uppercase_first[1], "uppercase_first");
        assert_eq!(parser.parse_function(lowercase[0].to_string()).unwrap(), lowercase[1], "lowercase");
        // assert_eq!(parser.parse_function(casefold[0].to_string()).unwrap(), casefold[1], "");
        assert_eq!(parser.parse_function(swapcase[0].to_string()).unwrap(), swapcase[1], "swapcase");
        assert_eq!(parser.parse_function(time.to_string()).unwrap(), Utc::now().format("%H:%M:%S").to_string().as_str(), "time");
        assert_eq!(parser.parse_function(date.to_string()).unwrap(), Utc::now().format("%d/%m/%Y").to_string().as_str(), "date");
        assert_eq!(parser.parse_function(date_time.to_string()).unwrap(), Utc::now().format("%d/%m/%Y %H:%M:%S").to_string().as_str(), "date_time");
        assert_eq!(parser.parse_function(function_in_function[0].to_string()).unwrap(), function_in_function[1], "function_in_function");

    }

    #[test]
    fn test_intern_function_error() {
        let var = GlobalVar::new();

        let parser = TemplateStr::new(var.var_map, None);

        assert_eq!(parser.parse_function("@{uppercase; word}".to_string()), Err(TmplError::NotFoundVariable { key: ("word".to_string()) }));
    }

    #[test]
    fn test_custom_function() {
        let var = GlobalVar::new();

        let test_1 = vec!["@{test}", "Test custom Function"];
        let test_2 = vec![r#"@{test_type; "text" 'text' `text` b/True i/123 f/123.4 int Vec[1] ('test', i/56) Map.value}"#, "ok"];

        let parser = TemplateStr::new(var.var_map, Some(var.list_func));

        assert_eq!(parser.parse_function(test_1[0].to_string()).unwrap(), test_1[1], "");
        assert_eq!(parser.parse_function(test_2[0].to_string()).unwrap(), test_2[1], "");

    }

    #[test]
    fn test_custom_function_error() {
        let var = GlobalVar::new();

        let parser = TemplateStr::new(var.var_map, None);

        assert_eq!(parser.parse_function("@{test26}".to_string()), Err(TmplError::NotFoundFunction { function_name: ("test26".to_string()) }));
    }
}

#[cfg(test)]
mod condition {

    use super::*;

    #[test]
    fn test_condition_in_condition() {
        let var = GlobalVar::new();

        let parser = TemplateStr::new(var.var_map, None);

        let condition_in_condition = vec!["#{'text' == 'text'; yes | #{'text' != 'text'; yes | no}}", "yes"];

        assert_eq!(parser.parse_condition(condition_in_condition[0].to_string()).unwrap(), condition_in_condition[1], "condition_in_condition");
    }

    #[test]
    fn test_condition_equal() {
        let var = GlobalVar::new();

        let str_equal_str = vec!["#{'text' == 'text'; yes | no}", "yes"];
        let str_equal2_str = vec!["#{'text' == 'texte'; yes | no}", "no"];
        let int_equal_str = vec!["#{i/4 == 'text'; yes | no}", "no"];
        let float_equal_str = vec!["#{f/4.5 == 'texte'; yes | no}", "no"];
        let bool_equal_str = vec!["#{b/True == 'texte'; yes | no}", "no"];

        let parser = TemplateStr::new(var.var_map, None);

        assert_eq!(parser.parse_condition(str_equal_str[0].to_string()).unwrap(), str_equal_str[1], "str_equal_str");
        assert_eq!(parser.parse_condition(str_equal2_str[0].to_string()).unwrap(), str_equal2_str[1], "str_equal2_str");
        assert_eq!(parser.parse_condition(int_equal_str[0].to_string()).unwrap(), int_equal_str[1], "int_equal_str");
        assert_eq!(parser.parse_condition(float_equal_str[0].to_string()).unwrap(), float_equal_str[1], "float_equal_str");
        assert_eq!(parser.parse_condition(bool_equal_str[0].to_string()).unwrap(), bool_equal_str[1], "bool_equal_str");

    }

    #[test]
    fn test_condition_not_equal() {
        let var = GlobalVar::new();

        let str_not_equal_str = vec!["#{'text' != 'text'; yes | no}", "no"];
        let str_not_equal2_str = vec!["#{'text' != 'texte'; yes | no}", "yes"];
        let int_not_equal_str = vec!["#{i/4 != 'text'; yes | no}", "yes"];
        let float_not_equal_str = vec!["#{f/4.5 != 'texte'; yes | no}", "yes"];
        let bool_not_equal_str = vec!["#{b/True != 'texte'; yes | no}", "yes"];

        let parser = TemplateStr::new(var.var_map, None);

        assert_eq!(parser.parse_condition(str_not_equal_str[0].to_string()).unwrap(), str_not_equal_str[1], "str_not_equal_str");
        assert_eq!(parser.parse_condition(str_not_equal2_str[0].to_string()).unwrap(), str_not_equal2_str[1], "str_not_equal2_str");
        assert_eq!(parser.parse_condition(int_not_equal_str[0].to_string()).unwrap(), int_not_equal_str[1], "int_not_equal_str");
        assert_eq!(parser.parse_condition(float_not_equal_str[0].to_string()).unwrap(), float_not_equal_str[1], "float_not_equal_str");
        assert_eq!(parser.parse_condition(bool_not_equal_str[0].to_string()).unwrap(), bool_not_equal_str[1], "bool_not_equal_str");

    }

    #[test]
    fn test_condition_superior_equal() {
        let var = GlobalVar::new();

        let parser = TemplateStr::new(var.var_map, None);

        // String
        let str_superior_equal_str = vec!["#{'text' >= 'text'; yes | no}", "yes"];
        let str_superior_equal2_str = vec!["#{'text' >= 'texte'; yes | no}", "no"];
        let str_superior_equal_int = vec!["#{'text' >= i/4; yes | no}", "yes"];
        let str_superior_equal2_int = vec!["#{'text' >= i/123; yes | no}", "no"];
        let str_superior_equal_float = vec!["#{'text' >= f/4.5; yes | no}", "no"];
        let str_superior_equal2_float = vec!["#{'text' >= f/3.5; yes | no}", "yes"];
        let str_superior_equal_bool = vec!["#{'text' >= b/True; yes | no}", "yes"];
        let str_superior_equal2_bool = vec!["#{'text' >= b/False; yes | no}", "yes"];

        assert_eq!(parser.parse_condition(str_superior_equal_str[0].to_string()).unwrap(), str_superior_equal_str[1], "str_superior_equal_str");
        assert_eq!(parser.parse_condition(str_superior_equal2_str[0].to_string()).unwrap(), str_superior_equal2_str[1], "str_superior_equal2_str");
        assert_eq!(parser.parse_condition(str_superior_equal_int[0].to_string()).unwrap(), str_superior_equal_int[1], "str_superior_equal_int");
        assert_eq!(parser.parse_condition(str_superior_equal2_int[0].to_string()).unwrap(), str_superior_equal2_int[1], "str_superior_equal2_int");
        assert_eq!(parser.parse_condition(str_superior_equal_float[0].to_string()).unwrap(), str_superior_equal_float[1], "str_superior_equal_float");
        assert_eq!(parser.parse_condition(str_superior_equal2_float[0].to_string()).unwrap(), str_superior_equal2_float[1], "str_superior_equal2_float");
        assert_eq!(parser.parse_condition(str_superior_equal_bool[0].to_string()).unwrap(), str_superior_equal_bool[1], "str_superior_equal_bool");
        assert_eq!(parser.parse_condition(str_superior_equal2_bool[0].to_string()).unwrap(), str_superior_equal2_bool[1], "str_superior_equal2_bool");

        // Int
        let int_superior_equal_str = vec!["#{i/4 >= 'text'; yes | no}", "yes"];
        let int_superior_equal2_str = vec!["#{i/4 >= 'texte'; yes | no}", "no"];
        let int_superior_equal_int = vec!["#{i/4 >= i/4; yes | no}", "yes"];
        let int_superior_equal2_int = vec!["#{i/4 >= i/5; yes | no}", "no"];
        let int_superior_equal_float = vec!["#{i/4 >= f/3.5; yes | no}", "yes"];
        let int_superior_equal2_float = vec!["#{i/4 >= f/4.5; yes | no}", "no"];
        let int_superior_equal_bool = vec!["#{i/4 >= b/True; yes | no}", "yes"];
        let int_superior_equal2_bool = vec!["#{i/4 >= b/False; yes | no}", "yes"];

        assert_eq!(parser.parse_condition(int_superior_equal_str[0].to_string()).unwrap(), int_superior_equal_str[1], "int_superior_equal_str");
        assert_eq!(parser.parse_condition(int_superior_equal2_str[0].to_string()).unwrap(), int_superior_equal2_str[1], "int_superior_equal2_str");
        assert_eq!(parser.parse_condition(int_superior_equal_int[0].to_string()).unwrap(), int_superior_equal_int[1], "int_superior_equal_int");
        assert_eq!(parser.parse_condition(int_superior_equal2_int[0].to_string()).unwrap(), int_superior_equal2_int[1], "int_superior_equal2_int");
        assert_eq!(parser.parse_condition(int_superior_equal_float[0].to_string()).unwrap(), int_superior_equal_float[1], "int_superior_equal_float");
        assert_eq!(parser.parse_condition(int_superior_equal2_float[0].to_string()).unwrap(), int_superior_equal2_float[1], "int_superior_equal2_float");
        assert_eq!(parser.parse_condition(int_superior_equal_bool[0].to_string()).unwrap(), int_superior_equal_bool[1], "int_superior_equal_bool");
        assert_eq!(parser.parse_condition(int_superior_equal2_bool[0].to_string()).unwrap(), int_superior_equal2_bool[1], "int_superior_equal2_bool");

        // Float
        let float_superior_equal_str = vec!["#{f/4.5 >= 'text'; yes | no}", "yes"];
        let float_superior_equal2_str = vec!["#{f/4.5 >= 'texte'; yes | no}", "no"];
        let float_superior_equal_int = vec!["#{f/4.5 >= i/4; yes | no}", "yes"];
        let float_superior_equal2_int = vec!["#{f/4.5 >= i/5; yes | no}", "no"];
        let float_superior_equal_float = vec!["#{f/4.5 >= f/3.5; yes | no}", "yes"];
        let float_superior_equal2_float = vec!["#{f/4.5 >= f/4.6; yes | no}", "no"];
        let float_superior_equal_bool = vec!["#{f/4.5 >= b/True; yes | no}", "yes"];
        let float_superior_equal2_bool = vec!["#{f/4.5 >= b/False; yes | no}", "yes"];

        assert_eq!(parser.parse_condition(float_superior_equal_str[0].to_string()).unwrap(), float_superior_equal_str[1], "float_superior_equal_str");
        assert_eq!(parser.parse_condition(float_superior_equal2_str[0].to_string()).unwrap(), float_superior_equal2_str[1], "float_superior_equal2_str");
        assert_eq!(parser.parse_condition(float_superior_equal_int[0].to_string()).unwrap(), float_superior_equal_int[1], "float_superior_equal_int");
        assert_eq!(parser.parse_condition(float_superior_equal2_int[0].to_string()).unwrap(), float_superior_equal2_int[1], "float_superior_equal2_int");
        assert_eq!(parser.parse_condition(float_superior_equal_float[0].to_string()).unwrap(), float_superior_equal_float[1], "float_superior_equal_float");
        assert_eq!(parser.parse_condition(float_superior_equal2_float[0].to_string()).unwrap(), float_superior_equal2_float[1], "float_superior_equal2_float");
        assert_eq!(parser.parse_condition(float_superior_equal_bool[0].to_string()).unwrap(), float_superior_equal_bool[1], "float_superior_equal_bool");
        assert_eq!(parser.parse_condition(float_superior_equal2_bool[0].to_string()).unwrap(), float_superior_equal2_bool[1], "float_superior_equal2_bool");

        // Bool
        let bool_superior_equal_str = vec!["#{b/True >= 'text'; yes | no}", "no"];
        let bool_superior_equal2_str = vec!["#{b/False >= 'texte'; yes | no}", "no"];
        let bool_superior_equal_int = vec!["#{b/True >= i/4; yes | no}", "no"];
        let bool_superior_equal2_int = vec!["#{b/False >= i/5; yes | no}", "no"];
        let bool_superior_equal_float = vec!["#{b/True >= f/3.5; yes | no}", "no"];
        let bool_superior_equal2_float = vec!["#{b/False >= f/4.6; yes | no}", "no"];
        let bool_superior_equal_bool = vec!["#{b/True >= b/True; yes | no}", "yes"];
        let bool_superior_equal2_bool = vec!["#{b/False >= b/False; yes | no}", "yes"];

        assert_eq!(parser.parse_condition(bool_superior_equal_str[0].to_string()).unwrap(), bool_superior_equal_str[1], "bool_superior_equal_str");
        assert_eq!(parser.parse_condition(bool_superior_equal2_str[0].to_string()).unwrap(), bool_superior_equal2_str[1], "bool_superior_equal2_str");
        assert_eq!(parser.parse_condition(bool_superior_equal_int[0].to_string()).unwrap(), bool_superior_equal_int[1], "bool_superior_equal_int");
        assert_eq!(parser.parse_condition(bool_superior_equal2_int[0].to_string()).unwrap(), bool_superior_equal2_int[1], "bool_superior_equal2_int");
        assert_eq!(parser.parse_condition(bool_superior_equal_float[0].to_string()).unwrap(), bool_superior_equal_float[1], "bool_superior_equal_float");
        assert_eq!(parser.parse_condition(bool_superior_equal2_float[0].to_string()).unwrap(), bool_superior_equal2_float[1], "bool_superior_equal2_float");
        assert_eq!(parser.parse_condition(bool_superior_equal_bool[0].to_string()).unwrap(), bool_superior_equal_bool[1], "bool_superior_equal_bool");
        assert_eq!(parser.parse_condition(bool_superior_equal2_bool[0].to_string()).unwrap(), bool_superior_equal2_bool[1], "bool_superior_equal2_bool");

    }

    #[test]
    fn test_condition_superior() {
        let var = GlobalVar::new();

        let parser = TemplateStr::new(var.var_map, None);

        // String
        let str_superior_str = vec!["#{'text' > 'text'; yes | no}", "no"];
        let str_superior2_str = vec!["#{'text' > 'texte'; yes | no}", "no"];
        let str_superior_int = vec!["#{'text' > i/4; yes | no}", "no"];
        let str_superior2_int = vec!["#{'text' > i/123; yes | no}", "no"];
        let str_superior_float = vec!["#{'text' > f/4.5; yes | no}", "no"];
        let str_superior2_float = vec!["#{'text' > f/3.5; yes | no}", "yes"];
        let str_superior_bool = vec!["#{'text' > b/True; yes | no}", "yes"];
        let str_superior2_bool = vec!["#{'text' > b/False; yes | no}", "yes"];

        assert_eq!(parser.parse_condition(str_superior_str[0].to_string()).unwrap(), str_superior_str[1], "str_superior_str");
        assert_eq!(parser.parse_condition(str_superior2_str[0].to_string()).unwrap(), str_superior2_str[1], "str_superior2_str");
        assert_eq!(parser.parse_condition(str_superior_int[0].to_string()).unwrap(), str_superior_int[1], "str_superior_int");
        assert_eq!(parser.parse_condition(str_superior2_int[0].to_string()).unwrap(), str_superior2_int[1], "str_superior2_int");
        assert_eq!(parser.parse_condition(str_superior_float[0].to_string()).unwrap(), str_superior_float[1], "str_superior_float");
        assert_eq!(parser.parse_condition(str_superior2_float[0].to_string()).unwrap(), str_superior2_float[1], "str_superior2_float");
        assert_eq!(parser.parse_condition(str_superior_bool[0].to_string()).unwrap(), str_superior_bool[1], "str_superior_bool");
        assert_eq!(parser.parse_condition(str_superior2_bool[0].to_string()).unwrap(), str_superior2_bool[1], "str_superior2_bool");

        // Int
        let int_superior_str = vec!["#{i/4 > 'text'; yes | no}", "no"];
        let int_superior2_str = vec!["#{i/4 > 'texte'; yes | no}", "no"];
        let int_superior_int = vec!["#{i/4 > i/4; yes | no}", "no"];
        let int_superior2_int = vec!["#{i/4 > i/5; yes | no}", "no"];
        let int_superior_float = vec!["#{i/4 > f/3.5; yes | no}", "yes"];
        let int_superior2_float = vec!["#{i/4 > f/4.5; yes | no}", "no"];
        let int_superior_bool = vec!["#{i/4 > b/True; yes | no}", "yes"];
        let int_superior2_bool = vec!["#{i/4 > b/False; yes | no}", "yes"];

        assert_eq!(parser.parse_condition(int_superior_str[0].to_string()).unwrap(), int_superior_str[1], "int_superior_str");
        assert_eq!(parser.parse_condition(int_superior2_str[0].to_string()).unwrap(), int_superior2_str[1], "int_superior2_str");
        assert_eq!(parser.parse_condition(int_superior_int[0].to_string()).unwrap(), int_superior_int[1], "int_superior_int");
        assert_eq!(parser.parse_condition(int_superior2_int[0].to_string()).unwrap(), int_superior2_int[1], "int_superior2_int");
        assert_eq!(parser.parse_condition(int_superior_float[0].to_string()).unwrap(), int_superior_float[1], "int_superior_float");
        assert_eq!(parser.parse_condition(int_superior2_float[0].to_string()).unwrap(), int_superior2_float[1], "int_superior2_float");
        assert_eq!(parser.parse_condition(int_superior_bool[0].to_string()).unwrap(), int_superior_bool[1], "int_superior_bool");
        assert_eq!(parser.parse_condition(int_superior2_bool[0].to_string()).unwrap(), int_superior2_bool[1], "int_superior2_bool");

        // Float
        let float_superior_str = vec!["#{f/4.5 > 'text'; yes | no}", "yes"];
        let float_superior2_str = vec!["#{f/4.5 > 'texte'; yes | no}", "no"];
        let float_superior_int = vec!["#{f/4.5 > i/4; yes | no}", "yes"];
        let float_superior2_int = vec!["#{f/4.5 > i/5; yes | no}", "no"];
        let float_superior_float = vec!["#{f/4.5 > f/3.5; yes | no}", "yes"];
        let float_superior2_float = vec!["#{f/4.5 > f/4.6; yes | no}", "no"];
        let float_superior_bool = vec!["#{f/4.5 > b/True; yes | no}", "yes"];
        let float_superior2_bool = vec!["#{f/4.5 > b/False; yes | no}", "yes"];

        assert_eq!(parser.parse_condition(float_superior_str[0].to_string()).unwrap(), float_superior_str[1], "float_superior_str");
        assert_eq!(parser.parse_condition(float_superior2_str[0].to_string()).unwrap(), float_superior2_str[1], "float_superior2_str");
        assert_eq!(parser.parse_condition(float_superior_int[0].to_string()).unwrap(), float_superior_int[1], "float_superior_int");
        assert_eq!(parser.parse_condition(float_superior2_int[0].to_string()).unwrap(), float_superior2_int[1], "float_superior2_int");
        assert_eq!(parser.parse_condition(float_superior_float[0].to_string()).unwrap(), float_superior_float[1], "float_superior_float");
        assert_eq!(parser.parse_condition(float_superior2_float[0].to_string()).unwrap(), float_superior2_float[1], "float_superior2_float");
        assert_eq!(parser.parse_condition(float_superior_bool[0].to_string()).unwrap(), float_superior_bool[1], "float_superior_bool");
        assert_eq!(parser.parse_condition(float_superior2_bool[0].to_string()).unwrap(), float_superior2_bool[1], "float_superior2_bool");

        // Bool
        let bool_superior_str = vec!["#{b/True > 'text'; yes | no}", "no"];
        let bool_superior2_str = vec!["#{b/False > 'texte'; yes | no}", "no"];
        let bool_superior_int = vec!["#{b/True > i/4; yes | no}", "no"];
        let bool_superior2_int = vec!["#{b/False > i/5; yes | no}", "no"];
        let bool_superior_float = vec!["#{b/True > f/3.5; yes | no}", "no"];
        let bool_superior2_float = vec!["#{b/False > f/4.6; yes | no}", "no"];
        let bool_superior_bool = vec!["#{b/True > b/True; yes | no}", "no"];
        let bool_superior2_bool = vec!["#{b/False > b/False; yes | no}", "no"];

        assert_eq!(parser.parse_condition(bool_superior_str[0].to_string()).unwrap(), bool_superior_str[1], "bool_superior_str");
        assert_eq!(parser.parse_condition(bool_superior2_str[0].to_string()).unwrap(), bool_superior2_str[1], "bool_superior2_str");
        assert_eq!(parser.parse_condition(bool_superior_int[0].to_string()).unwrap(), bool_superior_int[1], "bool_superior_int");
        assert_eq!(parser.parse_condition(bool_superior2_int[0].to_string()).unwrap(), bool_superior2_int[1], "bool_superior2_int");
        assert_eq!(parser.parse_condition(bool_superior_float[0].to_string()).unwrap(), bool_superior_float[1], "bool_superior_float");
        assert_eq!(parser.parse_condition(bool_superior2_float[0].to_string()).unwrap(), bool_superior2_float[1], "bool_superior2_float");
        assert_eq!(parser.parse_condition(bool_superior_bool[0].to_string()).unwrap(), bool_superior_bool[1], "bool_superior_bool");
        assert_eq!(parser.parse_condition(bool_superior2_bool[0].to_string()).unwrap(), bool_superior2_bool[1], "bool_superior2_bool");

    }

    #[test]
    fn test_condition_inferior_equal() {
        let var = GlobalVar::new();

        let parser = TemplateStr::new(var.var_map, None);

        // String
        let str_inferior_equal_str = vec!["#{'text' <= 'text'; yes | no}", "yes"];
        let str_inferior_equal2_str = vec!["#{'text' <= 'texte'; yes | no}", "yes"];
        let str_inferior_equal_int = vec!["#{'text' <= i/4; yes | no}", "yes"];
        let str_inferior_equal2_int = vec!["#{'text' <= i/123; yes | no}", "yes"];
        let str_inferior_equal_float = vec!["#{'text' <= f/4.5; yes | no}", "yes"];
        let str_inferior_equal2_float = vec!["#{'text' <= f/3.5; yes | no}", "no"];
        let str_inferior_equal_bool = vec!["#{'text' <= b/True; yes | no}", "no"];
        let str_inferior_equal2_bool = vec!["#{'text' <= b/False; yes | no}", "no"];

        assert_eq!(parser.parse_condition(str_inferior_equal_str[0].to_string()).unwrap(), str_inferior_equal_str[1], "str_inferior_equal_str");
        assert_eq!(parser.parse_condition(str_inferior_equal2_str[0].to_string()).unwrap(), str_inferior_equal2_str[1], "str_inferior_equal2_str");
        assert_eq!(parser.parse_condition(str_inferior_equal_int[0].to_string()).unwrap(), str_inferior_equal_int[1], "str_inferior_equal_int");
        assert_eq!(parser.parse_condition(str_inferior_equal2_int[0].to_string()).unwrap(), str_inferior_equal2_int[1], "str_inferior_equal2_int");
        assert_eq!(parser.parse_condition(str_inferior_equal_float[0].to_string()).unwrap(), str_inferior_equal_float[1], "str_inferior_equal_float");
        assert_eq!(parser.parse_condition(str_inferior_equal2_float[0].to_string()).unwrap(), str_inferior_equal2_float[1], "str_inferior_equal2_float");
        assert_eq!(parser.parse_condition(str_inferior_equal_bool[0].to_string()).unwrap(), str_inferior_equal_bool[1], "str_inferior_equal_bool");
        assert_eq!(parser.parse_condition(str_inferior_equal2_bool[0].to_string()).unwrap(), str_inferior_equal2_bool[1], "str_inferior_equal2_bool");

        // Int
        let int_inferior_equal_str = vec!["#{i/4 <= 'text'; yes | no}", "yes"];
        let int_inferior_equal2_str = vec!["#{i/4 <= 'texte'; yes | no}", "yes"];
        let int_inferior_equal_int = vec!["#{i/4 <= i/4; yes | no}", "yes"];
        let int_inferior_equal2_int = vec!["#{i/4 <= i/5; yes | no}", "yes"];
        let int_inferior_equal_float = vec!["#{i/4 <= f/3.5; yes | no}", "no"];
        let int_inferior_equal2_float = vec!["#{i/4 <= f/4.5; yes | no}", "yes"];
        let int_inferior_equal_bool = vec!["#{i/4 <= b/True; yes | no}", "no"];
        let int_inferior_equal2_bool = vec!["#{i/4 <= b/False; yes | no}", "no"];

        assert_eq!(parser.parse_condition(int_inferior_equal_str[0].to_string()).unwrap(), int_inferior_equal_str[1], "int_inferior_equal_str");
        assert_eq!(parser.parse_condition(int_inferior_equal2_str[0].to_string()).unwrap(), int_inferior_equal2_str[1], "int_inferior_equal2_str");
        assert_eq!(parser.parse_condition(int_inferior_equal_int[0].to_string()).unwrap(), int_inferior_equal_int[1], "int_inferior_equal_int");
        assert_eq!(parser.parse_condition(int_inferior_equal2_int[0].to_string()).unwrap(), int_inferior_equal2_int[1], "int_inferior_equal2_int");
        assert_eq!(parser.parse_condition(int_inferior_equal_float[0].to_string()).unwrap(), int_inferior_equal_float[1], "int_inferior_equal_float");
        assert_eq!(parser.parse_condition(int_inferior_equal2_float[0].to_string()).unwrap(), int_inferior_equal2_float[1], "int_inferior_equal2_float");
        assert_eq!(parser.parse_condition(int_inferior_equal_bool[0].to_string()).unwrap(), int_inferior_equal_bool[1], "int_inferior_equal_bool");
        assert_eq!(parser.parse_condition(int_inferior_equal2_bool[0].to_string()).unwrap(), int_inferior_equal2_bool[1], "int_inferior_equal2_bool");

        // Float
        let float_inferior_equal_str = vec!["#{f/4.5 <= 'text'; yes | no}", "no"];
        let float_inferior_equal2_str = vec!["#{f/4.5 <= 'texte'; yes | no}", "yes"];
        let float_inferior_equal_int = vec!["#{f/4.5 <= i/4; yes | no}", "no"];
        let float_inferior_equal2_int = vec!["#{f/4.5 <= i/5; yes | no}", "yes"];
        let float_inferior_equal_float = vec!["#{f/4.5 <= f/3.5; yes | no}", "no"];
        let float_inferior_equal2_float = vec!["#{f/4.5 <= f/4.6; yes | no}", "yes"];
        let float_inferior_equal_bool = vec!["#{f/4.5 <= b/True; yes | no}", "no"];
        let float_inferior_equal2_bool = vec!["#{f/4.5 <= b/False; yes | no}", "no"];

        assert_eq!(parser.parse_condition(float_inferior_equal_str[0].to_string()).unwrap(), float_inferior_equal_str[1], "float_inferior_equal_str");
        assert_eq!(parser.parse_condition(float_inferior_equal2_str[0].to_string()).unwrap(), float_inferior_equal2_str[1], "float_inferior_equal2_str");
        assert_eq!(parser.parse_condition(float_inferior_equal_int[0].to_string()).unwrap(), float_inferior_equal_int[1], "float_inferior_equal_int");
        assert_eq!(parser.parse_condition(float_inferior_equal2_int[0].to_string()).unwrap(), float_inferior_equal2_int[1], "float_inferior_equal2_int");
        assert_eq!(parser.parse_condition(float_inferior_equal_float[0].to_string()).unwrap(), float_inferior_equal_float[1], "float_inferior_equal_float");
        assert_eq!(parser.parse_condition(float_inferior_equal2_float[0].to_string()).unwrap(), float_inferior_equal2_float[1], "float_inferior_equal2_float");
        assert_eq!(parser.parse_condition(float_inferior_equal_bool[0].to_string()).unwrap(), float_inferior_equal_bool[1], "float_inferior_equal_bool");
        assert_eq!(parser.parse_condition(float_inferior_equal2_bool[0].to_string()).unwrap(), float_inferior_equal2_bool[1], "float_inferior_equal2_bool");

        // Bool
        let bool_inferior_equal_str = vec!["#{b/True <= 'text'; yes | no}", "yes"];
        let bool_inferior_equal2_str = vec!["#{b/False <= 'texte'; yes | no}", "yes"];
        let bool_inferior_equal_int = vec!["#{b/True <= i/4; yes | no}", "yes"];
        let bool_inferior_equal2_int = vec!["#{b/False <= i/5; yes | no}", "yes"];
        let bool_inferior_equal_float = vec!["#{b/True <= f/3.5; yes | no}", "yes"];
        let bool_inferior_equal2_float = vec!["#{b/False <= f/4.6; yes | no}", "yes"];
        let bool_inferior_equal_bool = vec!["#{b/True <= b/True; yes | no}", "yes"];
        let bool_inferior_equal2_bool = vec!["#{b/False <= b/False; yes | no}", "yes"];

        assert_eq!(parser.parse_condition(bool_inferior_equal_str[0].to_string()).unwrap(), bool_inferior_equal_str[1], "bool_inferior_equal_str");
        assert_eq!(parser.parse_condition(bool_inferior_equal2_str[0].to_string()).unwrap(), bool_inferior_equal2_str[1], "bool_inferior_equal2_str");
        assert_eq!(parser.parse_condition(bool_inferior_equal_int[0].to_string()).unwrap(), bool_inferior_equal_int[1], "bool_inferior_equal_int");
        assert_eq!(parser.parse_condition(bool_inferior_equal2_int[0].to_string()).unwrap(), bool_inferior_equal2_int[1], "bool_inferior_equal2_int");
        assert_eq!(parser.parse_condition(bool_inferior_equal_float[0].to_string()).unwrap(), bool_inferior_equal_float[1], "bool_inferior_equal_float");
        assert_eq!(parser.parse_condition(bool_inferior_equal2_float[0].to_string()).unwrap(), bool_inferior_equal2_float[1], "bool_inferior_equal2_float");
        assert_eq!(parser.parse_condition(bool_inferior_equal_bool[0].to_string()).unwrap(), bool_inferior_equal_bool[1], "bool_inferior_equal_bool");
        assert_eq!(parser.parse_condition(bool_inferior_equal2_bool[0].to_string()).unwrap(), bool_inferior_equal2_bool[1], "bool_inferior_equal2_bool");

    }

    #[test]
    fn test_condition_inferior() {
        let var = GlobalVar::new();

        let parser = TemplateStr::new(var.var_map, None);

        // String
        let str_inferior_str = vec!["#{'text' < 'text'; yes | no}", "no"];
        let str_inferior2_str = vec!["#{'text' < 'texte'; yes | no}", "yes"];
        let str_inferior_int = vec!["#{'text' < i/4; yes | no}", "no"];
        let str_inferior2_int = vec!["#{'text' < i/123; yes | no}", "yes"];
        let str_inferior_float = vec!["#{'text' < f/4.5; yes | no}", "yes"];
        let str_inferior2_float = vec!["#{'text' < f/3.5; yes | no}", "no"];
        let str_inferior_bool = vec!["#{'text' < b/True; yes | no}", "no"];
        let str_inferior2_bool = vec!["#{'text' < b/False; yes | no}", "no"];

        assert_eq!(parser.parse_condition(str_inferior_str[0].to_string()).unwrap(), str_inferior_str[1], "str_inferior_str");
        assert_eq!(parser.parse_condition(str_inferior2_str[0].to_string()).unwrap(), str_inferior2_str[1], "str_inferior2_str");
        assert_eq!(parser.parse_condition(str_inferior_int[0].to_string()).unwrap(), str_inferior_int[1], "str_inferior_int");
        assert_eq!(parser.parse_condition(str_inferior2_int[0].to_string()).unwrap(), str_inferior2_int[1], "str_inferior2_int");
        assert_eq!(parser.parse_condition(str_inferior_float[0].to_string()).unwrap(), str_inferior_float[1], "str_inferior_float");
        assert_eq!(parser.parse_condition(str_inferior2_float[0].to_string()).unwrap(), str_inferior2_float[1], "str_inferior2_float");
        assert_eq!(parser.parse_condition(str_inferior_bool[0].to_string()).unwrap(), str_inferior_bool[1], "str_inferior_bool");
        assert_eq!(parser.parse_condition(str_inferior2_bool[0].to_string()).unwrap(), str_inferior2_bool[1], "str_inferior2_bool");

        // Int
        let int_inferior_str = vec!["#{i/4 < 'text'; yes | no}", "no"];
        let int_inferior2_str = vec!["#{i/4 < 'texte'; yes | no}", "yes"];
        let int_inferior_int = vec!["#{i/4 < i/4; yes | no}", "no"];
        let int_inferior2_int = vec!["#{i/4 < i/5; yes | no}", "yes"];
        let int_inferior_float = vec!["#{i/4 < f/3.5; yes | no}", "no"];
        let int_inferior2_float = vec!["#{i/4 < f/4.5; yes | no}", "yes"];
        let int_inferior_bool = vec!["#{i/4 < b/True; yes | no}", "no"];
        let int_inferior2_bool = vec!["#{i/4 < b/False; yes | no}", "no"];

        assert_eq!(parser.parse_condition(int_inferior_str[0].to_string()).unwrap(), int_inferior_str[1], "int_inferior_str");
        assert_eq!(parser.parse_condition(int_inferior2_str[0].to_string()).unwrap(), int_inferior2_str[1], "int_inferior2_str");
        assert_eq!(parser.parse_condition(int_inferior_int[0].to_string()).unwrap(), int_inferior_int[1], "int_inferior_int");
        assert_eq!(parser.parse_condition(int_inferior2_int[0].to_string()).unwrap(), int_inferior2_int[1], "int_inferior2_int");
        assert_eq!(parser.parse_condition(int_inferior_float[0].to_string()).unwrap(), int_inferior_float[1], "int_inferior_float");
        assert_eq!(parser.parse_condition(int_inferior2_float[0].to_string()).unwrap(), int_inferior2_float[1], "int_inferior2_float");
        assert_eq!(parser.parse_condition(int_inferior_bool[0].to_string()).unwrap(), int_inferior_bool[1], "int_inferior_bool");
        assert_eq!(parser.parse_condition(int_inferior2_bool[0].to_string()).unwrap(), int_inferior2_bool[1], "int_inferior2_bool");

        // Float
        let float_inferior_str = vec!["#{f/4.5 < 'text'; yes | no}", "no"];
        let float_inferior2_str = vec!["#{f/4.5 < 'texte'; yes | no}", "yes"];
        let float_inferior_int = vec!["#{f/4.5 < i/4; yes | no}", "no"];
        let float_inferior2_int = vec!["#{f/4.5 < i/5; yes | no}", "yes"];
        let float_inferior_float = vec!["#{f/4.5 < f/3.5; yes | no}", "no"];
        let float_inferior2_float = vec!["#{f/4.5 < f/4.6; yes | no}", "yes"];
        let float_inferior_bool = vec!["#{f/4.5 < b/True; yes | no}", "no"];
        let float_inferior2_bool = vec!["#{f/4.5 < b/False; yes | no}", "no"];

        assert_eq!(parser.parse_condition(float_inferior_str[0].to_string()).unwrap(), float_inferior_str[1], "float_inferior_str");
        assert_eq!(parser.parse_condition(float_inferior2_str[0].to_string()).unwrap(), float_inferior2_str[1], "float_inferior2_str");
        assert_eq!(parser.parse_condition(float_inferior_int[0].to_string()).unwrap(), float_inferior_int[1], "float_inferior_int");
        assert_eq!(parser.parse_condition(float_inferior2_int[0].to_string()).unwrap(), float_inferior2_int[1], "float_inferior2_int");
        assert_eq!(parser.parse_condition(float_inferior_float[0].to_string()).unwrap(), float_inferior_float[1], "float_inferior_float");
        assert_eq!(parser.parse_condition(float_inferior2_float[0].to_string()).unwrap(), float_inferior2_float[1], "float_inferior2_float");
        assert_eq!(parser.parse_condition(float_inferior_bool[0].to_string()).unwrap(), float_inferior_bool[1], "float_inferior_bool");
        assert_eq!(parser.parse_condition(float_inferior2_bool[0].to_string()).unwrap(), float_inferior2_bool[1], "float_inferior2_bool");

        // Bool
        let bool_inferior_str = vec!["#{b/True < 'text'; yes | no}", "yes"];
        let bool_inferior2_str = vec!["#{b/False < 'texte'; yes | no}", "yes"];
        let bool_inferior_int = vec!["#{b/True < i/4; yes | no}", "yes"];
        let bool_inferior2_int = vec!["#{b/False < i/5; yes | no}", "yes"];
        let bool_inferior_float = vec!["#{b/True < f/3.5; yes | no}", "yes"];
        let bool_inferior2_float = vec!["#{b/False < f/4.6; yes | no}", "yes"];
        let bool_inferior_bool = vec!["#{b/True < b/True; yes | no}", "no"];
        let bool_inferior2_bool = vec!["#{b/False < b/False; yes | no}", "no"];

        assert_eq!(parser.parse_condition(bool_inferior_str[0].to_string()).unwrap(), bool_inferior_str[1], "bool_inferior_str");
        assert_eq!(parser.parse_condition(bool_inferior2_str[0].to_string()).unwrap(), bool_inferior2_str[1], "bool_inferior2_str");
        assert_eq!(parser.parse_condition(bool_inferior_int[0].to_string()).unwrap(), bool_inferior_int[1], "bool_inferior_int");
        assert_eq!(parser.parse_condition(bool_inferior2_int[0].to_string()).unwrap(), bool_inferior2_int[1], "bool_inferior2_int");
        assert_eq!(parser.parse_condition(bool_inferior_float[0].to_string()).unwrap(), bool_inferior_float[1], "bool_inferior_float");
        assert_eq!(parser.parse_condition(bool_inferior2_float[0].to_string()).unwrap(), bool_inferior2_float[1], "bool_inferior2_float");
        assert_eq!(parser.parse_condition(bool_inferior_bool[0].to_string()).unwrap(), bool_inferior_bool[1], "bool_inferior_bool");
        assert_eq!(parser.parse_condition(bool_inferior2_bool[0].to_string()).unwrap(), bool_inferior2_bool[1], "bool_inferior2_bool");

    }
}

#[cfg(test)]
mod switch {
    use super::*;

    #[test]
    fn test_switch() {
        let var = GlobalVar::new();

        let text_switch_1 = vec!["?{str; Jame::#0, Tony::#1, Marco::#2, _::#default}", "#0"];
        let text_switch_2 = vec!["?{int/int; 56::#0, 36::#1, 32::#2, _::#default}", "#2"];
        let text_switch_3 = vec!["?{int/int; 56::#0, 36::#1, 32::?{str/str; Jame::#42, 36::#1, 32::#2, _::#default}, _::#default}", "#42"];

        let parser = TemplateStr::new(var.var_map, None);
        
        assert_eq!(parser.parse_switch(text_switch_1[0].to_string()).unwrap(), text_switch_1[1], "text_switch_1");
        assert_eq!(parser.parse_switch(text_switch_2[0].to_string()).unwrap(), text_switch_2[1], "text_switch_2");
        assert_eq!(parser.parse_switch(text_switch_3[0].to_string()).unwrap(), text_switch_3[1], "text_switch_3");
    }
}


#[cfg(test)]
mod has {
    use super::*;

    #[test]
    fn test_has_variable() {
        let var = GlobalVar::new();

        let text_has_variable_1 = "${bool} and ${name}";
        let text_has_variable_2 = "${bool} and @{uppercase lower}";
        let text_has_variable_3 = "@{uppercaseFirst bool} and @{uppercase lower}";

        let parser = TemplateStr::new(var.var_map, None);

        assert_eq!(parser.has_variable(text_has_variable_1.to_string()), true, "text_has_variable_1");
        assert_eq!(parser.has_variable(text_has_variable_2.to_string()), true, "text_has_variable_2");
        assert_eq!(parser.has_variable(text_has_variable_3.to_string()), false, "text_has_variable_3");
    }

    #[test]
    fn test_has_function() {
        let var = GlobalVar::new();

        let text_has_function_1 = "@{uppercase; lower} and @{uppercaseFirst; lower}";
        let text_has_function_2 = "@{uppercase; lower} and #{'text' > 'text'; yes | no}";
        let text_has_function_3 = "#{'text' > 'text'; yes | no} and #{'text' < 'text'; yes | no}";

        let parser = TemplateStr::new(var.var_map, None);

        assert_eq!(parser.has_function(text_has_function_1.to_string()), true, "text_has_function_1");
        assert_eq!(parser.has_function(text_has_function_2.to_string()), true, "text_has_function_2");
        assert_eq!(parser.has_function(text_has_function_3.to_string()), false, "text_has_function_3");
    }

    #[test]
    fn test_has_condition() {
        let var = GlobalVar::new();

        let text_has_condition_1 = "#{'text' > 'text'; yes | no} and #{'text' < 'text'; yes | no}";
        let text_has_condition_2 = "#{'text' > 'text'; yes | no} and ?{age:int; 56:#0, 36:#1, 32:#2, _:#default}";
        let text_has_condition_3 = "?{age:int; 56:#0, 36:#1, 32:#2, _:#default} and ?{age:int; 56:#0, 36:#1, 32:#2, _:#default}";

        let parser = TemplateStr::new(var.var_map, None);

        assert_eq!(parser.has_condition(text_has_condition_1.to_string()), true, "text_has_condition_1");
        assert_eq!(parser.has_condition(text_has_condition_2.to_string()), true, "text_has_condition_2");
        assert_eq!(parser.has_condition(text_has_condition_3.to_string()), false, "text_has_condition_3");
    }

    #[test]
    fn test_has_switch() {
        let var = GlobalVar::new();

        let text_has_switch_1 = "?{int/int; 56::#0, 36::#1, 32::#2, _::#default} and ?{int/int; 56::#0, 36::#1, 32::#2, _::#default}";
        let text_has_switch_2 = "?{int/int; 56::#0, 36::#1, 32::#2, _::#default} and ${bool}";
        let text_has_switch_3 = "${bool} and ${name}";

        let parser = TemplateStr::new(var.var_map, None);

        assert_eq!(parser.has_switch(text_has_switch_1.to_string()), true, "text_has_switch_1");
        assert_eq!(parser.has_switch(text_has_switch_2.to_string()), true, "text_has_switch_2");
        assert_eq!(parser.has_switch(text_has_switch_3.to_string()), false, "text_has_switch_3");
    }

}