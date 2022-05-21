use template_str::{TemplateStr, t_type::{VariableMap as VMap, TVal, FuncMap}, varmap, list_func};
use chrono::Utc;

struct GlobalVar{
    var_map: VMap,
    list_func: FuncMap,
}

fn test(_: Vec<TVal>) -> String {
    return "Test custom Function".to_string();
}

fn test_type(list: Vec<TVal>) -> String {

    let mut text: String = "start".to_string();

    for i in list {
        match i {
            TVal::Str(_) => { text = format!("{} : {}", text, "test Str") },
            TVal::Int(_) => { text = format!("{} : {}", text, "test Int") },
            TVal::Float(_) => { text = format!("{} : {}", text, "test Float") },
            TVal::Bool(_) => { text = format!("{} : {}", text, "test Bool") },
            TVal::Hashmap(_) => { text = format!("{} : {}", text, "test Hashmap") },
            // _ => { text = format!("{} : {}", text, "None") },
        };
    }
    return text;
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

        let test_all_1 = vec!["Name is @{{uppercase; str}}, ${{int}} years old. Map: ${{Map.value}}. my keyboard; #{{lower == 'azerty'; azerty | qwerty}}, ?{{lower; azerty:yes, AZERTY:no, _:anyway}}",
        "Name is JAME, 32 years old. Map: Map in Map. my keyboard; azerty, yes"];
        let test_all_2 = vec!["test var in var ${{${{var}}}}", "test var in var 32"];
        let test_all_3 = vec!["test func in func @{{lowercase; @{{uppercase; str}}}}", "test func in func none"];
        let test_all_4 = vec!["test if in if #{{lower == 'azerty2'; azerty | #{{lower == 'querty'; yes | no}}}}","test if in if no"];
        let test_all_5 = vec!["test switch in switch ?{{str; Jame:?{{Build; Succes:#0, Failed:#1, _:#default}}, Tony:#1, Marco:#2, _:#default}}", "test switch in switch #0"];
        let test_all_6 = vec!["test wtf ?{{str; Jame:?{{int/${{var}}; 32:#0, 36:#1, _:#default}}, Tony:#1, Marco:#2, _:#default2}}", "test wtf #0"];

        let parser = TemplateStr::new(var.var_map, Some(var.list_func));

        assert_eq!(parser.parse(test_all_1[0].to_string()), test_all_1[1], "test_all_1");
        assert_eq!(parser.parse(test_all_2[0].to_string()), test_all_2[1], "test_all_2");
        assert_eq!(parser.parse(test_all_3[0].to_string()), test_all_3[1], "test_all_3");
        assert_eq!(parser.parse(test_all_4[0].to_string()), test_all_4[1], "test_all_4");
        assert_eq!(parser.parse(test_all_5[0].to_string()), test_all_5[1], "test_all_5");
        assert_eq!(parser.parse(test_all_6[0].to_string()), test_all_6[1], "test_all_6");
    }
}

#[cfg(test)]
mod variable {
    use super::*;

    #[test]
    fn test_variable() {

        let var = GlobalVar::new();

        let text_1 = vec!["var bool = ${{bool}} and name = ${{str}}", "var bool = true and name = Jame"];
        let text_2 = vec!["${{Map.value}}", "Map in Map"];
        let text_3 = vec!["${{MasterMap.SecondMap.value}}", "Map in Map in Map"];
        let text_4 = vec!["${{word}}", "None"];
        let text_5 = vec!["${{Map.SecondMap.value}}", "None"];
        let text_6 = vec!["${{${{var2}}}}", "Jame"];

        let parser = TemplateStr::new(var.var_map, None);
        
        assert_eq!(parser.parse_variable(text_1[0].to_string()), text_1[1], "text_1");
        assert_eq!(parser.parse_variable(text_2[0].to_string()), text_2[1], "text_2");
        assert_eq!(parser.parse_variable(text_3[0].to_string()), text_3[1], "text_3");
        assert_eq!(parser.parse_variable(text_4[0].to_string()), text_4[1], "text_4");
        assert_eq!(parser.parse_variable(text_5[0].to_string()), text_5[1], "text_5");
        assert_eq!(parser.parse_variable(text_6[0].to_string()), text_6[1], "text_6");
    }
}

#[cfg(test)]
mod function {

    use super::*;

    #[test]
    fn test_intern_function() {
        let var = GlobalVar::new();

        let uppercase = vec!["@{{uppercase; lower}}", "AZERTY"];
        let uppercase2 = vec!["@{{uppercase; word}}", "NONE"];
        let uppercase_first = vec!["@{{uppercaseFirst; lower}}", "Azerty"];
        let lowercase = vec!["@{{lowercase; upper}}", "azerty"];
        // let casefold = vec!["@{{casefold cfold}}", "grüssen"];
        let swapcase = vec!["@{{swapcase; swap}}", "aZeRtY"];
        let time = "@{{time}}";
        let date = "@{{date}}";
        let date_time = "@{{dateTime}}";
        let function_in_function = vec!["@{{uppercase; @{{lowercase; var2}}}}", "JAME"];

        let parser = TemplateStr::new(var.var_map, None);

        assert_eq!(parser.parse_function(uppercase[0].to_string()), uppercase[1], "uppercase");
        assert_eq!(parser.parse_function(uppercase2[0].to_string()), uppercase2[1], "uppercase2");
        assert_eq!(parser.parse_function(uppercase_first[0].to_string()), uppercase_first[1], "uppercase_first");
        assert_eq!(parser.parse_function(lowercase[0].to_string()), lowercase[1], "lowercase");
        // assert_eq!(parser.parse_function(casefold[0].to_string()), casefold[1], "");
        assert_eq!(parser.parse_function(swapcase[0].to_string()), swapcase[1], "swapcase");
        assert_eq!(parser.parse_function(time.to_string()), Utc::now().format("%H:%M:%S").to_string().as_str(), "time");
        assert_eq!(parser.parse_function(date.to_string()), Utc::now().format("%d/%m/%Y").to_string().as_str(), "date");
        assert_eq!(parser.parse_function(date_time.to_string()), Utc::now().format("%d/%m/%Y %H:%M:%S").to_string().as_str(), "date_time");
        assert_eq!(parser.parse_function(function_in_function[0].to_string()), function_in_function[1], "function_in_function");

    }

