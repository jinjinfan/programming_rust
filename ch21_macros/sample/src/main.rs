#![feature(trace_macros)]
#![recursion_limit = "256"]
#[macro_use] mod macros;


macro_rules! vec2 {
    ($elem : expr; $n : expr) => {
        ::std::vec::from_elem($elem, $n)
    };
    ($($x:expr),*) => {
        <[_]>::into_vec(Box::new([$($x),*]))
    };
    ($($x : expr),+,) => {
        vec2![$($x),*]
    }
}

use std::collections::HashMap;
#[derive(Clone, PartialEq, Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>)
}

impl From<bool> for Json {
    fn from(value: bool) -> Self {
        Json::Boolean(value)
    }
}

impl From<String> for Json {
    fn from(value: String) -> Self {
        Json::String(value)
    }
}

impl<'a> From<&'a str> for Json {
    fn from(value: &'a str) -> Self {
        Json::String(value.to_string())
    }
}

macro_rules! impl_from_num_for_json {
    ($($t :ident)*) => {
       $(
        impl From<$t> for Json {
            fn from(n : $t) -> Json {
                Json::Number(n as f64)
            }
        }
        )*
    };
}
impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize f32 f64);


#[test]
fn json_null() {
    assert_eq!(crate::json!(null), Json::Null);
}

#[test]
fn json_array_with_json_element() {
    let macro_generated_value = crate::json!(
        [
            {"pitch" : 440.0}
        ]
    );
    let hand_coded_value = Json::Array( vec! [
        Json::Object(Box::new(vec![
            ("pitch".to_string(), Json::Number(440.0))
        ].into_iter().collect()))
    ]);
    assert_eq!(macro_generated_value, hand_coded_value);
}

fn main() {
    /*trace_macros!(true);
    let numbers = vec![1,2,3];
    trace_macros!(false);
    println!("total: {}", numbers.iter().sum::<u64>());
    */
    let students = crate::json!([
        {
            "name" : "Jim",
            "class_of" : 1926,
            "major" : "singing"
        },
        {
            "name" : "Jason",
            "class_of" : 1702,
            "major" : "Knots"
        }
    ]);
    let width = 4.0;
    let desc = crate::json!({
        "width" : width,
        "height" : (width * 9.0/4.0)
    });
}
