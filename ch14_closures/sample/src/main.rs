#[derive(Debug)]
struct City {

}

fn count_selected_cities<F>(cities : &Vec<City>, test_fn : F) -> usize
    where F : Fn(&City) -> bool // Fn trait
{
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1
        }
    }
    count
}

fn call_twice<F>(mut closure : F) where F : FnMut() {
    closure();
    closure();
}

fn fnmut_test() {
    let mut i = 0;
    call_twice(|| i+=1);
    println!("i: {}", i);
}

fn move_clone() {
    let mut greeting = String::from("Hello, ");
    let greet = move |name| {
        greeting.push_str(name);
        println!("{}", greeting);
    };
    greet.clone()("Alfred");
    greet.clone()("Bruce");
}

use std::collections::HashMap;
// callbacks
struct Request {
    method : String,
    url : String,
    headers : HashMap<String, String>,
    body : Vec<u8>
}

struct Response {
    code : u32,
    headers : HashMap<String, String>,
    body : Vec<u8>
}

type BoxedCallback = Box<dyn Fn(&Request)-> Response>;

struct BasicRouter{
    routes: HashMap<String, BoxedCallback>
}

impl BasicRouter {
    fn new() -> BasicRouter{
        BasicRouter{routes: HashMap::new()}
    }
    fn add_route<C>(&mut self, url : &str, callback : C) 
        where C: Fn(&Request) -> Response + 'static
    {
        self.routes.insert(url.to_string(), Box::new(callback));
    }
    fn handle_request(&self, request : &Request) -> Response {
        match self.routes.get(&request.url) {
            None => not_found_response(),
            Some(callback) => callback(request)
        }
    }
}

fn not_found_response() -> Response{
    let mut headers = HashMap::new();
    headers.insert("test".to_string(), "test".to_string());
    let a : Response = Response {
        code : 1,
        headers : headers,
        body : Vec::new(),
    };
    a
}

//Function pointer
struct FnPointerRouter {
    routes : HashMap<String, fn(&Request) -> Response>
}

impl FnPointerRouter {
    fn new() -> FnPointerRouter {
        FnPointerRouter {
            routes : HashMap::new()
        }
    }
    fn add_route(&mut self, url : &str, callback :  fn(&Request) -> Response)
    {
        self.routes.insert(url.to_string(), callback);
    }
}


fn add_ten(x: u32) -> u32 {
    x+10
}

fn main() {
    fnmut_test();
    move_clone();
    //Function pointer
    let fn_ptr : fn(u32)-> u32 = add_ten;
    println!("{}", fn_ptr(1));
    //Closure pointer
    let closure_ptr : fn(u32) -> u32 = |x|x+1;
    println!("{}", closure_ptr(1));

}