    #[test]
    fn test_custom_function() {
        let var = GlobalVar::new();

        let test_1 = vec!["@{{test}}", "Test custom Function"];
        let test_2 = vec![r#"@{{test_type; "text1" 'text2' `text3` b/True i/123 f/123.4 int}}"#, "start : test Str : test Str : test Str : test Bool : test Int : test Float : test Str"];

        let parser = TemplateStr::new(var.var_map, Some(var.list_func));

        assert_eq!(parser.parse_function(test_1[0].to_string()), test_1[1], "");
        assert_eq!(parser.parse_function(test_2[0].to_string()), test_2[1], "");

    }
}

#[cfg(test)]
mod condition {

    use super::*;

    #[test]
    fn test_condition_in_condition() {
        let var = GlobalVar::new();

        let parser = TemplateStr::new(var.var_map, None);

        let condition_in_condition = vec!["#{{'text' == 'text'; yes | #{{'text' != 'text'; yes | no}}}}", "yes"];

        assert_eq!(parser.parse_condition(condition_in_condition[0].to_string()), condition_in_condition[1], "condition_in_condition");
    }

    #[test]
    fn test_condition_equal() {
        let var = GlobalVar::new();

        let str_equal_str = vec!["#{{'text' == 'text'; yes | no}}", "yes"];
        let str_equal2_str = vec!["#{{'text' == 'texte'; yes | no}}", "no"];
        let int_equal_str = vec!["#{{i/4 == 'text'; yes | no}}", "no"];
        let float_equal_str = vec!["#{{f/4.5 == 'texte'; yes | no}}", "no"];
        let bool_equal_str = vec!["#{{b/True == 'texte'; yes | no}}", "no"];

        let parser = TemplateStr::new(var.var_map, None);

        assert_eq!(parser.parse_condition(str_equal_str[0].to_string()), str_equal_str[1], "str_equal_str");
        assert_eq!(parser.parse_condition(str_equal2_str[0].to_string()), str_equal2_str[1], "str_equal2_str");
        assert_eq!(parser.parse_condition(int_equal_str[0].to_string()), int_equal_str[1], "int_equal_str");
        assert_eq!(parser.parse_condition(float_equal_str[0].to_string()), float_equal_str[1], "float_equal_str");
        assert_eq!(parser.parse_condition(bool_equal_str[0].to_string()), bool_equal_str[1], "bool_equal_str");

    }

    #[test]
    fn test_condition_not_equal() {
        let var = GlobalVar::new();

        let str_not_equal_str = vec!["#{{'text' != 'text'; yes | no}}", "no"];
        let str_not_equal2_str = vec!["#{{'text' != 'texte'; yes | no}}", "yes"];
        let int_not_equal_str = vec!["#{{i/4 != 'text'; yes | no}}", "yes"];
        let float_not_equal_str = vec!["#{{f/4.5 != 'texte'; yes | no}}", "yes"];
        let bool_not_equal_str = vec!["#{{b/True != 'texte'; yes | no}}", "yes"];

        let parser = TemplateStr::new(var.var_map, None);

        assert_eq!(parser.parse_condition(str_not_equal_str[0].to_string()), str_not_equal_str[1], "str_not_equal_str");
        assert_eq!(parser.parse_condition(str_not_equal2_str[0].to_string()), str_not_equal2_str[1], "str_not_equal2_str");
        assert_eq!(parser.parse_condition(int_not_equal_str[0].to_string()), int_not_equal_str[1], "int_not_equal_str");
        assert_eq!(parser.parse_condition(float_not_equal_str[0].to_string()), float_not_equal_str[1], "float_not_equal_str");
        assert_eq!(parser.parse_condition(bool_not_equal_str[0].to_string()), bool_not_equal_str[1], "bool_not_equal_str");

    }

