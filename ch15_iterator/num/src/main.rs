use std::error::Error;
use std::io::prelude::*;
use std::str::FromStr;
use std::env;


/*fn all<P>(&mut self, mut predicate:P) -> bool 
    where P : FnMut(Self::Item) -> bool,
          Self : Sized
{
    use std::ops::ControlFlow::*;
    self.try_fold((), |_, item| {
        if predicate(item) {Continue(())} else {Break(())}
    }) == Continue(())  
}
*/

struct I32Range {
    start : i32,
    end   : i32,
}

impl Iterator for I32Range {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.start >= self.end {
            return None;
        }
        let result = Some(self.start);
        self.start += 1;
        result
    }
}

// An ordered collection of `T`s
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element : T,
    left : BinaryTree<T>,
    right : BinaryTree<T>,
}
// In-order traversal of a BinaryTree
struct TreeIter<'a, T> {
    unvisited : Vec<&'a TreeNode<T>>
}

use self::BinaryTree::*;

impl<'a, T:'a> TreeIter<'a, T> {
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}

impl<'a, T> Iterator for TreeIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let node = self.unvisited.pop()?;
        self.push_left_edge(&node.right);
        Some(&node.element)
    }
}
impl<T> BinaryTree<T> {
    fn iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter{unvisited:Vec::new()};
        iter.push_left_edge(self);
        iter
    }
}

impl<T:Ord> BinaryTree<T> {
    fn add(&mut self, value : T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new({TreeNode {
                    element : value,
                    left : BinaryTree::Empty,
                    right : BinaryTree::Empty,
                }}))
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value)
                } else {
                    node.right.add(value)
                }
            }
        }
    }
}

impl<'a, T:'a> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }

}

fn main() -> Result<(), Box<dyn Error>>{
    let stdin = std::io::stdin();
    let sum = env::args().skip(1)
        .try_fold(0, |sum, arg|-> Result<u64, Box<dyn Error>>{
            Ok(sum + u64::from_str(&arg)?)
        })?;
    println!("{}", sum);

    let mut pi = 0.0;
    let mut numerator = 1.0;
    for k in (I32Range{start:0, end: 14}) {
        pi += numerator / (2 * k + 1) as f64;
        numerator /= -3.0;
    }
    pi *= f64::sqrt(12.0);
    assert_eq!(pi as f32, std::f32::consts::PI);

    let mut tree = BinaryTree::Empty;
    tree.add("jaeger");
    tree.add("robot");
    tree.add("droid");
    tree.add("mecha");

    let mut v = Vec::new();
    for kind in &tree {
        v.push(*kind);
    }
    println!("{:?}", v);
    assert_eq!(tree.iter()
                .map(|name| format!("mega-{}", name))
                .collect::<Vec<_>>(),
                vec!["mega-droid","mega-jaeger", "mega-mecha","mega-robot"]);
    Ok(())
}
