use std::io::Write;

// trait object
fn say_hello(out : &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

// Generic function and type parameters
fn say_hello2<W : Write>(out : &mut W) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

struct HtmlDocument {

}
trait WriteHtml {
    fn write_html(&mut self, html: &HtmlDocument) -> std::io::Result<()> ;
}

impl<W : Write> WriteHtml for W {
    fn write_html(&mut self, html: &HtmlDocument) -> std::io::Result<()> {
        Ok(())
    }

}

//subtraits
trait Visible {

}

trait Creature : Visible {

}

trait Creature1 where Self : Visible {

}

// Type associated functions
trait StringSet {
    fn new() -> Self
        where Self: Sized; // so that &dyn StringSet trait object is used
    fn from_slice(strings : &[&str]) -> Self
        where Self:Sized;
    fn contains(&self, string : &str) -> bool;
    fn add(&mut self, string : &str);
}

fn unknown_words<S: StringSet>(documents: &[String], wordlist : &S) -> S {
    let mut unknowns = S::new();
    for word in documents {
        if !wordlist.contains(word) {
            unknowns.add(word);
        }
    }
    unknowns
}

fn collect_into_vector<I: Iterator>(iter:I) -> Vec<I::Item> {
    let mut results = Vec::new();
    for value in iter {
        results.push(value);
    }
    results
}

fn dump(iter : &mut dyn Iterator<Item=String>) {
    for (index, s) in iter.enumerate() {
        println!("{}: {:?}", index, s);
    }
}
//impl Trait
use std::iter;
use std::vec::IntoIter;
fn cyclical_zip(v:Vec<u8>, u : Vec<u8>) -> 
    iter::Cycle<iter::Chain<IntoIter<u8>, IntoIter<u8>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn cyclical_zip2(v:Vec<u8>, u : Vec<u8>) -> impl Iterator<Item=u8> {
    v.into_iter().chain(u.into_iter()).cycle()
}

trait Float {
    const ZERO :Self;
    const ONE :Self;
}

fn fib<T: Float + std::ops::Add<Output=T>>(n : usize) -> T {
    match n {
        0 => T::ZERO,
        1 => T::ONE,
        n => fib::<T>(n-1) + fib::<T>(n-2)
    }
}

use std::ops::{Add, Mul};

fn dot<N>(v1:&[N], v2:&[N]) -> N
   where N : Add<Output=N> + Mul<Output=N> + Default + Copy {
    let mut total = N::default();
    for i in 0 .. v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

#[test]
fn test_dot() {
    assert_eq!(dot(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10);
    assert_eq!(dot(&[53.0, 7.0], &[1.0, 5.0]), 88.0);
}
fn main() {
    //trait object
    let mut buf : Vec<u8> = vec![];
    let writer : &mut dyn Write = &mut buf;
    //fully qualified method call
    <str as ToString>::to_string("hello");
}
