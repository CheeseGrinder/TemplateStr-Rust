pub const REG_STR : &str = r#""(?P<str_double>[^"]+)"|'(?P<str_single>[^']+)'|`(?P<str_back>[^`]+)`"#;
pub const REG_BOOL : &str = r#"b/(?P<bool>[Tt]rue|[Ff]alse)"#;
pub const REG_INT : &str = r#"i/(?P<int>[0-9_]+)"#;
pub const REG_FLOAT : &str = r#"f/(?P<float>[0-9_.]+)"#;
pub const REG_VAR : &str = r#"(?P<variable>[\w._-]+)(?:\[(?P<index>[\d]+)])?"#;
pub const REG_LIST : &str = r#"\((?P<list>[^\(\)]+)\)"#;

pub const REG_VARIABLE : &str = r#"(?P<match>\$\{(?P<variable>[\w._-]+)(?:\[(?P<index>[\d]+)])?})"#;
pub const REG_FUNCTION : &str = r#"(?P<match>@\{(?P<functionName>[^{}\s]+)(?:; (?P<parameters>[^{}]+))?})"#;
pub const REG_CONDITION : &str = r#"(?P<match>#\{(?P<conditionValue1>[^{#}]+) (?P<conditionSymbol>==|!=|<=|<|>=|>) (?P<conditionValue2>[^{#}]+); (?P<trueValue>[^{}]+) \| (?P<falseValue>[^{}]+)})"#;
pub const REG_SWITCH : &str = r#"(?P<match>\?\{(?:(?P<type>str|int|float)/)?(?P<variable>[\w._-]+)(?:\[(?P<index>[\d]+)])?; (?P<values>(?:[^{}]+::[^{}]+){2,}), _::(?P<defaultValue>[^{}]+)})"#;

