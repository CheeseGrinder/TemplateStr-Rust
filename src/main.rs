use template_str::{varmap, list_func, TemplateStr, t_type::{TVal, FuncMap, VariableMap as VMap}, vecTval};

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
            TVal::Vec(_) => todo!(),
            // _ => { text = format!("{} : {}", text, "None") },
        };
    }
    return text;
}

fn main() {

    let vec: FuncMap = list_func![test, test_type];

    let map: VMap = varmap!{
        "Build" => "Succes",
        "var" => "int",
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
        "test" => "",
        "MasterMap" => varmap!{
            "Vec" => vec![
                    TVal::Bool(true), 
                    TVal::Str("test".to_string())
                ],
            "SecondMap" => varmap!{
                "value" => "Map in Map in Map",
            },
        },
        "Vec" => vecTval![
            true, 
            "test"
        ],
    };

    let template = TemplateStr::new(map, Some(vec));

    let text = template.parse_variable("${Vec[2]}".to_string());


    println!("{:#?}", text.unwrap_err().to_string())

     

}
