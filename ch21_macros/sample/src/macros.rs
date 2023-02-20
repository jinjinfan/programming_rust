pub use std::collections::HashMap;
pub use std::boxed::Box;
pub use std::string::ToString;

#[macro_export]
macro_rules! json {
    (null) => {
        $crate::Json::Null
    };
    ([$($element:tt),*]) => {
        $crate::Json::Array(vec![$(json!($element)),*])
    };
    ({$($key:tt : $value:tt),*}) => {
        $crate::Json::Object($crate::macros::Box::new(vec![
            $(($key.to_string(), json!($value))),*
        ].into_iter().collect()))
    };
    ($other : tt) => {
        $crate::Json::from($other)
    }
}

macro_rules! complain {
    (msg : $msg:expr) => {
        println!("Complaint filed: {}", $msg);
    };
    (user : $useid : tt, msg : $msg:expr) => {
        println!("Complaint from user {} : {}", $useid, $msg);
    };
}