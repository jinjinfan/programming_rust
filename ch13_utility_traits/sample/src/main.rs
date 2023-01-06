struct RcBox<T: ?Sized> {
    ref_count: usize,
    value : T
}
fn display(boxed : &RcBox<dyn std::fmt::Display>) {
    println!("For your enjoyment: {}", &boxed.value);
}

fn sized_trait() {
    let boxed_lunch : RcBox<String> = RcBox {
        ref_count : 1,
        value : "lunch".to_string()
    };
    display(&boxed_lunch);
}

struct Selector<T> {
    elements : Vec<T>,
    current  : usize,
}

use std::ops::{Deref, DerefMut};

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}

fn show_it(thing:&str) {
    println!("{}", thing);
}

fn show_it_generic<T: std::fmt::Display>(thing:T) {
    println!("{}", thing);
}
fn deref_trait() {
    let mut s = Selector{
        elements: vec!['x','y','z'],
        current : 2
    };
    assert_eq!(*s, 'z');
    assert!((&s).is_alphabetic());
    *s = 'w';
    assert_eq!(s.elements, ['x','y','w']);
    let s = Selector{
        elements: vec!["good","bad","guy"],
        current : 2
    };
    show_it(&s);
    show_it_generic(&s as &str);
    show_it_generic(&*s);

}

use std::net::Ipv4Addr;
fn ping<A>(address : A)
    where A : Into<Ipv4Addr> {
    let ipv4_address = address.into();    
    println!("{:?}", ipv4_address);    
}

fn from_into() {
    ping(Ipv4Addr::new(23, 21, 68, 141));
    ping([66,164,219,98]);
    ping(0xd076eb94_u32);
}

fn try_from_into() {
    let huge : i64 = 26467474744;
    let smaller : i32 = huge.try_into().unwrap_or_else(|_|{
        if huge >= 0 {
            i32::MAX
        } else {
            i32::MIN
        }
    });
    println!("{}", smaller);
}

// Clone on write
use std::path::PathBuf;
use std::borrow::Cow;
enum Error {
    OutOfMemory,
    StackOverFlow,
    MachineOnFire,
    Unfathomable,
    FileNotFound(PathBuf),
}
fn describe(error: &Error) -> Cow<'static, str> {
    match *error {
        Error::OutOfMemory => "out of memory".into(),
        Error::StackOverFlow => "stack overflow".into(),
        Error::MachineOnFire => "machine on fire".into(),
        Error::Unfathomable => "machine bewildered".into(),
        Error::FileNotFound(ref path) => {
            format!("file not found: {}", path.display()).into()
        }
    }
}

fn clone_on_write() {
    let error = Error::FileNotFound(PathBuf::from("dhdhdh"));
    println!("{}", describe(&error));
    println!("{}", describe(&error).into_owned());
}

fn main() {
    sized_trait();
    deref_trait();
    from_into();
    try_from_into();
    clone_on_write();
}
