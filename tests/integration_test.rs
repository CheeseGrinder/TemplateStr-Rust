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
            _ => { text = format!("{} : {}", text, "None") },
        };
    }
    return text;
}

impl GlobalVar {
    fn new() -> GlobalVar {
        return GlobalVar {
            list_func: list_func![test, test_type],
            var_map: varmap!{
                "str" => "Jame",
                "age" => 32,
                "float" => 4.2,
                "bool" => true,
                "lower" => "azerty",
                "upper" => "AZERTY",
                "swap" => "AzErTy",
                "cfold" => "grüßen",
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
mod variable {
    use super::*;

    #[test]
    fn test_variable() {

        let var = GlobalVar::new();

        let text_1 = vec!["var bool = {{$bool}} and name = {{$str}}", "var bool = true and name = Jame"];
        let text_2 = vec!["{{$Map.value}}", "Map in Map"];
        let text_3 = vec!["{{$MasterMap.SecondMap.value}}", "Map in Map in Map"];
        let text_4 = vec!["{{$word}}", "None"];
        let text_5 = vec!["{{$Map.SecondMap.value}}", "None"];

        let mut parser = TemplateStr::new(Some(var.var_map), None);
        
        assert_eq!(parser.parse_variable(text_1[0].to_string()), text_1[1]);
        assert_eq!(parser.parse_variable(text_2[0].to_string()), text_2[1]);
        assert_eq!(parser.parse_variable(text_3[0].to_string()), text_3[1]);
        assert_eq!(parser.parse_variable(text_4[0].to_string()), text_4[1]);
        assert_eq!(parser.parse_variable(text_5[0].to_string()), text_5[1]);
    }
}

#[cfg(test)]
mod function {

    use super::*;

    #[test]
    fn test_interne_function() {
        let var = GlobalVar::new();

        let uppercase = vec!["{{@uppercase lower}}", "AZERTY"];
        let uppercase2 = vec!["{{@uppercase word}}", "NONE"];
        let uppercase_first = vec!["{{@uppercaseFirst lower}}", "Azerty"];
        let lowercase = vec!["{{@lowercase upper}}", "azerty"];
        // let casefold = vec!["{{@casefold cfold}}", "grüssen"];
        let swapcase = vec!["{{@swapcase swap}}", "aZeRtY"];
        let time = "{{@time}}";
        let date = "{{@date}}";
        let date_time = "{{@dateTime}}";

        let parser = TemplateStr::new(Some(var.var_map), None);

        assert_eq!(parser.parse_function(uppercase[0].to_string()), uppercase[1]);
        assert_eq!(parser.parse_function(uppercase2[0].to_string()), uppercase2[1]);
        assert_eq!(parser.parse_function(uppercase_first[0].to_string()), uppercase_first[1]);
        assert_eq!(parser.parse_function(lowercase[0].to_string()), lowercase[1]);
        // assert_eq!(parser.parse_function(casefold[0].to_string()), casefold[1]);
        assert_eq!(parser.parse_function(swapcase[0].to_string()), swapcase[1]);
        assert_eq!(parser.parse_function(time.to_string()), Utc::now().format("%H:%M:%S").to_string().as_str());
        assert_eq!(parser.parse_function(date.to_string()), Utc::now().format("%d/%m/%Y").to_string().as_str());
        assert_eq!(parser.parse_function(date_time.to_string()), Utc::now().format("%d/%m/%Y %H:%M:%S").to_string().as_str());

    }

    #[test]
    fn test_custom_function() {
        let var = GlobalVar::new();

        let test_1 = vec!["{{@test}}", "Test custom Function"];
        let test_2 = vec![r#"{{@test_type "text1" 'text2' `text3` <b:True> <n:123> <n:123.4> age}}"#, "start : test Str : test Str : test Str : test Bool : test Int : test Float : test Str"];

        let parser = TemplateStr::new(Some(var.var_map), Some(var.list_func));

        assert_eq!(parser.parse_function(test_1[0].to_string()), test_1[1]);
        assert_eq!(parser.parse_function(test_2[0].to_string()), test_2[1]);

    }
}