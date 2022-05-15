use template_str::{varmap, list_func, TemplateStr, t_type::{TVal, FuncMap, VariableMap as VMap}};

fn test(_: Vec<TVal>) -> String {
    return "Test custom Function".to_string();
}

fn test_type(list: Vec<TVal>) -> String {

    let mut text: String = "start".to_string();

    println!("{:?}", list);

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

fn main() {

    let vec: FuncMap = list_func![test, test_type];

    let map: VMap = varmap!{
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
    };

    let template = TemplateStr::new(Some(map), Some(vec));

    let text = template.parse("?{{str; Jame=#0, Tony:=#1, Marco:=#2, default=#default}}".to_string());

    println!("{}", text);
    
}