    #[test]
    fn test_condition_superior_equal() {
        let var = GlobalVar::new();

        let parser = TemplateStr::new(var.var_map, None);

        // String
        let str_superior_equal_str = vec!["#{{'text' >= 'text'; yes | no}}", "yes"];
        let str_superior_equal2_str = vec!["#{{'text' >= 'texte'; yes | no}}", "no"];
        let str_superior_equal_int = vec!["#{{'text' >= i/4; yes | no}}", "yes"];
        let str_superior_equal2_int = vec!["#{{'text' >= i/123; yes | no}}", "no"];
        let str_superior_equal_float = vec!["#{{'text' >= f/4.5; yes | no}}", "no"];
        let str_superior_equal2_float = vec!["#{{'text' >= f/3.5; yes | no}}", "yes"];
        let str_superior_equal_bool = vec!["#{{'text' >= b/True; yes | no}}", "yes"];
        let str_superior_equal2_bool = vec!["#{{'text' >= b/False; yes | no}}", "yes"];

        assert_eq!(parser.parse_condition(str_superior_equal_str[0].to_string()), str_superior_equal_str[1], "str_superior_equal_str");
        assert_eq!(parser.parse_condition(str_superior_equal2_str[0].to_string()), str_superior_equal2_str[1], "str_superior_equal2_str");
        assert_eq!(parser.parse_condition(str_superior_equal_int[0].to_string()), str_superior_equal_int[1], "str_superior_equal_int");
        assert_eq!(parser.parse_condition(str_superior_equal2_int[0].to_string()), str_superior_equal2_int[1], "str_superior_equal2_int");
        assert_eq!(parser.parse_condition(str_superior_equal_float[0].to_string()), str_superior_equal_float[1], "str_superior_equal_float");
        assert_eq!(parser.parse_condition(str_superior_equal2_float[0].to_string()), str_superior_equal2_float[1], "str_superior_equal2_float");
        assert_eq!(parser.parse_condition(str_superior_equal_bool[0].to_string()), str_superior_equal_bool[1], "str_superior_equal_bool");
        assert_eq!(parser.parse_condition(str_superior_equal2_bool[0].to_string()), str_superior_equal2_bool[1], "str_superior_equal2_bool");

        // Int
        let int_superior_equal_str = vec!["#{{i/4 >= 'text'; yes | no}}", "yes"];
        let int_superior_equal2_str = vec!["#{{i/4 >= 'texte'; yes | no}}", "no"];
        let int_superior_equal_int = vec!["#{{i/4 >= i/4; yes | no}}", "yes"];
        let int_superior_equal2_int = vec!["#{{i/4 >= i/5; yes | no}}", "no"];
        let int_superior_equal_float = vec!["#{{i/4 >= f/3.5; yes | no}}", "yes"];
        let int_superior_equal2_float = vec!["#{{i/4 >= f/4.5; yes | no}}", "no"];
        let int_superior_equal_bool = vec!["#{{i/4 >= b/True; yes | no}}", "yes"];
        let int_superior_equal2_bool = vec!["#{{i/4 >= b/False; yes | no}}", "yes"];

        assert_eq!(parser.parse_condition(int_superior_equal_str[0].to_string()), int_superior_equal_str[1], "int_superior_equal_str");
        assert_eq!(parser.parse_condition(int_superior_equal2_str[0].to_string()), int_superior_equal2_str[1], "int_superior_equal2_str");
        assert_eq!(parser.parse_condition(int_superior_equal_int[0].to_string()), int_superior_equal_int[1], "int_superior_equal_int");
        assert_eq!(parser.parse_condition(int_superior_equal2_int[0].to_string()), int_superior_equal2_int[1], "int_superior_equal2_int");
        assert_eq!(parser.parse_condition(int_superior_equal_float[0].to_string()), int_superior_equal_float[1], "int_superior_equal_float");
        assert_eq!(parser.parse_condition(int_superior_equal2_float[0].to_string()), int_superior_equal2_float[1], "int_superior_equal2_float");
        assert_eq!(parser.parse_condition(int_superior_equal_bool[0].to_string()), int_superior_equal_bool[1], "int_superior_equal_bool");
        assert_eq!(parser.parse_condition(int_superior_equal2_bool[0].to_string()), int_superior_equal2_bool[1], "int_superior_equal2_bool");

        // Float
        let float_superior_equal_str = vec!["#{{f/4.5 >= 'text'; yes | no}}", "yes"];
        let float_superior_equal2_str = vec!["#{{f/4.5 >= 'texte'; yes | no}}", "no"];
        let float_superior_equal_int = vec!["#{{f/4.5 >= i/4; yes | no}}", "yes"];
        let float_superior_equal2_int = vec!["#{{f/4.5 >= i/5; yes | no}}", "no"];
        let float_superior_equal_float = vec!["#{{f/4.5 >= f/3.5; yes | no}}", "yes"];
        let float_superior_equal2_float = vec!["#{{f/4.5 >= f/4.6; yes | no}}", "no"];
        let float_superior_equal_bool = vec!["#{{f/4.5 >= b/True; yes | no}}", "yes"];
        let float_superior_equal2_bool = vec!["#{{f/4.5 >= b/False; yes | no}}", "yes"];

        assert_eq!(parser.parse_condition(float_superior_equal_str[0].to_string()), float_superior_equal_str[1], "float_superior_equal_str");
        assert_eq!(parser.parse_condition(float_superior_equal2_str[0].to_string()), float_superior_equal2_str[1], "float_superior_equal2_str");
        assert_eq!(parser.parse_condition(float_superior_equal_int[0].to_string()), float_superior_equal_int[1], "float_superior_equal_int");
        assert_eq!(parser.parse_condition(float_superior_equal2_int[0].to_string()), float_superior_equal2_int[1], "float_superior_equal2_int");
        assert_eq!(parser.parse_condition(float_superior_equal_float[0].to_string()), float_superior_equal_float[1], "float_superior_equal_float");
        assert_eq!(parser.parse_condition(float_superior_equal2_float[0].to_string()), float_superior_equal2_float[1], "float_superior_equal2_float");
        assert_eq!(parser.parse_condition(float_superior_equal_bool[0].to_string()), float_superior_equal_bool[1], "float_superior_equal_bool");
        assert_eq!(parser.parse_condition(float_superior_equal2_bool[0].to_string()), float_superior_equal2_bool[1], "float_superior_equal2_bool");

        // Bool
        let bool_superior_equal_str = vec!["#{{b/True >= 'text'; yes | no}}", "no"];
        let bool_superior_equal2_str = vec!["#{{b/False >= 'texte'; yes | no}}", "no"];
        let bool_superior_equal_int = vec!["#{{b/True >= i/4; yes | no}}", "no"];
        let bool_superior_equal2_int = vec!["#{{b/False >= i/5; yes | no}}", "no"];
        let bool_superior_equal_float = vec!["#{{b/True >= f/3.5; yes | no}}", "no"];
        let bool_superior_equal2_float = vec!["#{{b/False >= f/4.6; yes | no}}", "no"];
        let bool_superior_equal_bool = vec!["#{{b/True >= b/True; yes | no}}", "yes"];
        let bool_superior_equal2_bool = vec!["#{{b/False >= b/False; yes | no}}", "yes"];

        assert_eq!(parser.parse_condition(bool_superior_equal_str[0].to_string()), bool_superior_equal_str[1], "bool_superior_equal_str");
        assert_eq!(parser.parse_condition(bool_superior_equal2_str[0].to_string()), bool_superior_equal2_str[1], "bool_superior_equal2_str");
        assert_eq!(parser.parse_condition(bool_superior_equal_int[0].to_string()), bool_superior_equal_int[1], "bool_superior_equal_int");
        assert_eq!(parser.parse_condition(bool_superior_equal2_int[0].to_string()), bool_superior_equal2_int[1], "bool_superior_equal2_int");
        assert_eq!(parser.parse_condition(bool_superior_equal_float[0].to_string()), bool_superior_equal_float[1], "bool_superior_equal_float");
        assert_eq!(parser.parse_condition(bool_superior_equal2_float[0].to_string()), bool_superior_equal2_float[1], "bool_superior_equal2_float");
        assert_eq!(parser.parse_condition(bool_superior_equal_bool[0].to_string()), bool_superior_equal_bool[1], "bool_superior_equal_bool");
        assert_eq!(parser.parse_condition(bool_superior_equal2_bool[0].to_string()), bool_superior_equal2_bool[1], "bool_superior_equal2_bool");

    }


