


#[macro_export]
macro_rules! varmap {

    ($($key: expr => $val: expr $(,) ?)*) => {{

            use template_str::t_type::{VariableMap, TVal};
            use std::collections::HashMap;
            use std::any::{Any, TypeId};

            fn cast_type<T: Any>(value: &T) -> Option<TVal> {
                let value = value as &dyn Any;
                match value.type_id() {
                    | x if x == TypeId::of::<&str>() => {
                        let cast_value = value.downcast_ref::<&str>();
                        return cast_value.map(|x| TVal::Str(x.to_string()))
                    },
                    | x if x == TypeId::of::<i32>() => {
                        let cast_value  = value.downcast_ref::<i32>();
                        return cast_value.map(|x| TVal::Int(*x))
                    },
                    | x if x == TypeId::of::<f64>() => {
                        let cast_value  = value.downcast_ref::<f64>();
                        return cast_value.map(|x| TVal::Float(*x))
                    },
                    | x if x == TypeId::of::<bool>() => {
                        let cast_value  = value.downcast_ref::<bool>();
                        return cast_value.map(|x| TVal::Bool(*x))
                    },
                    | x if x == TypeId::of::<VariableMap>() => {
                        let cast_value  = value.downcast_ref::<VariableMap>();
                        return cast_value.map(|x| TVal::Hashmap(x.clone()))
                    },
                    _ => {
                        println!("{:?}", value.type_id());
                        panic!("lol"); 
                    },
                }
            };
            
            let mut map = VariableMap::new();
            $(  

                let key = $key.to_string();
                if let Some(value) = cast_type(&$val) {
                    map.insert(key, value);
                }
            )*
            map
        }};
}

#[macro_export]
macro_rules! list_func {
    ($($func: expr $(,) ?)*) => {{

        use template_str::t_type::{FuncMap, Func};
        let mut map = FuncMap::new();
        $(
            map.insert(stringify!($func).to_string(), $func as Func);
        )*
        map
    }};
}

#[macro_export]
macro_rules! attempt { // `try` is a reserved keyword
    (@recurse ($a:expr) { } catch ($e:ident) $b:block) => {
       if let Err ($e) = $a $b
    };
    (@recurse ($a:expr) { $e:expr; $($tail:tt)* } $($handler:tt)*) => {
       attempt!{@recurse ($a.and_then (|_| $e)) { $($tail)* } $($handler)*}
    };
    ({ $e:expr; $($tail:tt)* } $($handler:tt)*) => {
       attempt!{@recurse ($e) { $($tail)* } $($handler)* }
    };
 }