    #[test]
    fn test_condition_superior() {
        let var = GlobalVar::new();

        let parser = TemplateStr::new(var.var_map, None);

        // String
        let str_superior_str = vec!["#{{'text' > 'text'; yes | no}}", "no"];
        let str_superior2_str = vec!["#{{'text' > 'texte'; yes | no}}", "no"];
        let str_superior_int = vec!["#{{'text' > i/4; yes | no}}", "no"];
        let str_superior2_int = vec!["#{{'text' > i/123; yes | no}}", "no"];
        let str_superior_float = vec!["#{{'text' > f/4.5; yes | no}}", "no"];
        let str_superior2_float = vec!["#{{'text' > f/3.5; yes | no}}", "yes"];
        let str_superior_bool = vec!["#{{'text' > b/True; yes | no}}", "yes"];
        let str_superior2_bool = vec!["#{{'text' > b/False; yes | no}}", "yes"];

        assert_eq!(parser.parse_condition(str_superior_str[0].to_string()), str_superior_str[1], "str_superior_str");
        assert_eq!(parser.parse_condition(str_superior2_str[0].to_string()), str_superior2_str[1], "str_superior2_str");
        assert_eq!(parser.parse_condition(str_superior_int[0].to_string()), str_superior_int[1], "str_superior_int");
        assert_eq!(parser.parse_condition(str_superior2_int[0].to_string()), str_superior2_int[1], "str_superior2_int");
        assert_eq!(parser.parse_condition(str_superior_float[0].to_string()), str_superior_float[1], "str_superior_float");
        assert_eq!(parser.parse_condition(str_superior2_float[0].to_string()), str_superior2_float[1], "str_superior2_float");
        assert_eq!(parser.parse_condition(str_superior_bool[0].to_string()), str_superior_bool[1], "str_superior_bool");
        assert_eq!(parser.parse_condition(str_superior2_bool[0].to_string()), str_superior2_bool[1], "str_superior2_bool");

        // Int
        let int_superior_str = vec!["#{{i/4 > 'text'; yes | no}}", "no"];
        let int_superior2_str = vec!["#{{i/4 > 'texte'; yes | no}}", "no"];
        let int_superior_int = vec!["#{{i/4 > i/4; yes | no}}", "no"];
        let int_superior2_int = vec!["#{{i/4 > i/5; yes | no}}", "no"];
        let int_superior_float = vec!["#{{i/4 > f/3.5; yes | no}}", "yes"];
        let int_superior2_float = vec!["#{{i/4 > f/4.5; yes | no}}", "no"];
        let int_superior_bool = vec!["#{{i/4 > b/True; yes | no}}", "yes"];
        let int_superior2_bool = vec!["#{{i/4 > b/False; yes | no}}", "yes"];

        assert_eq!(parser.parse_condition(int_superior_str[0].to_string()), int_superior_str[1], "int_superior_str");
        assert_eq!(parser.parse_condition(int_superior2_str[0].to_string()), int_superior2_str[1], "int_superior2_str");
        assert_eq!(parser.parse_condition(int_superior_int[0].to_string()), int_superior_int[1], "int_superior_int");
        assert_eq!(parser.parse_condition(int_superior2_int[0].to_string()), int_superior2_int[1], "int_superior2_int");
        assert_eq!(parser.parse_condition(int_superior_float[0].to_string()), int_superior_float[1], "int_superior_float");
        assert_eq!(parser.parse_condition(int_superior2_float[0].to_string()), int_superior2_float[1], "int_superior2_float");
        assert_eq!(parser.parse_condition(int_superior_bool[0].to_string()), int_superior_bool[1], "int_superior_bool");
        assert_eq!(parser.parse_condition(int_superior2_bool[0].to_string()), int_superior2_bool[1], "int_superior2_bool");

        // Float
        let float_superior_str = vec!["#{{f/4.5 > 'text'; yes | no}}", "yes"];
        let float_superior2_str = vec!["#{{f/4.5 > 'texte'; yes | no}}", "no"];
        let float_superior_int = vec!["#{{f/4.5 > i/4; yes | no}}", "yes"];
        let float_superior2_int = vec!["#{{f/4.5 > i/5; yes | no}}", "no"];
        let float_superior_float = vec!["#{{f/4.5 > f/3.5; yes | no}}", "yes"];
        let float_superior2_float = vec!["#{{f/4.5 > f/4.6; yes | no}}", "no"];
        let float_superior_bool = vec!["#{{f/4.5 > b/True; yes | no}}", "yes"];
        let float_superior2_bool = vec!["#{{f/4.5 > b/False; yes | no}}", "yes"];

        assert_eq!(parser.parse_condition(float_superior_str[0].to_string()), float_superior_str[1], "float_superior_str");
        assert_eq!(parser.parse_condition(float_superior2_str[0].to_string()), float_superior2_str[1], "float_superior2_str");
        assert_eq!(parser.parse_condition(float_superior_int[0].to_string()), float_superior_int[1], "float_superior_int");
        assert_eq!(parser.parse_condition(float_superior2_int[0].to_string()), float_superior2_int[1], "float_superior2_int");
        assert_eq!(parser.parse_condition(float_superior_float[0].to_string()), float_superior_float[1], "float_superior_float");
        assert_eq!(parser.parse_condition(float_superior2_float[0].to_string()), float_superior2_float[1], "float_superior2_float");
        assert_eq!(parser.parse_condition(float_superior_bool[0].to_string()), float_superior_bool[1], "float_superior_bool");
        assert_eq!(parser.parse_condition(float_superior2_bool[0].to_string()), float_superior2_bool[1], "float_superior2_bool");

        // Bool
        let bool_superior_str = vec!["#{{b/True > 'text'; yes | no}}", "no"];
        let bool_superior2_str = vec!["#{{b/False > 'texte'; yes | no}}", "no"];
        let bool_superior_int = vec!["#{{b/True > i/4; yes | no}}", "no"];
        let bool_superior2_int = vec!["#{{b/False > i/5; yes | no}}", "no"];
        let bool_superior_float = vec!["#{{b/True > f/3.5; yes | no}}", "no"];
        let bool_superior2_float = vec!["#{{b/False > f/4.6; yes | no}}", "no"];
        let bool_superior_bool = vec!["#{{b/True > b/True; yes | no}}", "no"];
        let bool_superior2_bool = vec!["#{{b/False > b/False; yes | no}}", "no"];

        assert_eq!(parser.parse_condition(bool_superior_str[0].to_string()), bool_superior_str[1], "bool_superior_str");
        assert_eq!(parser.parse_condition(bool_superior2_str[0].to_string()), bool_superior2_str[1], "bool_superior2_str");
        assert_eq!(parser.parse_condition(bool_superior_int[0].to_string()), bool_superior_int[1], "bool_superior_int");
        assert_eq!(parser.parse_condition(bool_superior2_int[0].to_string()), bool_superior2_int[1], "bool_superior2_int");
        assert_eq!(parser.parse_condition(bool_superior_float[0].to_string()), bool_superior_float[1], "bool_superior_float");
        assert_eq!(parser.parse_condition(bool_superior2_float[0].to_string()), bool_superior2_float[1], "bool_superior2_float");
        assert_eq!(parser.parse_condition(bool_superior_bool[0].to_string()), bool_superior_bool[1], "bool_superior_bool");
        assert_eq!(parser.parse_condition(bool_superior2_bool[0].to_string()), bool_superior2_bool[1], "bool_superior2_bool");

    }

    #[test]
    fn test_condition_inferior_equal() {
        let var = GlobalVar::new();

        let parser = TemplateStr::new(var.var_map, None);

        // String
        let str_inferior_equal_str = vec!["#{{'text' <= 'text'; yes | no}}", "yes"];
        let str_inferior_equal2_str = vec!["#{{'text' <= 'texte'; yes | no}}", "yes"];
        let str_inferior_equal_int = vec!["#{{'text' <= i/4; yes | no}}", "yes"];
        let str_inferior_equal2_int = vec!["#{{'text' <= i/123; yes | no}}", "yes"];
        let str_inferior_equal_float = vec!["#{{'text' <= f/4.5; yes | no}}", "yes"];
        let str_inferior_equal2_float = vec!["#{{'text' <= f/3.5; yes | no}}", "no"];
        let str_inferior_equal_bool = vec!["#{{'text' <= b/True; yes | no}}", "no"];
        let str_inferior_equal2_bool = vec!["#{{'text' <= b/False; yes | no}}", "no"];

        assert_eq!(parser.parse_condition(str_inferior_equal_str[0].to_string()), str_inferior_equal_str[1], "str_inferior_equal_str");
        assert_eq!(parser.parse_condition(str_inferior_equal2_str[0].to_string()), str_inferior_equal2_str[1], "str_inferior_equal2_str");
        assert_eq!(parser.parse_condition(str_inferior_equal_int[0].to_string()), str_inferior_equal_int[1], "str_inferior_equal_int");
        assert_eq!(parser.parse_condition(str_inferior_equal2_int[0].to_string()), str_inferior_equal2_int[1], "str_inferior_equal2_int");
        assert_eq!(parser.parse_condition(str_inferior_equal_float[0].to_string()), str_inferior_equal_float[1], "str_inferior_equal_float");
        assert_eq!(parser.parse_condition(str_inferior_equal2_float[0].to_string()), str_inferior_equal2_float[1], "str_inferior_equal2_float");
        assert_eq!(parser.parse_condition(str_inferior_equal_bool[0].to_string()), str_inferior_equal_bool[1], "str_inferior_equal_bool");
        assert_eq!(parser.parse_condition(str_inferior_equal2_bool[0].to_string()), str_inferior_equal2_bool[1], "str_inferior_equal2_bool");

        // Int
        let int_inferior_equal_str = vec!["#{{i/4 <= 'text'; yes | no}}", "yes"];
        let int_inferior_equal2_str = vec!["#{{i/4 <= 'texte'; yes | no}}", "yes"];
        let int_inferior_equal_int = vec!["#{{i/4 <= i/4; yes | no}}", "yes"];
        let int_inferior_equal2_int = vec!["#{{i/4 <= i/5; yes | no}}", "yes"];
        let int_inferior_equal_float = vec!["#{{i/4 <= f/3.5; yes | no}}", "no"];
        let int_inferior_equal2_float = vec!["#{{i/4 <= f/4.5; yes | no}}", "yes"];
        let int_inferior_equal_bool = vec!["#{{i/4 <= b/True; yes | no}}", "no"];
        let int_inferior_equal2_bool = vec!["#{{i/4 <= b/False; yes | no}}", "no"];

        assert_eq!(parser.parse_condition(int_inferior_equal_str[0].to_string()), int_inferior_equal_str[1], "int_inferior_equal_str");
        assert_eq!(parser.parse_condition(int_inferior_equal2_str[0].to_string()), int_inferior_equal2_str[1], "int_inferior_equal2_str");
        assert_eq!(parser.parse_condition(int_inferior_equal_int[0].to_string()), int_inferior_equal_int[1], "int_inferior_equal_int");
        assert_eq!(parser.parse_condition(int_inferior_equal2_int[0].to_string()), int_inferior_equal2_int[1], "int_inferior_equal2_int");
        assert_eq!(parser.parse_condition(int_inferior_equal_float[0].to_string()), int_inferior_equal_float[1], "int_inferior_equal_float");
        assert_eq!(parser.parse_condition(int_inferior_equal2_float[0].to_string()), int_inferior_equal2_float[1], "int_inferior_equal2_float");
        assert_eq!(parser.parse_condition(int_inferior_equal_bool[0].to_string()), int_inferior_equal_bool[1], "int_inferior_equal_bool");
        assert_eq!(parser.parse_condition(int_inferior_equal2_bool[0].to_string()), int_inferior_equal2_bool[1], "int_inferior_equal2_bool");

        // Float
        let float_inferior_equal_str = vec!["#{{f/4.5 <= 'text'; yes | no}}", "no"];
        let float_inferior_equal2_str = vec!["#{{f/4.5 <= 'texte'; yes | no}}", "yes"];
        let float_inferior_equal_int = vec!["#{{f/4.5 <= i/4; yes | no}}", "no"];
        let float_inferior_equal2_int = vec!["#{{f/4.5 <= i/5; yes | no}}", "yes"];
        let float_inferior_equal_float = vec!["#{{f/4.5 <= f/3.5; yes | no}}", "no"];
        let float_inferior_equal2_float = vec!["#{{f/4.5 <= f/4.6; yes | no}}", "yes"];
        let float_inferior_equal_bool = vec!["#{{f/4.5 <= b/True; yes | no}}", "no"];
        let float_inferior_equal2_bool = vec!["#{{f/4.5 <= b/False; yes | no}}", "no"];

        assert_eq!(parser.parse_condition(float_inferior_equal_str[0].to_string()), float_inferior_equal_str[1], "float_inferior_equal_str");
        assert_eq!(parser.parse_condition(float_inferior_equal2_str[0].to_string()), float_inferior_equal2_str[1], "float_inferior_equal2_str");
        assert_eq!(parser.parse_condition(float_inferior_equal_int[0].to_string()), float_inferior_equal_int[1], "float_inferior_equal_int");
        assert_eq!(parser.parse_condition(float_inferior_equal2_int[0].to_string()), float_inferior_equal2_int[1], "float_inferior_equal2_int");
        assert_eq!(parser.parse_condition(float_inferior_equal_float[0].to_string()), float_inferior_equal_float[1], "float_inferior_equal_float");
        assert_eq!(parser.parse_condition(float_inferior_equal2_float[0].to_string()), float_inferior_equal2_float[1], "float_inferior_equal2_float");
        assert_eq!(parser.parse_condition(float_inferior_equal_bool[0].to_string()), float_inferior_equal_bool[1], "float_inferior_equal_bool");
        assert_eq!(parser.parse_condition(float_inferior_equal2_bool[0].to_string()), float_inferior_equal2_bool[1], "float_inferior_equal2_bool");

        // Bool
        let bool_inferior_equal_str = vec!["#{{b/True <= 'text'; yes | no}}", "yes"];
        let bool_inferior_equal2_str = vec!["#{{b/False <= 'texte'; yes | no}}", "yes"];
        let bool_inferior_equal_int = vec!["#{{b/True <= i/4; yes | no}}", "yes"];
        let bool_inferior_equal2_int = vec!["#{{b/False <= i/5; yes | no}}", "yes"];
        let bool_inferior_equal_float = vec!["#{{b/True <= f/3.5; yes | no}}", "yes"];
        let bool_inferior_equal2_float = vec!["#{{b/False <= f/4.6; yes | no}}", "yes"];
        let bool_inferior_equal_bool = vec!["#{{b/True <= b/True; yes | no}}", "yes"];
        let bool_inferior_equal2_bool = vec!["#{{b/False <= b/False; yes | no}}", "yes"];

        assert_eq!(parser.parse_condition(bool_inferior_equal_str[0].to_string()), bool_inferior_equal_str[1], "bool_inferior_equal_str");
        assert_eq!(parser.parse_condition(bool_inferior_equal2_str[0].to_string()), bool_inferior_equal2_str[1], "bool_inferior_equal2_str");
        assert_eq!(parser.parse_condition(bool_inferior_equal_int[0].to_string()), bool_inferior_equal_int[1], "bool_inferior_equal_int");
        assert_eq!(parser.parse_condition(bool_inferior_equal2_int[0].to_string()), bool_inferior_equal2_int[1], "bool_inferior_equal2_int");
        assert_eq!(parser.parse_condition(bool_inferior_equal_float[0].to_string()), bool_inferior_equal_float[1], "bool_inferior_equal_float");
        assert_eq!(parser.parse_condition(bool_inferior_equal2_float[0].to_string()), bool_inferior_equal2_float[1], "bool_inferior_equal2_float");
        assert_eq!(parser.parse_condition(bool_inferior_equal_bool[0].to_string()), bool_inferior_equal_bool[1], "bool_inferior_equal_bool");
        assert_eq!(parser.parse_condition(bool_inferior_equal2_bool[0].to_string()), bool_inferior_equal2_bool[1], "bool_inferior_equal2_bool");

    }

    #[test]
    fn test_condition_inferior() {
        let var = GlobalVar::new();

        let parser = TemplateStr::new(var.var_map, None);

        // String
        let str_inferior_str = vec!["#{{'text' < 'text'; yes | no}}", "no"];
        let str_inferior2_str = vec!["#{{'text' < 'texte'; yes | no}}", "yes"];
        let str_inferior_int = vec!["#{{'text' < i/4; yes | no}}", "no"];
        let str_inferior2_int = vec!["#{{'text' < i/123; yes | no}}", "yes"];
        let str_inferior_float = vec!["#{{'text' < f/4.5; yes | no}}", "yes"];
        let str_inferior2_float = vec!["#{{'text' < f/3.5; yes | no}}", "no"];
        let str_inferior_bool = vec!["#{{'text' < b/True; yes | no}}", "no"];
        let str_inferior2_bool = vec!["#{{'text' < b/False; yes | no}}", "no"];

        assert_eq!(parser.parse_condition(str_inferior_str[0].to_string()), str_inferior_str[1], "str_inferior_str");
        assert_eq!(parser.parse_condition(str_inferior2_str[0].to_string()), str_inferior2_str[1], "str_inferior2_str");
        assert_eq!(parser.parse_condition(str_inferior_int[0].to_string()), str_inferior_int[1], "str_inferior_int");
        assert_eq!(parser.parse_condition(str_inferior2_int[0].to_string()), str_inferior2_int[1], "str_inferior2_int");
        assert_eq!(parser.parse_condition(str_inferior_float[0].to_string()), str_inferior_float[1], "str_inferior_float");
        assert_eq!(parser.parse_condition(str_inferior2_float[0].to_string()), str_inferior2_float[1], "str_inferior2_float");
        assert_eq!(parser.parse_condition(str_inferior_bool[0].to_string()), str_inferior_bool[1], "str_inferior_bool");
        assert_eq!(parser.parse_condition(str_inferior2_bool[0].to_string()), str_inferior2_bool[1], "str_inferior2_bool");

        // Int
        let int_inferior_str = vec!["#{{i/4 < 'text'; yes | no}}", "no"];
        let int_inferior2_str = vec!["#{{i/4 < 'texte'; yes | no}}", "yes"];
        let int_inferior_int = vec!["#{{i/4 < i/4; yes | no}}", "no"];
        let int_inferior2_int = vec!["#{{i/4 < i/5; yes | no}}", "yes"];
        let int_inferior_float = vec!["#{{i/4 < f/3.5; yes | no}}", "no"];
        let int_inferior2_float = vec!["#{{i/4 < f/4.5; yes | no}}", "yes"];
        let int_inferior_bool = vec!["#{{i/4 < b/True; yes | no}}", "no"];
        let int_inferior2_bool = vec!["#{{i/4 < b/False; yes | no}}", "no"];

        assert_eq!(parser.parse_condition(int_inferior_str[0].to_string()), int_inferior_str[1], "int_inferior_str");
        assert_eq!(parser.parse_condition(int_inferior2_str[0].to_string()), int_inferior2_str[1], "int_inferior2_str");
        assert_eq!(parser.parse_condition(int_inferior_int[0].to_string()), int_inferior_int[1], "int_inferior_int");
        assert_eq!(parser.parse_condition(int_inferior2_int[0].to_string()), int_inferior2_int[1], "int_inferior2_int");
        assert_eq!(parser.parse_condition(int_inferior_float[0].to_string()), int_inferior_float[1], "int_inferior_float");
        assert_eq!(parser.parse_condition(int_inferior2_float[0].to_string()), int_inferior2_float[1], "int_inferior2_float");
        assert_eq!(parser.parse_condition(int_inferior_bool[0].to_string()), int_inferior_bool[1], "int_inferior_bool");
        assert_eq!(parser.parse_condition(int_inferior2_bool[0].to_string()), int_inferior2_bool[1], "int_inferior2_bool");

        // Float
        let float_inferior_str = vec!["#{{f/4.5 < 'text'; yes | no}}", "no"];
        let float_inferior2_str = vec!["#{{f/4.5 < 'texte'; yes | no}}", "yes"];
        let float_inferior_int = vec!["#{{f/4.5 < i/4; yes | no}}", "no"];
        let float_inferior2_int = vec!["#{{f/4.5 < i/5; yes | no}}", "yes"];
        let float_inferior_float = vec!["#{{f/4.5 < f/3.5; yes | no}}", "no"];
        let float_inferior2_float = vec!["#{{f/4.5 < f/4.6; yes | no}}", "yes"];
        let float_inferior_bool = vec!["#{{f/4.5 < b/True; yes | no}}", "no"];
        let float_inferior2_bool = vec!["#{{f/4.5 < b/False; yes | no}}", "no"];

        assert_eq!(parser.parse_condition(float_inferior_str[0].to_string()), float_inferior_str[1], "float_inferior_str");
        assert_eq!(parser.parse_condition(float_inferior2_str[0].to_string()), float_inferior2_str[1], "float_inferior2_str");
        assert_eq!(parser.parse_condition(float_inferior_int[0].to_string()), float_inferior_int[1], "float_inferior_int");
        assert_eq!(parser.parse_condition(float_inferior2_int[0].to_string()), float_inferior2_int[1], "float_inferior2_int");
        assert_eq!(parser.parse_condition(float_inferior_float[0].to_string()), float_inferior_float[1], "float_inferior_float");
        assert_eq!(parser.parse_condition(float_inferior2_float[0].to_string()), float_inferior2_float[1], "float_inferior2_float");
        assert_eq!(parser.parse_condition(float_inferior_bool[0].to_string()), float_inferior_bool[1], "float_inferior_bool");
        assert_eq!(parser.parse_condition(float_inferior2_bool[0].to_string()), float_inferior2_bool[1], "float_inferior2_bool");

        // Bool
        let bool_inferior_str = vec!["#{{b/True < 'text'; yes | no}}", "yes"];
        let bool_inferior2_str = vec!["#{{b/False < 'texte'; yes | no}}", "yes"];
        let bool_inferior_int = vec!["#{{b/True < i/4; yes | no}}", "yes"];
        let bool_inferior2_int = vec!["#{{b/False < i/5; yes | no}}", "yes"];
        let bool_inferior_float = vec!["#{{b/True < f/3.5; yes | no}}", "yes"];
        let bool_inferior2_float = vec!["#{{b/False < f/4.6; yes | no}}", "yes"];
        let bool_inferior_bool = vec!["#{{b/True < b/True; yes | no}}", "no"];
        let bool_inferior2_bool = vec!["#{{b/False < b/False; yes | no}}", "no"];

        assert_eq!(parser.parse_condition(bool_inferior_str[0].to_string()), bool_inferior_str[1], "bool_inferior_str");
        assert_eq!(parser.parse_condition(bool_inferior2_str[0].to_string()), bool_inferior2_str[1], "bool_inferior2_str");
        assert_eq!(parser.parse_condition(bool_inferior_int[0].to_string()), bool_inferior_int[1], "bool_inferior_int");
        assert_eq!(parser.parse_condition(bool_inferior2_int[0].to_string()), bool_inferior2_int[1], "bool_inferior2_int");
        assert_eq!(parser.parse_condition(bool_inferior_float[0].to_string()), bool_inferior_float[1], "bool_inferior_float");
        assert_eq!(parser.parse_condition(bool_inferior2_float[0].to_string()), bool_inferior2_float[1], "bool_inferior2_float");
        assert_eq!(parser.parse_condition(bool_inferior_bool[0].to_string()), bool_inferior_bool[1], "bool_inferior_bool");
        assert_eq!(parser.parse_condition(bool_inferior2_bool[0].to_string()), bool_inferior2_bool[1], "bool_inferior2_bool");

    }
}

#[cfg(test)]
mod switch {
    use super::*;

    #[test]
    fn test_switch() {
        let var = GlobalVar::new();

        let text_switch_1 = vec!["?{{str; Jame:#0, Tony::#1, Marco:#2, _:#default}}", "#0"];
        let text_switch_2 = vec!["?{{int/int; 56:#0, 36:#1, 32:#2, _:#default}}", "#2"];
        let text_switch_3 = vec!["?{{int/int; 56:#0, 36:#1, 32:?{{str/str; Jame:#42, 36:#1, 32:#2, _:#default}}, _:#default}}", "#42"];

        let parser = TemplateStr::new(var.var_map, None);
        
        assert_eq!(parser.parse_switch(text_switch_1[0].to_string()), text_switch_1[1], "text_switch_1");
        assert_eq!(parser.parse_switch(text_switch_2[0].to_string()), text_switch_2[1], "text_switch_2");
        assert_eq!(parser.parse_switch(text_switch_3[0].to_string()), text_switch_3[1], "text_switch_3");
    }
}

